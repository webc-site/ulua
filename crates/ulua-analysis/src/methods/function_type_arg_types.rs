use crate::{records::function_type::FunctionType, type_aliases::type_pack_id::TypePackId};

impl FunctionType {
  pub fn arg_types(&self) -> TypePackId {
    self.arg_types
  }
}
