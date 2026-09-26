use alloc::vec::Vec;

use crate::{records::function_type::FunctionType, type_aliases::type_pack_id::TypePackId};

impl FunctionType {
  pub fn generic_packs(&self) -> &Vec<TypePackId> {
    &self.generic_packs
  }
}
