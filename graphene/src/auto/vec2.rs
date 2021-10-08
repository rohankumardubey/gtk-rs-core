// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Vec2(Boxed<ffi::graphene_vec2_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_vec2_get_type(), ptr as *mut _) as *mut ffi::graphene_vec2_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_vec2_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        type_ => || ffi::graphene_vec2_get_type(),
    }
}

impl Vec2 {
    #[doc(alias = "graphene_vec2_add")]
    pub fn add(&self, b: &Vec2) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            ffi::graphene_vec2_add(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec2_divide")]
    pub fn divide(&self, b: &Vec2) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            ffi::graphene_vec2_divide(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec2_dot")]
    pub fn dot(&self, b: &Vec2) -> f32 {
        unsafe { ffi::graphene_vec2_dot(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_vec2_equal")]
    fn equal(&self, v2: &Vec2) -> bool {
        unsafe { ffi::graphene_vec2_equal(self.to_glib_none().0, v2.to_glib_none().0) }
    }

    #[doc(alias = "graphene_vec2_get_x")]
    #[doc(alias = "get_x")]
    pub fn x(&self) -> f32 {
        unsafe { ffi::graphene_vec2_get_x(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_vec2_get_y")]
    #[doc(alias = "get_y")]
    pub fn y(&self) -> f32 {
        unsafe { ffi::graphene_vec2_get_y(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_vec2_init")]
    pub fn init(&mut self, x: f32, y: f32) {
        unsafe {
            ffi::graphene_vec2_init(self.to_glib_none_mut().0, x, y);
        }
    }

    //#[doc(alias = "graphene_vec2_init_from_float")]
    //pub fn init_from_float(&mut self, src: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 2) -> Option<Vec2> {
    //    unsafe { TODO: call ffi:graphene_vec2_init_from_float() }
    //}

    #[doc(alias = "graphene_vec2_init_from_vec2")]
    pub fn init_from_vec2(&mut self, src: &Vec2) {
        unsafe {
            ffi::graphene_vec2_init_from_vec2(self.to_glib_none_mut().0, src.to_glib_none().0);
        }
    }

    #[doc(alias = "graphene_vec2_interpolate")]
    pub fn interpolate(&self, v2: &Vec2, factor: f64) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            ffi::graphene_vec2_interpolate(
                self.to_glib_none().0,
                v2.to_glib_none().0,
                factor,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec2_length")]
    pub fn length(&self) -> f32 {
        unsafe { ffi::graphene_vec2_length(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_vec2_max")]
    pub fn max(&self, b: &Vec2) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            ffi::graphene_vec2_max(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec2_min")]
    pub fn min(&self, b: &Vec2) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            ffi::graphene_vec2_min(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec2_multiply")]
    pub fn multiply(&self, b: &Vec2) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            ffi::graphene_vec2_multiply(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec2_near")]
    pub fn near(&self, v2: &Vec2, epsilon: f32) -> bool {
        unsafe { ffi::graphene_vec2_near(self.to_glib_none().0, v2.to_glib_none().0, epsilon) }
    }

    #[doc(alias = "graphene_vec2_negate")]
    pub fn negate(&self) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            ffi::graphene_vec2_negate(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_vec2_normalize")]
    pub fn normalize(&self) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            ffi::graphene_vec2_normalize(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_vec2_scale")]
    pub fn scale(&self, factor: f32) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            ffi::graphene_vec2_scale(self.to_glib_none().0, factor, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_vec2_subtract")]
    pub fn subtract(&self, b: &Vec2) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            ffi::graphene_vec2_subtract(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    //#[doc(alias = "graphene_vec2_to_float")]
    //pub fn to_float(&self, dest: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 2) {
    //    unsafe { TODO: call ffi:graphene_vec2_to_float() }
    //}

    #[doc(alias = "graphene_vec2_one")]
    pub fn one() -> Vec2 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_vec2_one()) }
    }

    #[doc(alias = "graphene_vec2_x_axis")]
    pub fn x_axis() -> Vec2 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_vec2_x_axis()) }
    }

    #[doc(alias = "graphene_vec2_y_axis")]
    pub fn y_axis() -> Vec2 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_vec2_y_axis()) }
    }

    #[doc(alias = "graphene_vec2_zero")]
    pub fn zero() -> Vec2 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_vec2_zero()) }
    }
}

impl PartialEq for Vec2 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Vec2 {}
