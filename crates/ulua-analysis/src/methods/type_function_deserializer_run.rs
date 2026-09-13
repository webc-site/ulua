use crate::records::type_function_deserializer::TypeFunctionDeserializer;

impl TypeFunctionDeserializer {
  pub fn run(&mut self) {
    while !self.queue.is_empty() {
      self.steps += 1;

      if self.has_exceeded_iteration_limit() || self.has_errors() {
        break;
      }

      if let Some((tfkind, kind)) = self.queue.pop() {
        self.deserialize_children_type_function_kind_type_or_pack(tfkind, kind);
      }

      if let Some(scope) = self.function_scopes.last().cloned()
        && self.queue.len() == scope.old_queue_size
        && !self.has_errors()
      {
        unsafe { self.close_function_scope(scope.function) };
        self.function_scopes.pop();
      }
    }
  }
}
