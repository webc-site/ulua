use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    internal_type_finder::InternalTypeFinder,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl InternalTypeFinder {
  pub fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    _tp: TypePackId,
    _tfitp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    LUAU_ASSERT!(false);
    false
  }
}
