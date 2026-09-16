use core::ffi::c_void;

use crate::{
  records::{
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    work_item_iterative_type_function_type_visitor::WorkItem,
  },
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};
impl IterativeTypeFunctionTypeVisitor {
  pub fn work_item_type_function_type_id_i32(ty: TypeFunctionTypeId, parent: i32) -> WorkItem {
    WorkItem {
      t: ty as *const c_void,
      is_type: true,
      parent,
    }
  }
}
