//! User input mechanisms: numbers, colors, and text in various forms.

use super::Control;
use libui_sys::{
    self, uiCheckbox, uiCombobox, uiControl, uiEntry, uiMultilineEntry, uiRadioButtons, uiSlider,
    uiSpinbox,
};
use std::ffi::{CStr, CString};
use std::i32;
use std::mem;
use std::os::raw::c_void;
use ui::UI;

pub trait NumericEntry {
    fn value(&self, ctx: &UI) -> i32;
    fn set_value(&mut self, ctx: &UI, value: i32);
    fn on_changed<'ctx, F: FnMut(i32) + 'ctx>(&mut self, ctx: &'ctx UI, callback: F);
}

pub trait TextEntry {
    fn value(&self, ctx: &UI) -> String;
    fn set_value(&mut self, ctx: &UI, value: &str);
    fn on_changed<'ctx, F: FnMut(String) + 'ctx>(&mut self, ctx: &'ctx UI, callback: F);
}

define_control! {
    /// Numerical entry control which allows users to set any value in a range by typing or incrementing/decrementing.
    rust_type: Spinbox,
    sys_type: uiSpinbox
}

define_control! {
    /// Numerical entry which allows users to select a value by picking a location along a line.
    rust_type: Slider,
    sys_type: uiSlider
}

impl Spinbox {
    // Create a new Spinbox which can produce values from `min` to `max`.
    pub fn new(_ctx: &UI, min: i32, max: i32) -> Self {
        unsafe { Spinbox::from_raw(libui_sys::uiNewSpinbox(min, max)) }
    }

    // Create a new Spinbox with the maximum possible range.
    pub fn new_unlimited(_ctx: &UI) -> Self {
        Self::new(_ctx, i32::MIN, i32::MAX)
    }
}

impl Slider {
    // Create a new Spinbox which can produce values from `min` to `max`.
    pub fn new(_ctx: &UI, min: i32, max: i32) -> Self {
        unsafe { Slider::from_raw(libui_sys::uiNewSlider(min, max)) }
    }
}

impl NumericEntry for Spinbox {
    fn value(&self, _ctx: &UI) -> i32 {
        unsafe { libui_sys::uiSpinboxValue(self.uiSpinbox) }
    }

    fn set_value(&mut self, _ctx: &UI, value: i32) {
        unsafe { libui_sys::uiSpinboxSetValue(self.uiSpinbox, value) }
    }

