use crate::records::type_function_serializer::TypeFunctionSerializer;

impl TypeFunctionSerializer {
  pub fn run(&mut self) {
    while !self.queue.is_empty() {
      if self.has_exceeded_iteration_limit() || self.has_errors() {
        break;
      }

      self.steps += 1;

      if let Some((kind, tfkind)) = self.queue.pop() {
        self.serialize_children_type_or_pack_type_function_kind(kind, tfkind);
      }
    }
  }
}
