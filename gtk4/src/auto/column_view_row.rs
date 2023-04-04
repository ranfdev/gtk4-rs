// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkColumnViewRow")]
    pub struct ColumnViewRow(Object<ffi::GtkColumnViewRow, ffi::GtkColumnViewRowClass>);

    match fn {
        type_ => || ffi::gtk_column_view_row_get_type(),
    }
}

impl ColumnViewRow {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ColumnViewRow`] objects.
    ///
    /// This method returns an instance of [`ColumnViewRowBuilder`](crate::builders::ColumnViewRowBuilder) which can be used to create [`ColumnViewRow`] objects.
    pub fn builder() -> ColumnViewRowBuilder {
        ColumnViewRowBuilder::new()
    }

    #[doc(alias = "gtk_column_view_row_get_activatable")]
    #[doc(alias = "get_activatable")]
    pub fn is_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_row_get_activatable(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_column_view_row_get_focusable")]
    #[doc(alias = "get_focusable")]
    pub fn is_focusable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_row_get_focusable(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_column_view_row_get_item")]
    #[doc(alias = "get_item")]
    pub fn item(&self) -> Option<glib::Object> {
        unsafe { from_glib_none(ffi::gtk_column_view_row_get_item(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_column_view_row_get_position")]
    #[doc(alias = "get_position")]
    pub fn position(&self) -> u32 {
        unsafe { ffi::gtk_column_view_row_get_position(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_column_view_row_get_selectable")]
    #[doc(alias = "get_selectable")]
    pub fn is_selectable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_row_get_selectable(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_column_view_row_get_selected")]
    #[doc(alias = "get_selected")]
    pub fn is_selected(&self) -> bool {
        unsafe { from_glib(ffi::gtk_column_view_row_get_selected(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_column_view_row_set_activatable")]
    pub fn set_activatable(&self, activatable: bool) {
        unsafe {
            ffi::gtk_column_view_row_set_activatable(
                self.to_glib_none().0,
                activatable.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_column_view_row_set_focusable")]
    pub fn set_focusable(&self, focusable: bool) {
        unsafe {
            ffi::gtk_column_view_row_set_focusable(self.to_glib_none().0, focusable.into_glib());
        }
    }

    #[doc(alias = "gtk_column_view_row_set_selectable")]
    pub fn set_selectable(&self, selectable: bool) {
        unsafe {
            ffi::gtk_column_view_row_set_selectable(self.to_glib_none().0, selectable.into_glib());
        }
    }

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    #[doc(alias = "activatable")]
    pub fn connect_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_activatable_trampoline<F: Fn(&ColumnViewRow) + 'static>(
            this: *mut ffi::GtkColumnViewRow,
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
                b"notify::activatable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activatable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    #[doc(alias = "focusable")]
    pub fn connect_focusable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_focusable_trampoline<F: Fn(&ColumnViewRow) + 'static>(
            this: *mut ffi::GtkColumnViewRow,
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
                b"notify::focusable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_focusable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    #[doc(alias = "item")]
    pub fn connect_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_item_trampoline<F: Fn(&ColumnViewRow) + 'static>(
            this: *mut ffi::GtkColumnViewRow,
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
                b"notify::item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_item_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    #[doc(alias = "position")]
    pub fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<F: Fn(&ColumnViewRow) + 'static>(
            this: *mut ffi::GtkColumnViewRow,
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
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    #[doc(alias = "selectable")]
    pub fn connect_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selectable_trampoline<F: Fn(&ColumnViewRow) + 'static>(
            this: *mut ffi::GtkColumnViewRow,
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
                b"notify::selectable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selectable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    #[doc(alias = "selected")]
    pub fn connect_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_trampoline<F: Fn(&ColumnViewRow) + 'static>(
            this: *mut ffi::GtkColumnViewRow,
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
                b"notify::selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ColumnViewRow`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ColumnViewRowBuilder {
    builder: glib::object::ObjectBuilder<'static, ColumnViewRow>,
}

impl ColumnViewRowBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    pub fn activatable(self, activatable: bool) -> Self {
        Self {
            builder: self.builder.property("activatable", activatable),
        }
    }

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    pub fn selectable(self, selectable: bool) -> Self {
        Self {
            builder: self.builder.property("selectable", selectable),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ColumnViewRow`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ColumnViewRow {
        self.builder.build()
    }
}

impl fmt::Display for ColumnViewRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ColumnViewRow")
    }
}
