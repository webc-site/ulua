use alloc::vec::Vec;

use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type},
  records::{any_type::AnyType, never_type::NeverType, union_type::UnionType},
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

pub fn reduce_union(types: &[TypeId]) -> Vec<TypeId> {
  let mut result = Vec::new();

  for &t in types {
    let t = follow_type::follow(t);

    if get_type::get::<NeverType>(t).is_some() {
      continue;
    }

    if get_type::get::<ErrorType>(t).is_some() || get_type::get::<AnyType>(t).is_some() {
      return vec![t];
    }

    if let Some(utv) = get_type::get::<UnionType>(t).as_ref() {
      // C++ `for (TypeId ty : utv)`——UnionTypeIterator 展平嵌套 union 并
      // follow,裸遍历 options 会漏掉嵌套成员。
      for ty in begin_union_type(utv) {
        let ty = follow_type::follow(ty);

        if get_type::get::<NeverType>(ty).is_some() {
          continue;
        }

        if get_type::get::<ErrorType>(ty).is_some() || get_type::get::<AnyType>(ty).is_some() {
          return vec![ty];
        }

        if !result.contains(&ty) {
          result.push(ty);
        }
      }
    } else if !result.contains(&t) {
      result.push(t);
    }
  }

  result
}
