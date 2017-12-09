// This file was generated by gir (8080733) from gir-files (469db10)
// DO NOT EDIT

use AssistantPageType;
use Bin;
use Buildable;
use Container;
use Widget;
use Window;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Assistant(Object<ffi::GtkAssistant, ffi::GtkAssistantClass>): Window, Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_assistant_get_type(),
    }
}

impl Assistant {
    pub fn new() -> Assistant {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_assistant_new()).downcast_unchecked()
        }
    }
}

impl Default for Assistant {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AssistantExt {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P);

    fn append_page<P: IsA<Widget>>(&self, page: &P) -> i32;

    fn commit(&self);

    fn get_current_page(&self) -> i32;

    fn get_n_pages(&self) -> i32;

    fn get_nth_page(&self, page_num: i32) -> Option<Widget>;

    fn get_page_complete<P: IsA<Widget>>(&self, page: &P) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_page_has_padding<P: IsA<Widget>>(&self, page: &P) -> bool;

    fn get_page_title<P: IsA<Widget>>(&self, page: &P) -> Option<String>;

    fn get_page_type<P: IsA<Widget>>(&self, page: &P) -> AssistantPageType;

    fn insert_page<P: IsA<Widget>>(&self, page: &P, position: i32) -> i32;

    fn next_page(&self);

    fn prepend_page<P: IsA<Widget>>(&self, page: &P) -> i32;

    fn previous_page(&self);

    fn remove_action_widget<P: IsA<Widget>>(&self, child: &P);

    fn remove_page(&self, page_num: i32);

    fn set_current_page(&self, page_num: i32);

    //fn set_forward_page_func<'a, P: Into<Option<&'a /*Unimplemented*/AssistantPageFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, page_func: P, data: Q, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_page_complete<P: IsA<Widget>>(&self, page: &P, complete: bool);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_page_has_padding<P: IsA<Widget>>(&self, page: &P, has_padding: bool);

    fn set_page_title<P: IsA<Widget>>(&self, page: &P, title: &str);

    fn set_page_type<P: IsA<Widget>>(&self, page: &P, type_: AssistantPageType);

    fn update_buttons_state(&self);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_property_use_header_bar(&self) -> i32;

    fn get_child_complete<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_complete<T: IsA<Widget>>(&self, item: &T, complete: bool);

    fn get_child_has_padding<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_has_padding<T: IsA<Widget>>(&self, item: &T, has_padding: bool);

    fn get_child_page_type<T: IsA<Widget>>(&self, item: &T) -> AssistantPageType;

    fn set_child_page_type<T: IsA<Widget>>(&self, item: &T, page_type: AssistantPageType);

    fn get_child_title<T: IsA<Widget>>(&self, item: &T) -> Option<String>;

