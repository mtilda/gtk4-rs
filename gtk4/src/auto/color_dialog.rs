// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Window};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "GtkColorDialog")]
    pub struct ColorDialog(Object<ffi::GtkColorDialog, ffi::GtkColorDialogClass>);

    match fn {
        type_ => || ffi::gtk_color_dialog_get_type(),
    }
}

impl ColorDialog {
    #[doc(alias = "gtk_color_dialog_new")]
    pub fn new() -> ColorDialog {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_color_dialog_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ColorDialog`] objects.
    ///
    /// This method returns an instance of [`ColorDialogBuilder`](crate::builders::ColorDialogBuilder) which can be used to create [`ColorDialog`] objects.
    pub fn builder() -> ColorDialogBuilder {
        ColorDialogBuilder::new()
    }

    #[doc(alias = "gtk_color_dialog_choose_rgba")]
    pub fn choose_rgba<P: FnOnce(Result<gdk::RGBA, glib::Error>) + 'static>(
        &self,
        parent: Option<&impl IsA<Window>>,
        initial_color: Option<&gdk::RGBA>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn choose_rgba_trampoline<
            P: FnOnce(Result<gdk::RGBA, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::gtk_color_dialog_choose_rgba_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = choose_rgba_trampoline::<P>;
        unsafe {
            ffi::gtk_color_dialog_choose_rgba(
                self.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                initial_color.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn choose_rgba_future(
        &self,
        parent: Option<&(impl IsA<Window> + Clone + 'static)>,
        initial_color: Option<&gdk::RGBA>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<gdk::RGBA, glib::Error>> + 'static>> {
        let parent = parent.map(ToOwned::to_owned);
        let initial_color = initial_color.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.choose_rgba(
                parent.as_ref().map(::std::borrow::Borrow::borrow),
                initial_color.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "gtk_color_dialog_get_modal")]
    #[doc(alias = "get_modal")]
    pub fn is_modal(&self) -> bool {
        unsafe { from_glib(ffi::gtk_color_dialog_get_modal(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_color_dialog_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_color_dialog_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_color_dialog_get_with_alpha")]
    #[doc(alias = "get_with_alpha")]
    pub fn is_with_alpha(&self) -> bool {
        unsafe { from_glib(ffi::gtk_color_dialog_get_with_alpha(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_color_dialog_set_modal")]
    pub fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_color_dialog_set_modal(self.to_glib_none().0, modal.into_glib());
        }
    }

    #[doc(alias = "gtk_color_dialog_set_title")]
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_color_dialog_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_color_dialog_set_with_alpha")]
    pub fn set_with_alpha(&self, with_alpha: bool) {
        unsafe {
            ffi::gtk_color_dialog_set_with_alpha(self.to_glib_none().0, with_alpha.into_glib());
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "modal")]
    pub fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<F: Fn(&ColorDialog) + 'static>(
            this: *mut ffi::GtkColorDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modal\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_modal_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&ColorDialog) + 'static>(
            this: *mut ffi::GtkColorDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "with-alpha")]
    pub fn connect_with_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_with_alpha_trampoline<F: Fn(&ColorDialog) + 'static>(
            this: *mut ffi::GtkColorDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::with-alpha\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_with_alpha_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
impl Default for ColorDialog {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ColorDialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ColorDialogBuilder {
    builder: glib::object::ObjectBuilder<'static, ColorDialog>,
}

impl ColorDialogBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn with_alpha(self, with_alpha: bool) -> Self {
        Self {
            builder: self.builder.property("with-alpha", with_alpha),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ColorDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ColorDialog {
        self.builder.build()
    }
}