    fn on_changed<'ctx, F: FnMut(i32) + 'ctx>(&mut self, _ctx: &'ctx UI, callback: F) {
        unsafe {
            let mut data: Box<Box<dyn FnMut(i32)>> = Box::new(Box::new(callback));
            libui_sys::uiSpinboxOnChanged(
                self.uiSpinbox,
                Some(c_callback),
                &mut *data as *mut Box<dyn FnMut(i32)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(spinbox: *mut uiSpinbox, data: *mut c_void) {
            unsafe {
                let val = libui_sys::uiSpinboxValue(spinbox);
                mem::transmute::<*mut c_void, &mut Box<dyn FnMut(i32)>>(data)(val);
            }
        }
    }
}

impl NumericEntry for Slider {
    fn value(&self, _ctx: &UI) -> i32 {
        unsafe { libui_sys::uiSliderValue(self.uiSlider) }
    }

    fn set_value(&mut self, _ctx: &UI, value: i32) {
        unsafe { libui_sys::uiSliderSetValue(self.uiSlider, value) }
    }

    fn on_changed<'ctx, F: FnMut(i32) + 'ctx>(&mut self, _ctx: &'ctx UI, callback: F) {
        unsafe {
            let mut data: Box<Box<dyn FnMut(i32)>> = Box::new(Box::new(callback));
            libui_sys::uiSliderOnChanged(
                self.uiSlider,
                Some(c_callback),
                &mut *data as *mut Box<dyn FnMut(i32)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(slider: *mut uiSlider, data: *mut c_void) {
            unsafe {
                let val = libui_sys::uiSliderValue(slider);
                mem::transmute::<*mut c_void, &mut Box<dyn FnMut(i32)>>(data)(val);
            }
        }
    }
}

define_control! {
    /// Single-line editable text buffer.
    rust_type: Entry,
    sys_type: uiEntry
}

define_control! {
    /// Multi-line editable text buffer.
    rust_type: MultilineEntry,
    sys_type: uiMultilineEntry
}

impl Entry {
    pub fn new(_ctx: &UI) -> Entry {
        unsafe { Entry::from_raw(libui_sys::uiNewEntry()) }
    }
}

impl MultilineEntry {
    pub fn new(_ctx: &UI) -> MultilineEntry {
        unsafe { MultilineEntry::from_raw(libui_sys::uiNewMultilineEntry()) }
    }
}

impl TextEntry for Entry {
    fn value(&self, _ctx: &UI) -> String {
        unsafe {
            CStr::from_ptr(libui_sys::uiEntryText(self.uiEntry))
                .to_string_lossy()
                .into_owned()
        }
    }
    fn set_value(&mut self, _ctx: &UI, value: &str) {
        let cstring = CString::new(value.as_bytes().to_vec()).unwrap();
        unsafe { libui_sys::uiEntrySetText(self.uiEntry, cstring.as_ptr()) }
    }

    fn on_changed<'ctx, F: FnMut(String) + 'ctx>(&mut self, _ctx: &'ctx UI, callback: F) {
        unsafe {
            let mut data: Box<Box<dyn FnMut(String)>> = Box::new(Box::new(callback));
            libui_sys::uiEntryOnChanged(
                self.uiEntry,
                Some(c_callback),
                &mut *data as *mut Box<dyn FnMut(String)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(entry: *mut uiEntry, data: *mut c_void) {
            unsafe {
                let string = CStr::from_ptr(libui_sys::uiEntryText(entry))
                    .to_string_lossy()
                    .into_owned();
                mem::transmute::<*mut c_void, &mut Box<dyn FnMut(String)>>(data)(string);
                mem::forget(entry);
            }
        }
    }
}

impl TextEntry for MultilineEntry {
    fn value(&self, _ctx: &UI) -> String {
        unsafe {
            CStr::from_ptr(libui_sys::uiMultilineEntryText(self.uiMultilineEntry))
                .to_string_lossy()
                .into_owned()
        }
    }
    fn set_value(&mut self, _ctx: &UI, value: &str) {
        let cstring = CString::new(value.as_bytes().to_vec()).unwrap();
        unsafe { libui_sys::uiMultilineEntrySetText(self.uiMultilineEntry, cstring.as_ptr()) }
    }

    fn on_changed<'ctx, F: FnMut(String) + 'ctx>(&mut self, _ctx: &'ctx UI, callback: F) {
        unsafe {
            let mut data: Box<Box<dyn FnMut(String)>> = Box::new(Box::new(callback));
            libui_sys::uiMultilineEntryOnChanged(
                self.uiMultilineEntry,
                Some(c_callback),
                &mut *data as *mut Box<dyn FnMut(String)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(entry: *mut uiMultilineEntry, data: *mut c_void) {
            unsafe {
                let string = CStr::from_ptr(libui_sys::uiMultilineEntryText(entry))
                    .to_string_lossy()
                    .into_owned();
                mem::transmute::<*mut c_void, &mut Box<dyn FnMut(String)>>(data)(string);
                mem::forget(entry);
            }
        }
    }
}

define_control! {
    /// Allows the user to select any one of its options, from a list shown only when selected.
    rust_type: Combobox,
    sys_type: uiCombobox
}

impl Combobox {
    /// Create a new Combobox
    pub fn new(_ctx: &UI) -> Self {
        unsafe { Combobox::from_raw(libui_sys::uiNewCombobox()) }
    }

    /// Adds a new option to the combination box.
    pub fn append(&self, _ctx: &UI, name: &str) {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            libui_sys::uiComboboxAppend(self.uiCombobox, c_string.as_ptr())
        }
    }

    /// Returns the index of the currently selected option.
    pub fn selected(&self, _ctx: &UI) -> i32 {
        unsafe { libui_sys::uiComboboxSelected(self.uiCombobox) }
    }

    pub fn set_selected(&mut self, _ctx: &UI, value: i32) {
        unsafe { libui_sys::uiComboboxSetSelected(self.uiCombobox, value) }
    }

    pub fn on_selected<F: FnMut(i32)>(&mut self, _ctx: &UI, callback: F) {
        unsafe {
            let mut data: Box<Box<dyn FnMut(i32)>> = Box::new(Box::new(callback));
            libui_sys::uiComboboxOnSelected(
                self.uiCombobox,
                Some(c_callback),
                &mut *data as *mut Box<dyn FnMut(i32)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(combobox: *mut uiCombobox, data: *mut c_void) {
            unsafe {
                let val = libui_sys::uiComboboxSelected(combobox);
                mem::transmute::<*mut c_void, &mut Box<dyn FnMut(i32)>>(data)(val);
            }
        }
    }
}

define_control! {
    /// Boolean selection control which can be checked or unchecked.
    rust_type: Checkbox,
    sys_type: uiCheckbox
}

impl Checkbox {
    // Create a new Checkbox which can produce values from `min` to `max`.
    pub fn new(_ctx: &UI, text: &str) -> Self {
        let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
        unsafe { Checkbox::from_raw(libui_sys::uiNewCheckbox(c_string.as_ptr())) }
    }

    pub fn checked(&self, _ctx: &UI) -> bool {
        unsafe { libui_sys::uiCheckboxChecked(self.uiCheckbox) != 0 }
    }

    pub fn set_checked(&mut self, _ctx: &UI, checked: bool) {
        unsafe { libui_sys::uiCheckboxSetChecked(self.uiCheckbox, checked as i32) }
    }

    pub fn on_toggled<F: FnMut(bool)>(&mut self, _ctx: &UI, callback: F) {
        unsafe {
            let mut data: Box<Box<dyn FnMut(bool)>> = Box::new(Box::new(callback));
            libui_sys::uiCheckboxOnToggled(
                self.uiCheckbox,
                Some(c_callback),
                &mut *data as *mut Box<dyn FnMut(bool)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(checkbox: *mut uiCheckbox, data: *mut c_void) {
            unsafe {
                let val = libui_sys::uiCheckboxChecked(checkbox) != 0;
                mem::transmute::<*mut c_void, &mut Box<dyn FnMut(bool)>>(data)(val);
            }
        }
    }
}

define_control! {
    /// A set of toggles; only one can be selected at a time.
    rust_type: RadioButtons,
    sys_type: uiRadioButtons
}

impl RadioButtons {
    pub fn new(_ctx: &UI) -> Self {
        unsafe { RadioButtons::from_raw(libui_sys::uiNewRadioButtons()) }
    }

    pub fn append(&self, _ctx: &UI, name: &str) {
        let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
        unsafe {
            libui_sys::uiRadioButtonsAppend(self.uiRadioButtons, c_string.as_ptr());
        }
    }

    pub fn selected(&self, _ctx: &UI) -> i32 {
        unsafe { libui_sys::uiRadioButtonsSelected(self.uiRadioButtons) }
    }

    pub fn set_selected(&mut self, _ctx: &UI, idx: i32) {
        unsafe {
            libui_sys::uiRadioButtonsSetSelected(self.uiRadioButtons, idx);
        }
    }

    pub fn on_selected<'ctx, F: FnMut(i32) + 'ctx>(&self, _ctx: &'ctx UI, callback: F) {
        unsafe {
            let mut data: Box<Box<dyn FnMut(i32)>> = Box::new(Box::new(callback));
            libui_sys::uiRadioButtonsOnSelected(
                self.uiRadioButtons,
                Some(c_callback),
                &mut *data as *mut Box<dyn FnMut(i32)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(radio_buttons: *mut uiRadioButtons, data: *mut c_void) {
            unsafe {
                let val = libui_sys::uiRadioButtonsSelected(radio_buttons);
                mem::transmute::<*mut c_void, &mut Box<dyn FnMut(i32)>>(data)(val);
            }
        }
    }
}
