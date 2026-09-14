use ulua_analysis::{records::function_type::FunctionType, type_aliases::type_id::TypeId};

use crate::records::overload_resolver_fixture::OverloadResolverFixture;

impl OverloadResolverFixture {
  pub fn fn_item(&self, args: &[TypeId], rets: &[TypeId]) -> TypeId {
    let arg_pack = self.pack_initializer_list_type_id(args);
    let ret_pack = self.pack_initializer_list_type_id(rets);
    unsafe {
      (*self.arena).add_type(FunctionType::function_type_new(
        arg_pack, ret_pack, None, false,
      ))
    }
  }
}
