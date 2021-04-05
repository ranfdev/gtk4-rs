// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::TreePath;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct TreeDragSource(Interface<ffi::GtkTreeDragSource, ffi::GtkTreeDragSourceIface>);

    match fn {
        get_type => || ffi::gtk_tree_drag_source_get_type(),
    }
}

pub const NONE_TREE_DRAG_SOURCE: Option<&TreeDragSource> = None;

pub trait TreeDragSourceExt: 'static {
    #[doc(alias = "gtk_tree_drag_source_drag_data_delete")]
    fn drag_data_delete(&self, path: &mut TreePath) -> bool;

    #[doc(alias = "gtk_tree_drag_source_drag_data_get")]
    fn drag_data_get(&self, path: &mut TreePath) -> Option<gdk::ContentProvider>;

    #[doc(alias = "gtk_tree_drag_source_row_draggable")]
    fn row_draggable(&self, path: &mut TreePath) -> bool;
}

impl<O: IsA<TreeDragSource>> TreeDragSourceExt for O {
    fn drag_data_delete(&self, path: &mut TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_source_drag_data_delete(
                self.as_ref().to_glib_none().0,
                path.to_glib_none_mut().0,
            ))
        }
    }

    fn drag_data_get(&self, path: &mut TreePath) -> Option<gdk::ContentProvider> {
        unsafe {
            from_glib_full(ffi::gtk_tree_drag_source_drag_data_get(
                self.as_ref().to_glib_none().0,
                path.to_glib_none_mut().0,
            ))
        }
    }

    fn row_draggable(&self, path: &mut TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_source_row_draggable(
                self.as_ref().to_glib_none().0,
                path.to_glib_none_mut().0,
            ))
        }
    }
}

impl fmt::Display for TreeDragSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeDragSource")
    }
}
