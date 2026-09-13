use crate::records::type_function_reducer::TypeFunctionReducer;

impl TypeFunctionReducer {
  pub fn done(&self) -> bool {
    self.queued_tys.empty() && self.queued_tps.empty()
  }
}
