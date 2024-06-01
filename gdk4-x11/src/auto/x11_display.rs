// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, X11Screen};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GdkX11Display")]
    pub struct X11Display(Object<ffi::GdkX11Display, ffi::GdkX11DisplayClass>) @extends gdk::Display;

    match fn {
        type_ => || ffi::gdk_x11_display_get_type(),
    }
}

impl X11Display {
    //#[doc(alias = "gdk_x11_display_broadcast_startup_message")]
    //pub fn broadcast_startup_message(&self, message_type: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:gdk_x11_display_broadcast_startup_message() }
    //}

    #[doc(alias = "gdk_x11_display_error_trap_pop")]
    pub fn error_trap_pop(&self) -> i32 {
        unsafe { ffi::gdk_x11_display_error_trap_pop(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_x11_display_error_trap_pop_ignored")]
    pub fn error_trap_pop_ignored(&self) {
        unsafe {
            ffi::gdk_x11_display_error_trap_pop_ignored(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_x11_display_error_trap_push")]
    pub fn error_trap_push(&self) {
        unsafe {
            ffi::gdk_x11_display_error_trap_push(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_x11_display_get_default_group")]
    #[doc(alias = "get_default_group")]
    pub fn default_group(&self) -> gdk::Surface {
        unsafe {
            from_glib_none(ffi::gdk_x11_display_get_default_group(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gdk_x11_display_get_egl_version")]
    #[doc(alias = "get_egl_version")]
    pub fn egl_version(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut major = std::mem::MaybeUninit::uninit();
            let mut minor = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_x11_display_get_egl_version(
                self.to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            ));
            if ret {
                Some((major.assume_init(), minor.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_x11_display_get_glx_version")]
    #[doc(alias = "get_glx_version")]
    pub fn glx_version(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut major = std::mem::MaybeUninit::uninit();
            let mut minor = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_x11_display_get_glx_version(
                self.to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            ));
            if ret {
                Some((major.assume_init(), minor.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_x11_display_get_primary_monitor")]
    #[doc(alias = "get_primary_monitor")]
    pub fn primary_monitor(&self) -> gdk::Monitor {
        unsafe {
            from_glib_none(ffi::gdk_x11_display_get_primary_monitor(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_x11_display_get_screen")]
    #[doc(alias = "get_screen")]
    pub fn screen(&self) -> X11Screen {
        unsafe { from_glib_none(ffi::gdk_x11_display_get_screen(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gdk_x11_display_get_startup_notification_id")]
    #[doc(alias = "get_startup_notification_id")]
    pub fn startup_notification_id(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gdk_x11_display_get_startup_notification_id(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_x11_display_get_user_time")]
    #[doc(alias = "get_user_time")]
    pub fn user_time(&self) -> u32 {
        unsafe { ffi::gdk_x11_display_get_user_time(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_x11_display_grab")]
    pub fn grab(&self) {
        unsafe {
            ffi::gdk_x11_display_grab(self.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_16", deprecated = "Since 4.16")]
    #[allow(deprecated)]
    #[doc(alias = "gdk_x11_display_set_cursor_theme")]
    pub fn set_cursor_theme(&self, theme: Option<&str>, size: i32) {
        unsafe {
            ffi::gdk_x11_display_set_cursor_theme(
                self.to_glib_none().0,
                theme.to_glib_none().0,
                size,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gdk_x11_display_set_startup_notification_id")]
    pub fn set_startup_notification_id(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_x11_display_set_startup_notification_id(
                self.to_glib_none().0,
                startup_id.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_x11_display_set_surface_scale")]
    pub fn set_surface_scale(&self, scale: i32) {
        unsafe {
            ffi::gdk_x11_display_set_surface_scale(self.to_glib_none().0, scale);
        }
    }

    #[doc(alias = "gdk_x11_display_string_to_compound_text")]
    pub fn string_to_compound_text(&self, str: &str) -> (i32, glib::GString, i32, Vec<u8>) {
        unsafe {
            let mut encoding = std::ptr::null();
            let mut format = std::mem::MaybeUninit::uninit();
            let mut ctext = std::ptr::null_mut();
            let mut length = std::mem::MaybeUninit::uninit();
            let ret = ffi::gdk_x11_display_string_to_compound_text(
                self.to_glib_none().0,
                str.to_glib_none().0,
                &mut encoding,
                format.as_mut_ptr(),
                &mut ctext,
                length.as_mut_ptr(),
            );
            (
                ret,
                from_glib_none(encoding),
                format.assume_init(),
                FromGlibContainer::from_glib_full_num(ctext, length.assume_init() as _),
            )
        }
    }

    #[doc(alias = "gdk_x11_display_ungrab")]
    pub fn ungrab(&self) {
        unsafe {
            ffi::gdk_x11_display_ungrab(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_x11_display_utf8_to_compound_text")]
    pub fn utf8_to_compound_text(&self, str: &str) -> Option<(glib::GString, i32, Vec<u8>)> {
        unsafe {
            let mut encoding = std::ptr::null();
            let mut format = std::mem::MaybeUninit::uninit();
            let mut ctext = std::ptr::null_mut();
            let mut length = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_x11_display_utf8_to_compound_text(
                self.to_glib_none().0,
                str.to_glib_none().0,
                &mut encoding,
                format.as_mut_ptr(),
                &mut ctext,
                length.as_mut_ptr(),
            ));
            if ret {
                Some((
                    from_glib_none(encoding),
                    format.assume_init(),
                    FromGlibContainer::from_glib_full_num(ctext, length.assume_init() as _),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_x11_display_open")]
    pub fn open(display_name: Option<&str>) -> Option<gdk::Display> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gdk_x11_display_open(display_name.to_glib_none().0)) }
    }
}
