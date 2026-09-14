use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::replacer::Replacer, type_aliases::type_id::TypeId};

impl Replacer {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    let res = unsafe { (*self.replacements).find(&ty) }.expect("TypeId not found in replacements");
    LUAU_ASSERT!(!res.is_null());
    let cleaned = *res;
    self.base.dont_traverse_into_type_id(cleaned);
    cleaned
  }
}
