// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ShortcutAction;
use crate::Widget;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;

glib::wrapper! {
    pub struct CallbackAction(Object<ffi::GtkCallbackAction, ffi::GtkCallbackActionClass>) @extends ShortcutAction;

    match fn {
        get_type => || ffi::gtk_callback_action_get_type(),
    }
}

impl CallbackAction {
    #[doc(alias = "gtk_callback_action_new")]
    pub fn new(
        callback: Option<Box_<dyn Fn(&Widget, Option<&glib::Variant>) -> bool + 'static>>,
    ) -> CallbackAction {
        assert_initialized_main_thread!();
        let callback_data: Box_<
            Option<Box_<dyn Fn(&Widget, Option<&glib::Variant>) -> bool + 'static>>,
        > = Box_::new(callback);
        unsafe extern "C" fn callback_func(
            widget: *mut ffi::GtkWidget,
            args: *mut glib::ffi::GVariant,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let widget = from_glib_borrow(widget);
            let args: Borrowed<Option<glib::Variant>> = from_glib_borrow(args);
            let callback: &Option<Box_<dyn Fn(&Widget, Option<&glib::Variant>) -> bool + 'static>> =
                &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&widget, args.as_ref().as_ref())
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let callback = if callback_data.is_some() {
            Some(callback_func as _)
        } else {
            None
        };
        unsafe extern "C" fn destroy_func(data: glib::ffi::gpointer) {
            let _callback: Box_<
                Option<Box_<dyn Fn(&Widget, Option<&glib::Variant>) -> bool + 'static>>,
            > = Box_::from_raw(data as *mut _);
        }
        let destroy_call2 = Some(destroy_func as _);
        let super_callback0: Box_<
            Option<Box_<dyn Fn(&Widget, Option<&glib::Variant>) -> bool + 'static>>,
        > = callback_data;
        unsafe {
            from_glib_full(ffi::gtk_callback_action_new(
                callback,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call2,
            ))
        }
    }
}

impl fmt::Display for CallbackAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CallbackAction")
    }
}
