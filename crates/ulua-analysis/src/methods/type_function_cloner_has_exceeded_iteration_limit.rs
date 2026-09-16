use ulua_common::DFInt;

use crate::records::type_function_cloner::TypeFunctionCloner;

impl TypeFunctionCloner {
  pub fn has_exceeded_iteration_limit(&self) -> bool {
    self.steps + self.queue.len() as i32 >= DFInt::LuauTypeFunctionSerdeIterationLimit.get()
  }
}
