use crate::{
  records::ambiguous_function_call::AmbiguousFunctionCall,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl AmbiguousFunctionCall {
  pub fn new(function: TypeId, arguments: TypePackId) -> Self {
    Self {
      function,
      arguments,
    }
  }
}
