use ulua_analysis::{records::function_type::FunctionType, type_aliases::type_id::TypeId};

use crate::records::simplify_fixture::SimplifyFixture;

impl SimplifyFixture {
  pub fn mk_function(&mut self, arg: TypeId, ret: TypeId) -> TypeId {
    let arg_pack = self.arena.add_type_pack_initializer_list_type_id(&[arg]);
    let ret_pack = self.arena.add_type_pack_initializer_list_type_id(&[ret]);
    self.arena.add_type(FunctionType::function_type_new(
      arg_pack, ret_pack, None, false,
    ))
  }
}
