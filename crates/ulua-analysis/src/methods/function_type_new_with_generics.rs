use alloc::vec::Vec;

use crate::{
  records::{function_definition::FunctionDefinition, function_type::FunctionType},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl FunctionType {
  pub fn new_with_generics(
    generics: Vec<TypeId>,
    generic_packs: Vec<TypePackId>,
    arg_types: TypePackId,
    ret_types: TypePackId,
    defn: Option<FunctionDefinition>,
    has_self: bool,
  ) -> Self {
    let mut result = Self::function_type_new(arg_types, ret_types, defn, has_self);
    result.generics = generics;
    result.generic_packs = generic_packs;
    result
  }
}
