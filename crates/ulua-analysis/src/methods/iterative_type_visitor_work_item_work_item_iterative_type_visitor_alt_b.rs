use crate::{
  records::iterative_type_visitor::IterativeTypeVisitor, type_aliases::type_pack_id::TypePackId,
};

impl IterativeTypeVisitor {
  pub fn work_item_type_pack_id_i32(&mut self, _tp: TypePackId, _parent: i32) {
    // Empty constructor body per source: IterativeTypeVisitor::WorkItem::WorkItem(TypePackId tp, int32_t parent)
  }
}
