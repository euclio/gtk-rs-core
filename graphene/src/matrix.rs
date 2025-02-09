// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Matrix;
use crate::Point3D;
use crate::Vec3;
use crate::Vec4;
use glib::translate::*;

impl Matrix {
    #[doc(alias = "graphene_matrix_init_from_2d")]
    #[doc(alias = "new_from_2d")]
    pub fn from_2d(xx: f64, yx: f64, xy: f64, yy: f64, x_0: f64, y_0: f64) -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_from_2d(mat.to_glib_none_mut().0, xx, yx, xy, yy, x_0, y_0);
            mat
        }
    }

    #[doc(alias = "graphene_matrix_init_from_float")]
    #[doc(alias = "new_from_float")]
    pub fn from_float(v: &[f32; 16]) -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_from_float(mat.to_glib_none_mut().0, v.as_ptr() as *const _);
            mat
        }
    }

    #[doc(alias = "graphene_matrix_init_from_matrix")]
    #[doc(alias = "new_from_matrix")]
    pub fn from_matrix(src: &Matrix) -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_from_matrix(mat.to_glib_none_mut().0, src.to_glib_none().0);
            mat
        }
    }

    #[doc(alias = "graphene_matrix_init_from_vec4")]
    #[doc(alias = "new_from_vec4")]
    pub fn from_vec4(v0: &Vec4, v1: &Vec4, v2: &Vec4, v3: &Vec4) -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_from_vec4(
                mat.to_glib_none_mut().0,
                v0.to_glib_none().0,
                v1.to_glib_none().0,
                v2.to_glib_none().0,
                v3.to_glib_none().0,
            );
            mat
        }
    }

    #[doc(alias = "graphene_matrix_init_frustum")]
    pub fn new_frustum(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        z_near: f32,
        z_far: f32,
    ) -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_frustum(
                mat.to_glib_none_mut().0,
                left,
                right,
                bottom,
                top,
                z_near,
                z_far,
            );
            mat
        }
    }

    #[doc(alias = "graphene_matrix_init_identity")]
    pub fn new_identity() -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_identity(mat.to_glib_none_mut().0);
            mat
        }
    }

    #[doc(alias = "graphene_matrix_init_look_at")]
    pub fn new_look_at(eye: &Vec3, center: &Vec3, up: &Vec3) -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_look_at(
                mat.to_glib_none_mut().0,
                eye.to_glib_none().0,
                center.to_glib_none().0,
                up.to_glib_none().0,
            );
            mat
        }
    }

    #[doc(alias = "graphene_matrix_init_ortho")]
    pub fn new_ortho(
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        z_near: f32,
        z_far: f32,
    ) -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_ortho(
                mat.to_glib_none_mut().0,
                left,
                right,
                top,
                bottom,
                z_near,
                z_far,
            );
            mat
        }
    }

    #[doc(alias = "graphene_matrix_init_perspective")]
    pub fn new_perspective(fovy: f32, aspect: f32, z_near: f32, z_far: f32) -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_perspective(
                mat.to_glib_none_mut().0,
                fovy,
                aspect,
                z_near,
                z_far,
            );
            mat
        }
    }

    #[doc(alias = "graphene_matrix_init_rotate")]
    pub fn new_rotate(angle: f32, axis: &Vec3) -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_rotate(
                mat.to_glib_none_mut().0,
                angle,
                axis.to_glib_none().0,
            );
            mat
        }
    }

    #[doc(alias = "graphene_matrix_init_scale")]
    pub fn new_scale(x: f32, y: f32, z: f32) -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_scale(mat.to_glib_none_mut().0, x, y, z);
            mat
        }
    }

    #[doc(alias = "graphene_matrix_init_skew")]
    pub fn new_skew(x_skew: f32, y_skew: f32) -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_skew(mat.to_glib_none_mut().0, x_skew, y_skew);
            mat
        }
    }

    #[doc(alias = "graphene_matrix_init_translate")]
    pub fn new_translate(p: &Point3D) -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            let mut mat = Matrix::uninitialized();
            ffi::graphene_matrix_init_translate(mat.to_glib_none_mut().0, p.to_glib_none().0);
            mat
        }
    }

    #[doc(alias = "graphene_matrix_to_float")]
    pub fn to_float(&self) -> [f32; 16] {
        unsafe {
            let mut out = std::mem::MaybeUninit::uninit();
            ffi::graphene_matrix_to_float(self.to_glib_none().0, out.as_mut_ptr());
            out.assume_init()
        }
    }
}
