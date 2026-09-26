use alloc::vec::Vec;

use crate::{
  functions::{begin_type::begin_union_type, get_type, is_prim::is_nil},
  records::{type_checker::TypeChecker, union_type::UnionType},
  type_aliases::type_id::TypeId,
};
impl TypeChecker {
  pub fn try_strip_union_from_nil(&mut self, ty: TypeId) -> Option<TypeId> {
    let utv = get_type::get::<UnionType>(ty)?;

    // C++ `std::any_of(begin(utv), end(utv), isNil)` 与
    // `for (TypeId option : utv)` — UnionTypeIterator 防环展平并 follow。
    if !begin_union_type(utv).any(is_nil) {
      return Some(ty);
    }

    let result: Vec<TypeId> = begin_union_type(utv)
      .filter(|&option| !is_nil(option))
      .collect();

    if result.is_empty() {
      return None;
    }

    if result.len() == 1 {
      return Some(result[0]);
    }

    Some(self.add_type(&UnionType { options: result }))
  }
}
