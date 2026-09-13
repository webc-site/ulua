use crate::{
  records::{
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    work_item_iterative_type_function_type_visitor::WorkItem,
  },
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn traverse_type_function_type_id(&mut self, ty: TypeFunctionTypeId) {
    self
      .work_queue
      .push(WorkItem::work_item_type_function_type_id_i32(
        ty,
        self.parent_cursor,
      ));
  }
}
