use core::ptr::null_mut;

use crate::{
  records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
  type_aliases::type_function_type_pack_id::TypeFunctionTypePackId,
};
impl IterativeTypeFunctionTypeVisitor {
  pub fn work_item_as_type_pack(&self) -> Option<*const TypeFunctionTypePackId> {
    Some(null_mut())
  }
}
