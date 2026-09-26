use crate::records::type_function_serializer::TypeFunctionSerializer;

impl TypeFunctionSerializer {
  /// C++ `TypeFunctionSerializer::run`（TypeFunctionRuntimeBuilder.cpp:101-111）：
  /// 先计步、再查限位/错误，与 deserializer/cloner 的循环同构。
  pub fn run(&mut self) {
    while !self.queue.is_empty() {
      self.steps += 1;

      if self.has_exceeded_iteration_limit() || self.has_errors() {
        break;
      }

      if let Some((kind, tfkind)) = self.queue.pop() {
        self.serialize_children_kind(kind, tfkind);
      }
    }
  }
}
