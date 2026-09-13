use crate::{
  records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn run_type_function_type_id(&mut self, root_ty: TypeFunctionTypeId) {
    self.parent_cursor = -1;
    self.work_cursor = 0;
    self.work_queue.clear();
    self.traverse_type_function_type_id(root_ty);
    self.process_work_queue();
  }
}
