// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Adjustment, Align, Buildable, ConstraintTarget, LayoutManager,
    ListBase, ListItemFactory, Orientable, Orientation, Overflow, Scrollable, ScrollablePolicy,
    SelectionModel, Widget,
};
#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
use crate::{ListScrollFlags, ListTabBehavior, ScrollInfo};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkListView")]
    pub struct ListView(Object<ffi::GtkListView, ffi::GtkListViewClass>) @extends ListBase, Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable, Scrollable;

    match fn {
        type_ => || ffi::gtk_list_view_get_type(),
    }
}

impl ListView {
    #[doc(alias = "gtk_list_view_new")]
    pub fn new(
        model: Option<impl IsA<SelectionModel>>,
        factory: Option<impl IsA<ListItemFactory>>,
    ) -> ListView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_list_view_new(
                model.map(|p| p.upcast()).into_glib_ptr(),
                factory.map(|p| p.upcast()).into_glib_ptr(),
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ListView`] objects.
    ///
    /// This method returns an instance of [`ListViewBuilder`](crate::builders::ListViewBuilder) which can be used to create [`ListView`] objects.
    pub fn builder() -> ListViewBuilder {
        ListViewBuilder::new()
    }

    #[doc(alias = "gtk_list_view_get_enable_rubberband")]
    #[doc(alias = "get_enable_rubberband")]
    pub fn enables_rubberband(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_view_get_enable_rubberband(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_list_view_get_factory")]
    #[doc(alias = "get_factory")]
    pub fn factory(&self) -> Option<ListItemFactory> {
        unsafe { from_glib_none(ffi::gtk_list_view_get_factory(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_list_view_get_header_factory")]
    #[doc(alias = "get_header_factory")]
    pub fn header_factory(&self) -> Option<ListItemFactory> {
        unsafe { from_glib_none(ffi::gtk_list_view_get_header_factory(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_list_view_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<SelectionModel> {
        unsafe { from_glib_none(ffi::gtk_list_view_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_list_view_get_show_separators")]
    #[doc(alias = "get_show_separators")]
    pub fn shows_separators(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_view_get_show_separators(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_list_view_get_single_click_activate")]
    #[doc(alias = "get_single_click_activate")]
    pub fn is_single_click_activate(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_view_get_single_click_activate(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_list_view_get_tab_behavior")]
    #[doc(alias = "get_tab_behavior")]
    pub fn tab_behavior(&self) -> ListTabBehavior {
        unsafe { from_glib(ffi::gtk_list_view_get_tab_behavior(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_list_view_scroll_to")]
    pub fn scroll_to(&self, pos: u32, flags: ListScrollFlags, scroll: Option<ScrollInfo>) {
        unsafe {
            ffi::gtk_list_view_scroll_to(
                self.to_glib_none().0,
                pos,
                flags.into_glib(),
                scroll.into_glib_ptr(),
            );
        }
    }

    #[doc(alias = "gtk_list_view_set_enable_rubberband")]
    pub fn set_enable_rubberband(&self, enable_rubberband: bool) {
        unsafe {
            ffi::gtk_list_view_set_enable_rubberband(
                self.to_glib_none().0,
                enable_rubberband.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_list_view_set_factory")]
    pub fn set_factory(&self, factory: Option<&impl IsA<ListItemFactory>>) {
        unsafe {
            ffi::gtk_list_view_set_factory(
                self.to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_list_view_set_header_factory")]
    pub fn set_header_factory(&self, factory: Option<&impl IsA<ListItemFactory>>) {
        unsafe {
            ffi::gtk_list_view_set_header_factory(
                self.to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_list_view_set_model")]
    pub fn set_model(&self, model: Option<&impl IsA<SelectionModel>>) {
        unsafe {
            ffi::gtk_list_view_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_list_view_set_show_separators")]
    pub fn set_show_separators(&self, show_separators: bool) {
        unsafe {
            ffi::gtk_list_view_set_show_separators(
                self.to_glib_none().0,
                show_separators.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_list_view_set_single_click_activate")]
    pub fn set_single_click_activate(&self, single_click_activate: bool) {
        unsafe {
            ffi::gtk_list_view_set_single_click_activate(
                self.to_glib_none().0,
                single_click_activate.into_glib(),
            );
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_list_view_set_tab_behavior")]
    pub fn set_tab_behavior(&self, tab_behavior: ListTabBehavior) {
        unsafe {
            ffi::gtk_list_view_set_tab_behavior(self.to_glib_none().0, tab_behavior.into_glib());
        }
    }

    #[doc(alias = "activate")]
    pub fn connect_activate<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&ListView, u32) + 'static>(
            this: *mut ffi::GtkListView,
            position: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), position)
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

    #[doc(alias = "enable-rubberband")]
    pub fn connect_enable_rubberband_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_rubberband_trampoline<F: Fn(&ListView) + 'static>(
            this: *mut ffi::GtkListView,
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
                b"notify::enable-rubberband\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_enable_rubberband_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "factory")]
    pub fn connect_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_factory_trampoline<F: Fn(&ListView) + 'static>(
            this: *mut ffi::GtkListView,
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
                b"notify::factory\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_factory_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "header-factory")]
    pub fn connect_header_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_header_factory_trampoline<F: Fn(&ListView) + 'static>(
            this: *mut ffi::GtkListView,
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
                b"notify::header-factory\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_header_factory_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&ListView) + 'static>(
            this: *mut ffi::GtkListView,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-separators")]
    pub fn connect_show_separators_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_separators_trampoline<F: Fn(&ListView) + 'static>(
            this: *mut ffi::GtkListView,
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
                b"notify::show-separators\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_separators_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "single-click-activate")]
    pub fn connect_single_click_activate_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_single_click_activate_trampoline<F: Fn(&ListView) + 'static>(
            this: *mut ffi::GtkListView,
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
                b"notify::single-click-activate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_single_click_activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "tab-behavior")]
    pub fn connect_tab_behavior_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_behavior_trampoline<F: Fn(&ListView) + 'static>(
            this: *mut ffi::GtkListView,
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
                b"notify::tab-behavior\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_tab_behavior_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ListView {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ListView`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ListViewBuilder {
    builder: glib::object::ObjectBuilder<'static, ListView>,
}

impl ListViewBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn enable_rubberband(self, enable_rubberband: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("enable-rubberband", enable_rubberband),
        }
    }

