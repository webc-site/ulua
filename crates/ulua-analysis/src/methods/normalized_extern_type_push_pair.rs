use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{normalized_extern_type::NormalizedExternType, type_ids::TypeIds},
  type_aliases::type_id::TypeId,
};

impl NormalizedExternType {
  pub fn push_pair(&mut self, ty: TypeId, negations: TypeIds) {
    let result = self.extern_types.insert(ty, negations);

    if result.is_none() {
      self.ordering.push(ty);
    }

    LUAU_ASSERT!(self.ordering.len() == self.extern_types.len());
  }
}
