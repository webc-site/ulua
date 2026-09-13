use crate::{
  records::ambiguous_function_call::AmbiguousFunctionCall, type_aliases::type_pack_id::TypePackId,
};

impl AmbiguousFunctionCall {
  pub fn arguments(&self) -> TypePackId {
    self.arguments
  }
}
