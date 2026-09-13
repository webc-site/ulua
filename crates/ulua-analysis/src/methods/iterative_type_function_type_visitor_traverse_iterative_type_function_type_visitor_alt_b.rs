use crate::{
  records::{
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    work_item_iterative_type_function_type_visitor::WorkItem,
  },
  type_aliases::type_function_type_pack_id::TypeFunctionTypePackId,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn traverse_type_function_type_pack_id(&mut self, tp: TypeFunctionTypePackId) {
    self
      .work_queue
      .push(WorkItem::work_item_type_function_type_pack_id_i32(
        tp,
        self.parent_cursor,
      ));
  }
}
