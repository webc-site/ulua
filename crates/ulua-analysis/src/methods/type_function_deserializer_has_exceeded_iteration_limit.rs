use ulua_common::DFInt;

use crate::records::type_function_deserializer::TypeFunctionDeserializer;
impl TypeFunctionDeserializer {
  pub fn has_exceeded_iteration_limit(&self) -> bool {
    let limit = DFInt::LuauTypeFunctionSerdeIterationLimit.get();
    limit != 0 && self.steps + self.queue.len() as i32 >= limit
  }
}
