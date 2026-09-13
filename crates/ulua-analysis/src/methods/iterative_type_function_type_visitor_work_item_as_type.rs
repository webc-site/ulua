use crate::{
  records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn work_item_as_type(&self) -> Option<*const TypeFunctionTypeId> {
    let work_item = self.work_queue[self.work_cursor as usize].clone();
    if work_item.is_type {
      Some(work_item.as_type() as *const TypeFunctionTypeId)
    } else {
      None
    }
  }
}
