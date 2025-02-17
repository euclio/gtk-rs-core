// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Box;
use crate::Point3D;
use crate::Vec3;
use glib::translate::*;

impl Box {
    #[doc(alias = "graphene_box_get_vertices")]
    #[doc(alias = "get_vertices")]
    pub fn vertices(&self) -> [Vec3; 8] {
        unsafe {
            let mut out: [ffi::graphene_vec3_t; 8] = std::mem::zeroed();
            ffi::graphene_box_get_vertices(self.to_glib_none().0, &mut out as *mut _);
            [
                from_glib_none(&out[0] as *const _),
                from_glib_none(&out[1] as *const _),
                from_glib_none(&out[2] as *const _),
                from_glib_none(&out[3] as *const _),
                from_glib_none(&out[4] as *const _),
                from_glib_none(&out[5] as *const _),
                from_glib_none(&out[6] as *const _),
                from_glib_none(&out[7] as *const _),
            ]
        }
    }

    #[doc(alias = "graphene_box_init")]
    pub fn new(min: Option<&Point3D>, max: Option<&Point3D>) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            let mut b = Box::uninitialized();
            ffi::graphene_box_init(
                b.to_glib_none_mut().0,
                min.to_glib_none().0,
                max.to_glib_none().0,
            );
            b
        }
    }

    #[doc(alias = "graphene_box_init_from_box")]
    #[doc(alias = "new_from_box")]
    pub fn from_box(src: &Box) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            let mut b = Box::uninitialized();
            ffi::graphene_box_init_from_box(b.to_glib_none_mut().0, src.to_glib_none().0);
            b
        }
    }

    #[doc(alias = "graphene_box_init_from_points")]
    #[doc(alias = "new_from_points")]
    pub fn from_points(&mut self, points: &[Point3D]) -> Box {
        assert_initialized_main_thread!();

        let n = points.len() as u32;

        unsafe {
            let mut b = Box::uninitialized();
            ffi::graphene_box_init_from_points(b.to_glib_none_mut().0, n, points.to_glib_none().0);
            b
        }
    }

    #[doc(alias = "graphene_box_init_from_vec3")]
    #[doc(alias = "new_from_vec3")]
    pub fn from_vec3(min: Option<&Vec3>, max: Option<&Vec3>) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            let mut b = Box::uninitialized();
            ffi::graphene_box_init_from_vec3(
                b.to_glib_none_mut().0,
                min.to_glib_none().0,
                max.to_glib_none().0,
            );
            b
        }
    }

    #[doc(alias = "graphene_box_init_from_vectors")]
    #[doc(alias = "new_from_vectors")]
    pub fn from_vectors(vectors: &[Vec3]) -> Box {
        assert_initialized_main_thread!();

        let n = vectors.len() as u32;

        unsafe {
            let mut b = Box::uninitialized();
            ffi::graphene_box_init_from_vectors(
                b.to_glib_none_mut().0,
                n,
                vectors.to_glib_none().0,
            );
            b
        }
    }
}
