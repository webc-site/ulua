use crate::{
  records::ambiguous_function_call::AmbiguousFunctionCall, type_aliases::type_id::TypeId,
};

impl AmbiguousFunctionCall {
  pub fn function(&self) -> TypeId {
    self.function
  }
}
