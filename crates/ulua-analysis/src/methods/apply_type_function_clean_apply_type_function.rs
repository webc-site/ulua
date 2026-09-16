use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::apply_type_function::ApplyTypeFunction, type_aliases::type_id::TypeId};

impl ApplyTypeFunction {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    let arg = self.type_arguments.find(&ty).unwrap();
    LUAU_ASSERT!(!arg.is_null());
    *arg
  }
}
