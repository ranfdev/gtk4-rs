// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::LayoutManager;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct OverlayLayout(Object<ffi::GtkOverlayLayout, ffi::GtkOverlayLayoutClass>) @extends LayoutManager;

    match fn {
        get_type => || ffi::gtk_overlay_layout_get_type(),
    }
}

impl OverlayLayout {
    #[doc(alias = "gtk_overlay_layout_new")]
    pub fn new() -> OverlayLayout {
        assert_initialized_main_thread!();
        unsafe { LayoutManager::from_glib_full(ffi::gtk_overlay_layout_new()).unsafe_cast() }
    }
}

impl Default for OverlayLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for OverlayLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("OverlayLayout")
    }
}
