use crate::{
  records::type_function_cloner::TypeFunctionCloner,
  type_aliases::type_function_kind::TypeFunctionKind,
};

impl TypeFunctionCloner {
  pub fn run(&mut self) {
    while !self.queue.is_empty() {
      self.steps += 1;

      if self.has_exceeded_iteration_limit() {
        break;
      }

      let (ty, tfti): (TypeFunctionKind, TypeFunctionKind) = {
        let last = self.queue.pop().unwrap();
        (last.0, last.1)
      };

      self.clone_children_type_function_kind_type_function_kind(&ty, &tfti);
    }
  }
}