    fn set_child_title<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, title: P);

    fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_escape<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_escape(&self);

    fn connect_prepare<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_use_header_bar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Assistant> + IsA<Container> + IsA<glib::object::Object> + glib::object::ObjectExt> AssistantExt for O {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_assistant_add_action_widget(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    fn append_page<P: IsA<Widget>>(&self, page: &P) -> i32 {
        unsafe {
            ffi::gtk_assistant_append_page(self.to_glib_none().0, page.to_glib_none().0)
        }
    }

    fn commit(&self) {
        unsafe {
            ffi::gtk_assistant_commit(self.to_glib_none().0);
        }
    }

    fn get_current_page(&self) -> i32 {
        unsafe {
            ffi::gtk_assistant_get_current_page(self.to_glib_none().0)
        }
    }

    fn get_n_pages(&self) -> i32 {
        unsafe {
            ffi::gtk_assistant_get_n_pages(self.to_glib_none().0)
        }
    }

    fn get_nth_page(&self, page_num: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_assistant_get_nth_page(self.to_glib_none().0, page_num))
        }
    }

    fn get_page_complete<P: IsA<Widget>>(&self, page: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_assistant_get_page_complete(self.to_glib_none().0, page.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_page_has_padding<P: IsA<Widget>>(&self, page: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_assistant_get_page_has_padding(self.to_glib_none().0, page.to_glib_none().0))
        }
    }

    fn get_page_title<P: IsA<Widget>>(&self, page: &P) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_assistant_get_page_title(self.to_glib_none().0, page.to_glib_none().0))
        }
    }

    fn get_page_type<P: IsA<Widget>>(&self, page: &P) -> AssistantPageType {
        unsafe {
            from_glib(ffi::gtk_assistant_get_page_type(self.to_glib_none().0, page.to_glib_none().0))
        }
    }

    fn insert_page<P: IsA<Widget>>(&self, page: &P, position: i32) -> i32 {
        unsafe {
            ffi::gtk_assistant_insert_page(self.to_glib_none().0, page.to_glib_none().0, position)
        }
    }

    fn next_page(&self) {
        unsafe {
            ffi::gtk_assistant_next_page(self.to_glib_none().0);
        }
    }

    fn prepend_page<P: IsA<Widget>>(&self, page: &P) -> i32 {
        unsafe {
            ffi::gtk_assistant_prepend_page(self.to_glib_none().0, page.to_glib_none().0)
        }
    }

    fn previous_page(&self) {
        unsafe {
            ffi::gtk_assistant_previous_page(self.to_glib_none().0);
        }
    }

    fn remove_action_widget<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_assistant_remove_action_widget(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    fn remove_page(&self, page_num: i32) {
        unsafe {
            ffi::gtk_assistant_remove_page(self.to_glib_none().0, page_num);
        }
    }

    fn set_current_page(&self, page_num: i32) {
        unsafe {
            ffi::gtk_assistant_set_current_page(self.to_glib_none().0, page_num);
        }
    }

    //fn set_forward_page_func<'a, P: Into<Option<&'a /*Unimplemented*/AssistantPageFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, page_func: P, data: Q, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_assistant_set_forward_page_func() }
    //}

    fn set_page_complete<P: IsA<Widget>>(&self, page: &P, complete: bool) {
        unsafe {
            ffi::gtk_assistant_set_page_complete(self.to_glib_none().0, page.to_glib_none().0, complete.to_glib());
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_page_has_padding<P: IsA<Widget>>(&self, page: &P, has_padding: bool) {
        unsafe {
            ffi::gtk_assistant_set_page_has_padding(self.to_glib_none().0, page.to_glib_none().0, has_padding.to_glib());
        }
    }

    fn set_page_title<P: IsA<Widget>>(&self, page: &P, title: &str) {
        unsafe {
            ffi::gtk_assistant_set_page_title(self.to_glib_none().0, page.to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_page_type<P: IsA<Widget>>(&self, page: &P, type_: AssistantPageType) {
        unsafe {
            ffi::gtk_assistant_set_page_type(self.to_glib_none().0, page.to_glib_none().0, type_.to_glib());
        }
    }

    fn update_buttons_state(&self) {
        unsafe {
            ffi::gtk_assistant_update_buttons_state(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_property_use_header_bar(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "use-header-bar".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_child_complete<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "complete".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_complete<T: IsA<Widget>>(&self, item: &T, complete: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "complete".to_glib_none().0, Value::from(&complete).to_glib_none().0);
        }
    }

    fn get_child_has_padding<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "has-padding".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_has_padding<T: IsA<Widget>>(&self, item: &T, has_padding: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "has-padding".to_glib_none().0, Value::from(&has_padding).to_glib_none().0);
        }
    }

    fn get_child_page_type<T: IsA<Widget>>(&self, item: &T) -> AssistantPageType {
        unsafe {
            let mut value = Value::from_type(<AssistantPageType as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "page-type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_page_type<T: IsA<Widget>>(&self, item: &T, page_type: AssistantPageType) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "page-type".to_glib_none().0, Value::from(&page_type).to_glib_none().0);
        }
    }

    fn get_child_title<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "title".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_child_title<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, title: P) {
        let title = title.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "title".to_glib_none().0, Value::from(title).to_glib_none().0);
        }
    }

    fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "apply",
                transmute(apply_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancel",
                transmute(cancel_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "close",
                transmute(close_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_escape<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "escape",
                transmute(escape_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_escape(&self) {
        let _ = self.emit("escape", &[]).unwrap();
    }

    fn connect_prepare<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "prepare",
                transmute(prepare_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_use_header_bar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-header-bar",
                transmute(notify_use_header_bar_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn apply_trampoline<P>(this: *mut ffi::GtkAssistant, f: glib_ffi::gpointer)
where P: IsA<Assistant> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Assistant::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn cancel_trampoline<P>(this: *mut ffi::GtkAssistant, f: glib_ffi::gpointer)
where P: IsA<Assistant> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Assistant::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn close_trampoline<P>(this: *mut ffi::GtkAssistant, f: glib_ffi::gpointer)
where P: IsA<Assistant> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Assistant::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn escape_trampoline<P>(this: *mut ffi::GtkAssistant, f: glib_ffi::gpointer)
where P: IsA<Assistant> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Assistant::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn prepare_trampoline<P>(this: *mut ffi::GtkAssistant, page: *mut ffi::GtkWidget, f: glib_ffi::gpointer)
where P: IsA<Assistant> {
    callback_guard!();
    let f: &&(Fn(&P, &Widget) + 'static) = transmute(f);
    f(&Assistant::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(page))
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
unsafe extern "C" fn notify_use_header_bar_trampoline<P>(this: *mut ffi::GtkAssistant, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Assistant> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Assistant::from_glib_borrow(this).downcast_unchecked())
}
