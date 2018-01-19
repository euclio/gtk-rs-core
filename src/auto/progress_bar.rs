// This file was generated by gir (c6b70b0) from gir-files (469db10)
// DO NOT EDIT

use Buildable;
use Orientable;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use pango;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ProgressBar(Object<ffi::GtkProgressBar, ffi::GtkProgressBarClass>): Widget, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_progress_bar_get_type(),
    }
}

impl ProgressBar {
    pub fn new() -> ProgressBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_progress_bar_new()).downcast_unchecked()
        }
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ProgressBarExt {
    fn get_ellipsize(&self) -> pango::EllipsizeMode;

    fn get_fraction(&self) -> f64;

    fn get_inverted(&self) -> bool;

    fn get_pulse_step(&self) -> f64;

    fn get_show_text(&self) -> bool;

    fn get_text(&self) -> Option<String>;

    fn pulse(&self);

    fn set_ellipsize(&self, mode: pango::EllipsizeMode);

    fn set_fraction(&self, fraction: f64);

    fn set_inverted(&self, inverted: bool);

    fn set_pulse_step(&self, fraction: f64);

    fn set_show_text(&self, show_text: bool);

    fn set_text<'a, P: Into<Option<&'a str>>>(&self, text: P);

    fn connect_property_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fraction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pulse_step_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ProgressBar> + IsA<glib::object::Object>> ProgressBarExt for O {
    fn get_ellipsize(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(ffi::gtk_progress_bar_get_ellipsize(self.to_glib_none().0))
        }
    }

    fn get_fraction(&self) -> f64 {
        unsafe {
            ffi::gtk_progress_bar_get_fraction(self.to_glib_none().0)
        }
    }

    fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_progress_bar_get_inverted(self.to_glib_none().0))
        }
    }

    fn get_pulse_step(&self) -> f64 {
        unsafe {
            ffi::gtk_progress_bar_get_pulse_step(self.to_glib_none().0)
        }
    }

    fn get_show_text(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_progress_bar_get_show_text(self.to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_progress_bar_get_text(self.to_glib_none().0))
        }
    }

    fn pulse(&self) {
        unsafe {
            ffi::gtk_progress_bar_pulse(self.to_glib_none().0);
        }
    }

    fn set_ellipsize(&self, mode: pango::EllipsizeMode) {
        unsafe {
            ffi::gtk_progress_bar_set_ellipsize(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn set_fraction(&self, fraction: f64) {
        unsafe {
            ffi::gtk_progress_bar_set_fraction(self.to_glib_none().0, fraction);
        }
    }

    fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::gtk_progress_bar_set_inverted(self.to_glib_none().0, inverted.to_glib());
        }
    }

    fn set_pulse_step(&self, fraction: f64) {
        unsafe {
            ffi::gtk_progress_bar_set_pulse_step(self.to_glib_none().0, fraction);
        }
    }

    fn set_show_text(&self, show_text: bool) {
        unsafe {
            ffi::gtk_progress_bar_set_show_text(self.to_glib_none().0, show_text.to_glib());
        }
    }

    fn set_text<'a, P: Into<Option<&'a str>>>(&self, text: P) {
        let text = text.into();
        let text = text.to_glib_none();
        unsafe {
            ffi::gtk_progress_bar_set_text(self.to_glib_none().0, text.0);
        }
    }

    fn connect_property_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::ellipsize",
                transmute(notify_ellipsize_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_fraction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::fraction",
                transmute(notify_fraction_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::inverted",
                transmute(notify_inverted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pulse_step_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pulse-step",
                transmute(notify_pulse_step_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-text",
                transmute(notify_show_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text",
                transmute(notify_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_ellipsize_trampoline<P>(this: *mut ffi::GtkProgressBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProgressBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ProgressBar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_fraction_trampoline<P>(this: *mut ffi::GtkProgressBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProgressBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ProgressBar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_inverted_trampoline<P>(this: *mut ffi::GtkProgressBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProgressBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ProgressBar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pulse_step_trampoline<P>(this: *mut ffi::GtkProgressBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProgressBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ProgressBar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_text_trampoline<P>(this: *mut ffi::GtkProgressBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProgressBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ProgressBar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_trampoline<P>(this: *mut ffi::GtkProgressBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProgressBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ProgressBar::from_glib_borrow(this).downcast_unchecked())
}
