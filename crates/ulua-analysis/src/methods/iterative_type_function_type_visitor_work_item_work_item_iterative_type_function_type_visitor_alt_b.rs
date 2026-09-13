use core::ffi::c_void;

use crate::{
  records::{
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    work_item_iterative_type_function_type_visitor::WorkItem,
  },
  type_aliases::type_function_type_pack_id::TypeFunctionTypePackId,
};
impl IterativeTypeFunctionTypeVisitor {
  pub fn work_item_type_function_type_pack_id_i32(
    tp: TypeFunctionTypePackId,
    parent: i32,
  ) -> WorkItem {
    WorkItem {
      t: tp as *const c_void,
      is_type: false,
      parent,
    }
  }
}
