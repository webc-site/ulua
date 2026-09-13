use alloc::vec::Vec;

use crate::{
  functions::{get_type_alt_j::get_type_id, is_nil::is_nil},
  records::{type_arena::TypeArena, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

pub fn try_strip_union_from_nil(arena: &mut TypeArena, ty: TypeId) -> Option<TypeId> {
  if !get_type_id::<UnionType>(ty).is_none() {
    let utv = get_type_id::<UnionType>(ty).unwrap();

    if !utv.options.iter().any(|option| is_nil(*option)) {
      return Some(ty);
    }

    let mut result = Vec::new();

    for option in &utv.options {
      if !is_nil(*option) {
        result.push(*option);
      }
    }

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
