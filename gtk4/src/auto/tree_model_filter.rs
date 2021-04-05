// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::TreeDragSource;
use crate::TreeIter;
use crate::TreeModel;
use crate::TreePath;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;

glib::wrapper! {
    pub struct TreeModelFilter(Object<ffi::GtkTreeModelFilter, ffi::GtkTreeModelFilterClass>) @implements TreeDragSource, TreeModel;

    match fn {
        get_type => || ffi::gtk_tree_model_filter_get_type(),
    }
}

#[derive(Clone, Default)]
pub struct TreeModelFilterBuilder {
    child_model: Option<TreeModel>,
}

impl TreeModelFilterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> TreeModelFilter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref child_model) = self.child_model {
            properties.push(("child-model", child_model));
        }
        let ret = glib::Object::new::<TreeModelFilter>(&properties).expect("object new");
        ret
    }

    pub fn child_model<P: IsA<TreeModel>>(mut self, child_model: &P) -> Self {
        self.child_model = Some(child_model.clone().upcast());
        self
    }
}

pub const NONE_TREE_MODEL_FILTER: Option<&TreeModelFilter> = None;

pub trait TreeModelFilterExt: 'static {
    #[doc(alias = "gtk_tree_model_filter_clear_cache")]
    fn clear_cache(&self);

    #[doc(alias = "gtk_tree_model_filter_convert_child_iter_to_iter")]
    fn convert_child_iter_to_iter(&self, child_iter: &TreeIter) -> Option<TreeIter>;

    #[doc(alias = "gtk_tree_model_filter_convert_child_path_to_path")]
    fn convert_child_path_to_path(&self, child_path: &TreePath) -> Option<TreePath>;

    #[doc(alias = "gtk_tree_model_filter_convert_iter_to_child_iter")]
    fn convert_iter_to_child_iter(&self, filter_iter: &TreeIter) -> TreeIter;

    #[doc(alias = "gtk_tree_model_filter_convert_path_to_child_path")]
    fn convert_path_to_child_path(&self, filter_path: &TreePath) -> Option<TreePath>;

    #[doc(alias = "gtk_tree_model_filter_get_model")]
    fn get_model(&self) -> Option<TreeModel>;

    #[doc(alias = "gtk_tree_model_filter_refilter")]
    fn refilter(&self);

    #[doc(alias = "gtk_tree_model_filter_set_visible_column")]
    fn set_visible_column(&self, column: i32);

    #[doc(alias = "gtk_tree_model_filter_set_visible_func")]
    fn set_visible_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(&self, func: P);

    fn get_property_child_model(&self) -> Option<TreeModel>;
}

impl<O: IsA<TreeModelFilter>> TreeModelFilterExt for O {
    fn clear_cache(&self) {
        unsafe {
            ffi::gtk_tree_model_filter_clear_cache(self.as_ref().to_glib_none().0);
        }
    }

    fn convert_child_iter_to_iter(&self, child_iter: &TreeIter) -> Option<TreeIter> {
        unsafe {
            let mut filter_iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_filter_convert_child_iter_to_iter(
                self.as_ref().to_glib_none().0,
                filter_iter.to_glib_none_mut().0,
                mut_override(child_iter.to_glib_none().0),
            ));
            if ret {
                Some(filter_iter)
            } else {
                None
            }
        }
    }

    fn convert_child_path_to_path(&self, child_path: &TreePath) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_filter_convert_child_path_to_path(
                self.as_ref().to_glib_none().0,
                mut_override(child_path.to_glib_none().0),
            ))
        }
    }

    fn convert_iter_to_child_iter(&self, filter_iter: &TreeIter) -> TreeIter {
        unsafe {
            let mut child_iter = TreeIter::uninitialized();
            ffi::gtk_tree_model_filter_convert_iter_to_child_iter(
                self.as_ref().to_glib_none().0,
                child_iter.to_glib_none_mut().0,
                mut_override(filter_iter.to_glib_none().0),
            );
            child_iter
        }
    }

    fn convert_path_to_child_path(&self, filter_path: &TreePath) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_filter_convert_path_to_child_path(
                self.as_ref().to_glib_none().0,
                mut_override(filter_path.to_glib_none().0),
            ))
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_tree_model_filter_get_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn refilter(&self) {
        unsafe {
            ffi::gtk_tree_model_filter_refilter(self.as_ref().to_glib_none().0);
        }
    }

    fn set_visible_column(&self, column: i32) {
        unsafe {
            ffi::gtk_tree_model_filter_set_visible_column(self.as_ref().to_glib_none().0, column);
        }
    }

    fn set_visible_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(&self, func: P) {
        let func_data: Box_<P> = Box_::new(func);
        unsafe extern "C" fn func_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(
            model: *mut ffi::GtkTreeModel,
            iter: *mut ffi::GtkTreeIter,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let model = from_glib_borrow(model);
            let iter = from_glib_borrow(iter);
            let callback: &P = &*(data as *mut _);
            let res = (*callback)(&model, &iter);
            res.to_glib()
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            ffi::gtk_tree_model_filter_set_visible_func(
                self.as_ref().to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn get_property_child_model(&self) -> Option<TreeModel> {
        unsafe {
            let mut value = glib::Value::from_type(<TreeModel as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"child-model\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `child-model` getter")
        }
    }
}

impl fmt::Display for TreeModelFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeModelFilter")
    }
}
