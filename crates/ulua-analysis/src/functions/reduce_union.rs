use alloc::vec::Vec;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{any_type::AnyType, never_type::NeverType, union_type::UnionType},
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

pub fn reduce_union(types: &[TypeId]) -> Vec<TypeId> {
  let mut result = Vec::new();

  for &t in types {
    let t = follow_type_id(t);

    if !get_type_id::<NeverType>(t).is_none() {
      continue;
    }

    if !get_type_id::<ErrorType>(t).is_none() || !get_type_id::<AnyType>(t).is_none() {
      return vec![t];
    }

    if let Some(utv) = get_type_id::<UnionType>(t).as_ref() {
      for &ty in utv.options.iter() {
        let ty = follow_type_id(ty);

        if !get_type_id::<NeverType>(ty).is_none() {
          continue;
        }

        if !get_type_id::<ErrorType>(ty).is_none() || !get_type_id::<AnyType>(ty).is_none() {
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
