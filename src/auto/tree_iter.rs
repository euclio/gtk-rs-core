// This file was generated by gir (6bcd52a) from gir-files (1069259)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TreeIter(Boxed<ffi::GtkTreeIter>);

    match fn {
        copy => |ptr| ffi::gtk_tree_iter_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_tree_iter_free(ptr),
        get_type => || ffi::gtk_tree_iter_get_type(),
    }
}
