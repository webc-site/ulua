use alloc::vec::Vec;

use crate::{
  functions::{begin_type::begin_union_type, get_type, is_prim::is_nil},
  records::{type_arena::TypeArena, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

/// C++ `static std::optional<TypeId> tryStripUnionFromNil(TypeArena&, TypeId)`
/// (`Analysis/src/TypeUtils.cpp`)。迭代走 `begin(utv)` —— UnionTypeIterator
/// 防环展平并 follow。
pub fn try_strip_union_from_nil(arena: &mut TypeArena, ty: TypeId) -> Option<TypeId> {
  if let Some(utv) = get_type::get::<UnionType>(ty) {
    if !begin_union_type(utv).any(is_nil) {
      return Some(ty);
    }

    let result: Vec<TypeId> = begin_union_type(utv)
      .filter(|&option| !is_nil(option))
      .collect();

    if result.is_empty() {
      return None;
    }

    return if result.len() == 1 {
      Some(result[0])
    } else {
      Some(arena.add_type(UnionType { options: result }))
    };
  }

  None
}
