// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Overflow,
    Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkHeaderBar")]
    pub struct HeaderBar(Object<ffi::GtkHeaderBar>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_header_bar_get_type(),
    }
}

impl HeaderBar {
    #[doc(alias = "gtk_header_bar_new")]
    pub fn new() -> HeaderBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_header_bar_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`HeaderBar`] objects.
    ///
    /// This method returns an instance of [`HeaderBarBuilder`](crate::builders::HeaderBarBuilder) which can be used to create [`HeaderBar`] objects.
    pub fn builder() -> HeaderBarBuilder {
        HeaderBarBuilder::new()
    }

    #[doc(alias = "gtk_header_bar_get_decoration_layout")]
    #[doc(alias = "get_decoration_layout")]
    pub fn decoration_layout(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_decoration_layout(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_header_bar_get_show_title_buttons")]
    #[doc(alias = "get_show_title_buttons")]
    pub fn shows_title_buttons(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_header_bar_get_show_title_buttons(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_header_bar_get_title_widget")]
    #[doc(alias = "get_title_widget")]
    pub fn title_widget(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_header_bar_get_title_widget(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_header_bar_pack_end")]
    pub fn pack_end(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_header_bar_pack_end(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_header_bar_pack_start")]
    pub fn pack_start(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_header_bar_pack_start(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_header_bar_remove")]
    pub fn remove(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_header_bar_remove(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_header_bar_set_decoration_layout")]
    pub fn set_decoration_layout(&self, layout: Option<&str>) {
        unsafe {
            ffi::gtk_header_bar_set_decoration_layout(
                self.to_glib_none().0,
                layout.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_header_bar_set_show_title_buttons")]
    pub fn set_show_title_buttons(&self, setting: bool) {
        unsafe {
            ffi::gtk_header_bar_set_show_title_buttons(self.to_glib_none().0, setting.into_glib());
        }
    }

    #[doc(alias = "gtk_header_bar_set_title_widget")]
    pub fn set_title_widget(&self, title_widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_header_bar_set_title_widget(
                self.to_glib_none().0,
                title_widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "decoration-layout")]
    pub fn connect_decoration_layout_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_decoration_layout_trampoline<F: Fn(&HeaderBar) + 'static>(
            this: *mut ffi::GtkHeaderBar,
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
                b"notify::decoration-layout\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_decoration_layout_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-title-buttons")]
    pub fn connect_show_title_buttons_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_title_buttons_trampoline<F: Fn(&HeaderBar) + 'static>(
            this: *mut ffi::GtkHeaderBar,
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
                b"notify::show-title-buttons\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_title_buttons_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title-widget")]
    pub fn connect_title_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_widget_trampoline<F: Fn(&HeaderBar) + 'static>(
            this: *mut ffi::GtkHeaderBar,
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
                b"notify::title-widget\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_widget_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for HeaderBar {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`HeaderBar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct HeaderBarBuilder {
    builder: glib::object::ObjectBuilder<'static, HeaderBar>,
}

impl HeaderBarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn decoration_layout(self, decoration_layout: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("decoration-layout", decoration_layout.into()),
        }
    }

    pub fn show_title_buttons(self, show_title_buttons: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-title-buttons", show_title_buttons),
        }
    }

    pub fn title_widget(self, title_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("title-widget", title_widget.clone().upcast()),
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

    // rustdoc-stripper-ignore-next
    /// Build the [`HeaderBar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> HeaderBar {
        self.builder.build()
    }
}
