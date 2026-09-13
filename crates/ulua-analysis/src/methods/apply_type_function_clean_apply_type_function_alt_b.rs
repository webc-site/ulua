use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::apply_type_function::ApplyTypeFunction, type_aliases::type_pack_id::TypePackId,
};

impl ApplyTypeFunction {
  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let arg = self
      .type_pack_arguments
      .find(&tp)
      .expect("TypePackId not found in type_pack_arguments");
    LUAU_ASSERT!(!arg.is_null());
    *arg
  }
}
