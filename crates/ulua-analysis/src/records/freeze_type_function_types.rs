use alloc::string::String;

use crate::{
  records::{
    iterative_type_function_type_visitor,
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    type_function_type::TypeFunctionType,
  },
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};
#[derive(Debug, Clone)]
pub struct FreezeTypeFunctionTypes {
  pub base: IterativeTypeFunctionTypeVisitor,
}

impl FreezeTypeFunctionTypes {
  pub fn new() -> Self {
    Self {
      base: IterativeTypeFunctionTypeVisitor::iterative_type_function_type_visitor_string(
        String::from("FreezeTypeFunctionTypes"),
      ),
    }
  }
}

impl Default for FreezeTypeFunctionTypes {
  fn default() -> Self {
    Self::new()
  }
}

impl iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor {
  pub fn visit_type_function_type_id(&mut self, ty: TypeFunctionTypeId) -> bool {
    unsafe {
      (*(ty as *mut TypeFunctionType)).frozen = true;
    }
    true
  }
}
