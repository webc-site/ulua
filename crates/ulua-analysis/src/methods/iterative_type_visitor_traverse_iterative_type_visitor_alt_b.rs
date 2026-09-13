use crate::{
  records::{
    iterative_type_visitor::IterativeTypeVisitor, work_item_iterative_type_visitor::WorkItem,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl IterativeTypeVisitor {
  pub fn traverse_type_pack_id(&mut self, tp: TypePackId) {
    self
      .work_queue
      .push(WorkItem::work_item_type_pack_id_i32(tp, self.parent_cursor));
  }
}
