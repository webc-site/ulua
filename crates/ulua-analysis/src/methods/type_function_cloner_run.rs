use crate::records::type_function_cloner::TypeFunctionCloner;

impl TypeFunctionCloner {
  /// C++ `TypeFunctionCloner::run`（TypeFunctionRuntime.cpp:2665-2676）。
  pub fn run(&mut self) {
    while !self.queue.is_empty() {
      self.steps += 1;

      if self.has_exceeded_iteration_limit() {
        break;
      }

      let Some((ty, tfti)) = self.queue.pop() else {
        break;
      };

      self.clone_children_kind(&ty, &tfti);
    }
  }
}
