use ulua_analysis::{records::function_type::FunctionType, type_aliases::type_id::TypeId};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn generic_fn(
    &mut self,
    generics: Vec<TypeId>,
    args: Vec<TypeId>,
    rets: Vec<TypeId>,
  ) -> TypeId {
    let arg_pack = self.pack_initializer_list_type_id(args);
    let ret_pack = self.pack_initializer_list_type_id(rets);
    self.arena.add_type(FunctionType::new_with_generics(
      generics,
      Vec::new(),
      arg_pack,
      ret_pack,
      None,
      false,
    ))
  }
}
