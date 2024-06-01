// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ColorStop};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GskRadialGradientNode")]
    pub struct RadialGradientNode(Shared<ffi::GskRadialGradientNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl StaticType for RadialGradientNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_radial_gradient_node_get_type()) }
    }
}

impl RadialGradientNode {
    #[doc(alias = "gsk_radial_gradient_node_new")]
    pub fn new(
        bounds: &graphene::Rect,
        center: &graphene::Point,
        hradius: f32,
        vradius: f32,
        start: f32,
        end: f32,
        color_stops: &[ColorStop],
    ) -> RadialGradientNode {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as _;
        unsafe {
            from_glib_full(ffi::gsk_radial_gradient_node_new(
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                hradius,
                vradius,
                start,
                end,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        }
    }

    #[doc(alias = "gsk_radial_gradient_node_get_center")]
    #[doc(alias = "get_center")]
    pub fn center(&self) -> graphene::Point {
        unsafe {
            from_glib_none(ffi::gsk_radial_gradient_node_get_center(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_radial_gradient_node_get_color_stops")]
    #[doc(alias = "get_color_stops")]
    pub fn color_stops(&self) -> Vec<ColorStop> {
        unsafe {
            let mut n_stops = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::gsk_radial_gradient_node_get_color_stops(
                    self.to_glib_none().0,
                    n_stops.as_mut_ptr(),
                ),
                n_stops.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "gsk_radial_gradient_node_get_end")]
    #[doc(alias = "get_end")]
    pub fn end(&self) -> f32 {
        unsafe { ffi::gsk_radial_gradient_node_get_end(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_radial_gradient_node_get_hradius")]
    #[doc(alias = "get_hradius")]
    pub fn hradius(&self) -> f32 {
        unsafe { ffi::gsk_radial_gradient_node_get_hradius(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_radial_gradient_node_get_n_color_stops")]
    #[doc(alias = "get_n_color_stops")]
    pub fn n_color_stops(&self) -> usize {
        unsafe { ffi::gsk_radial_gradient_node_get_n_color_stops(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_radial_gradient_node_get_start")]
    #[doc(alias = "get_start")]
    pub fn start(&self) -> f32 {
        unsafe { ffi::gsk_radial_gradient_node_get_start(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_radial_gradient_node_get_vradius")]
    #[doc(alias = "get_vradius")]
    pub fn vradius(&self) -> f32 {
        unsafe { ffi::gsk_radial_gradient_node_get_vradius(self.to_glib_none().0) }
    }
}
