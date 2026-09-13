use alloc::vec::Vec;

use crate::{records::function_type::FunctionType, type_aliases::type_id::TypeId};

impl FunctionType {
  pub fn generics(&self) -> &Vec<TypeId> {
    &self.generics
  }
}
