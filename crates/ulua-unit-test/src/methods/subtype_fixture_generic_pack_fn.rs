use ulua_analysis::{
  records::function_type::FunctionType,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn generic_pack_fn(
    &mut self,
    generic_packs: Vec<TypePackId>,
    arg_types: TypePackId,
    ret_types: TypePackId,
  ) -> TypeId {
    self.arena.add_type(FunctionType::new_with_generics(
      Vec::new(),
      generic_packs,
      arg_types,
      ret_types,
      None,
      false,
    ))
  }
}
