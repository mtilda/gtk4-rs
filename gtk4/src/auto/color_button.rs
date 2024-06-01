// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ColorChooser, ConstraintTarget,
    LayoutManager, Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkColorButton")]
    pub struct ColorButton(Object<ffi::GtkColorButton>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, ColorChooser;

    match fn {
        type_ => || ffi::gtk_color_button_get_type(),
    }
}

impl ColorButton {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_color_button_new")]
    pub fn new() -> ColorButton {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_color_button_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_color_button_new_with_rgba")]
    #[doc(alias = "new_with_rgba")]
    pub fn with_rgba(rgba: &gdk::RGBA) -> ColorButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_button_new_with_rgba(rgba.to_glib_none().0))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ColorButton`] objects.
    ///
    /// This method returns an instance of [`ColorButtonBuilder`](crate::builders::ColorButtonBuilder) which can be used to create [`ColorButton`] objects.
    pub fn builder() -> ColorButtonBuilder {
        ColorButtonBuilder::new()
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_color_button_get_modal")]
    #[doc(alias = "get_modal")]
    pub fn is_modal(&self) -> bool {
        unsafe { from_glib(ffi::gtk_color_button_get_modal(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_color_button_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_color_button_get_title(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_color_button_set_modal")]
    pub fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_color_button_set_modal(self.to_glib_none().0, modal.into_glib());
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_color_button_set_title")]
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_color_button_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[doc(alias = "show-editor")]
    pub fn shows_editor(&self) -> bool {
        ObjectExt::property(self, "show-editor")
    }

    #[doc(alias = "show-editor")]
    pub fn set_show_editor(&self, show_editor: bool) {
        ObjectExt::set_property(self, "show-editor", show_editor)
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "activate")]
    pub fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&ColorButton) + 'static>(
            this: *mut ffi::GtkColorButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    pub fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    #[doc(alias = "color-set")]
    pub fn connect_color_set<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn color_set_trampoline<F: Fn(&ColorButton) + 'static>(
            this: *mut ffi::GtkColorButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"color-set\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    color_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "modal")]
    pub fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<F: Fn(&ColorButton) + 'static>(
            this: *mut ffi::GtkColorButton,
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

    #[doc(alias = "show-editor")]
    pub fn connect_show_editor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_editor_trampoline<F: Fn(&ColorButton) + 'static>(
            this: *mut ffi::GtkColorButton,
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
                b"notify::show-editor\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_editor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&ColorButton) + 'static>(
            this: *mut ffi::GtkColorButton,
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
}

impl Default for ColorButton {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ColorButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ColorButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, ColorButton>,
}

impl ColorButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn show_editor(self, show_editor: bool) -> Self {
        Self {
            builder: self.builder.property("show-editor", show_editor),
        }
    }

    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn rgba(self, rgba: &gdk::RGBA) -> Self {
        Self {
            builder: self.builder.property("rgba", rgba),
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn use_alpha(self, use_alpha: bool) -> Self {
        Self {
            builder: self.builder.property("use-alpha", use_alpha),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ColorButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ColorButton {
        self.builder.build()
    }
}
