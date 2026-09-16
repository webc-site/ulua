use crate::{records::iterative_type_visitor::IterativeTypeVisitor, type_aliases::type_id::TypeId};

impl IterativeTypeVisitor {
  pub fn run_type_id(&mut self, root_ty: TypeId) {
    self.parent_cursor = -1;
    self.work_cursor = 0;
    self.work_queue.clear();
    self.traverse_type_id(root_ty);
    self.iterative_type_visitor_process_work_queue();
  }
}
