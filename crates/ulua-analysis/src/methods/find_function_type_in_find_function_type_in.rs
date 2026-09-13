use alloc::{string::String, vec::Vec};
use core::ptr::{null, null_mut};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{
  find_function_type_in::FindFunctionTypeIn, iterative_type_visitor::IterativeTypeVisitor,
};
impl FindFunctionTypeIn {
  pub fn new(number_of_lambda_parameters: i32) -> Self {
    let mut visitor = FindFunctionTypeIn {
      base: IterativeTypeVisitor {
        seen: DenseHashSet::new(null_mut()),
        work_queue: Vec::new(),
        parent_cursor: -1,
        work_cursor: 0,
        visitor_name: String::from("FindFunctionTypeIn"),
        skip_bound_types: true,
        visit_once: true,
      },
      number_of_lambda_parameters,
      candidate: null(),
    };
    visitor
      .base
      .iterative_type_visitor_string_bool_bool("FindFunctionTypeIn", true, true);
    visitor
  }
}
