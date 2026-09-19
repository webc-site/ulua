use alloc::{string::String, vec::Vec};
use core::{ffi::c_void, ptr::null_mut};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{
  contains_generics::ContainsGenerics, iterative_type_visitor::IterativeTypeVisitor,
};
impl ContainsGenerics {
  pub fn contains_generics_contains_generics(generics: *mut DenseHashSet<*const c_void>) -> Self {
    ContainsGenerics {
      base: IterativeTypeVisitor {
        seen: DenseHashSet::new(null_mut()),
        work_queue: Vec::new(),
        parent_cursor: -1,
        work_cursor: 0,
        visitor_name: String::from("ContainsGenerics"),
        skip_bound_types: true,
        visit_once: true,
      },
      generics,
      found: false,
    }
  }
}
