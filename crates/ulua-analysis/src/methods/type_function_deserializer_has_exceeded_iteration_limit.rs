use crate::{
  methods::type_function_serde_checks::exceeded_serde_iteration_limit,
  records::type_function_deserializer::TypeFunctionDeserializer,
};

impl TypeFunctionDeserializer {
  pub fn has_exceeded_iteration_limit(&self) -> bool {
    exceeded_serde_iteration_limit(self.steps, self.queue.len())
  }
}
