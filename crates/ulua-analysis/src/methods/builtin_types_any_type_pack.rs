use crate::{records::builtin_types::BuiltinTypes, type_aliases::type_pack_id::TypePackId};

impl BuiltinTypes {
  pub fn any_type_pack(&self) -> TypePackId {
    self.any_type_pack
  }
}
