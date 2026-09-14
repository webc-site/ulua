use crate::{records::iterative_type_visitor::IterativeTypeVisitor, type_aliases::type_id::TypeId};

impl IterativeTypeVisitor {
  pub fn work_item_type_id_i32(&mut self, _ty: TypeId, _parent: i32) {
    // Empty constructor body per source: IterativeTypeVisitor::WorkItem::WorkItem(TypeId ty, int32_t parent)
  }
}
