use crate::{
  records::{
    iterative_type_visitor::IterativeTypeVisitor, work_item_iterative_type_visitor::WorkItem,
  },
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn traverse_type_id(&mut self, ty: TypeId) {
    self
      .work_queue
      .push(WorkItem::work_item_type_id_i32(ty, self.parent_cursor));
  }
}
