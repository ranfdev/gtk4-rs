// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Window;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct MountOperation(Object<ffi::GtkMountOperation, ffi::GtkMountOperationClass>) @extends gio::MountOperation;

    match fn {
        get_type => || ffi::gtk_mount_operation_get_type(),
    }
}

impl MountOperation {
    #[doc(alias = "gtk_mount_operation_new")]
    pub fn new<P: IsA<Window>>(parent: Option<&P>) -> MountOperation {
        assert_initialized_main_thread!();
        unsafe {
            gio::MountOperation::from_glib_full(ffi::gtk_mount_operation_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

#[derive(Clone, Default)]
pub struct MountOperationBuilder {
    display: Option<gdk::Display>,
    parent: Option<Window>,
    anonymous: Option<bool>,
    choice: Option<i32>,
    domain: Option<String>,
    is_tcrypt_hidden_volume: Option<bool>,
    is_tcrypt_system_volume: Option<bool>,
    password: Option<String>,
    //password-save: /*Unknown type*/,
    pim: Option<u32>,
    username: Option<String>,
}

impl MountOperationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> MountOperation {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref anonymous) = self.anonymous {
            properties.push(("anonymous", anonymous));
        }
        if let Some(ref choice) = self.choice {
            properties.push(("choice", choice));
        }
        if let Some(ref domain) = self.domain {
            properties.push(("domain", domain));
        }
        if let Some(ref is_tcrypt_hidden_volume) = self.is_tcrypt_hidden_volume {
            properties.push(("is-tcrypt-hidden-volume", is_tcrypt_hidden_volume));
        }
        if let Some(ref is_tcrypt_system_volume) = self.is_tcrypt_system_volume {
            properties.push(("is-tcrypt-system-volume", is_tcrypt_system_volume));
        }
        if let Some(ref password) = self.password {
            properties.push(("password", password));
        }
        if let Some(ref pim) = self.pim {
            properties.push(("pim", pim));
        }
        if let Some(ref username) = self.username {
            properties.push(("username", username));
        }
        let ret = glib::Object::new::<MountOperation>(&properties).expect("object new");
        ret
    }

    pub fn display(mut self, display: &gdk::Display) -> Self {
        self.display = Some(display.clone());
        self
    }

    pub fn parent<P: IsA<Window>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn anonymous(mut self, anonymous: bool) -> Self {
        self.anonymous = Some(anonymous);
        self
    }

    pub fn choice(mut self, choice: i32) -> Self {
        self.choice = Some(choice);
        self
    }

    pub fn domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    pub fn is_tcrypt_hidden_volume(mut self, is_tcrypt_hidden_volume: bool) -> Self {
        self.is_tcrypt_hidden_volume = Some(is_tcrypt_hidden_volume);
        self
    }

    pub fn is_tcrypt_system_volume(mut self, is_tcrypt_system_volume: bool) -> Self {
        self.is_tcrypt_system_volume = Some(is_tcrypt_system_volume);
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }

    pub fn pim(mut self, pim: u32) -> Self {
        self.pim = Some(pim);
        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }
}

pub const NONE_MOUNT_OPERATION: Option<&MountOperation> = None;

pub trait MountOperationExt: 'static {
    #[doc(alias = "gtk_mount_operation_get_display")]
    fn get_display(&self) -> Option<gdk::Display>;

    #[doc(alias = "gtk_mount_operation_get_parent")]
    fn get_parent(&self) -> Option<Window>;

    #[doc(alias = "gtk_mount_operation_is_showing")]
    fn is_showing(&self) -> bool;

    #[doc(alias = "gtk_mount_operation_set_display")]
    fn set_display(&self, display: &gdk::Display);

    #[doc(alias = "gtk_mount_operation_set_parent")]
    fn set_parent<P: IsA<Window>>(&self, parent: Option<&P>);

    fn get_property_is_showing(&self) -> bool;

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_is_showing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MountOperation>> MountOperationExt for O {
    fn get_display(&self) -> Option<gdk::Display> {
        unsafe {
            from_glib_none(ffi::gtk_mount_operation_get_display(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_parent(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_mount_operation_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_showing(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_mount_operation_is_showing(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_display(&self, display: &gdk::Display) {
        unsafe {
            ffi::gtk_mount_operation_set_display(
                self.as_ref().to_glib_none().0,
                display.to_glib_none().0,
            );
        }
    }

    fn set_parent<P: IsA<Window>>(&self, parent: Option<&P>) {
        unsafe {
            ffi::gtk_mount_operation_set_parent(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn get_property_is_showing(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"is-showing\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `is-showing` getter")
                .unwrap()
        }
    }

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::display\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_display_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_showing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_showing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-showing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_showing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MountOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MountOperation")
    }
}
