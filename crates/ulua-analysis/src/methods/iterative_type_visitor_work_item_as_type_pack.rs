use crate::{
  records::iterative_type_visitor::IterativeTypeVisitor, type_aliases::type_pack_id::TypePackId,
};

impl IterativeTypeVisitor {
  pub fn iterative_type_visitor_work_item_as_type_pack(&mut self) -> Option<*const TypePackId> {
    if let Some(work_item) = self.work_queue.last()
      && !work_item.is_type
    {
      return Some(work_item.as_type_pack());
    }
    None
  }
}
