// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ShortcutAction};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GtkNothingAction")]
    pub struct NothingAction(Object<ffi::GtkNothingAction, ffi::GtkNothingActionClass>) @extends ShortcutAction;

    match fn {
        type_ => || ffi::gtk_nothing_action_get_type(),
    }
}

impl NothingAction {
    #[doc(alias = "gtk_nothing_action_get")]
    pub fn get() -> NothingAction {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_nothing_action_get()) }
    }
}