    pub fn factory(self, factory: &impl IsA<ListItemFactory>) -> Self {
        Self {
            builder: self.builder.property("factory", factory.clone().upcast()),
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    pub fn header_factory(self, header_factory: &impl IsA<ListItemFactory>) -> Self {
        Self {
            builder: self
                .builder
                .property("header-factory", header_factory.clone().upcast()),
        }
    }

    pub fn model(self, model: &impl IsA<SelectionModel>) -> Self {
        Self {
            builder: self.builder.property("model", model.clone().upcast()),
        }
    }

    pub fn show_separators(self, show_separators: bool) -> Self {
        Self {
            builder: self.builder.property("show-separators", show_separators),
        }
    }

    pub fn single_click_activate(self, single_click_activate: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("single-click-activate", single_click_activate),
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    pub fn tab_behavior(self, tab_behavior: ListTabBehavior) -> Self {
        Self {
            builder: self.builder.property("tab-behavior", tab_behavior),
        }
    }

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
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

    pub fn hadjustment(self, hadjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("hadjustment", hadjustment.clone().upcast()),
        }
    }

    pub fn hscroll_policy(self, hscroll_policy: ScrollablePolicy) -> Self {
        Self {
            builder: self.builder.property("hscroll-policy", hscroll_policy),
        }
    }

    pub fn vadjustment(self, vadjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("vadjustment", vadjustment.clone().upcast()),
        }
    }

    pub fn vscroll_policy(self, vscroll_policy: ScrollablePolicy) -> Self {
        Self {
            builder: self.builder.property("vscroll-policy", vscroll_policy),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ListView`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ListView {
        self.builder.build()
    }
}
