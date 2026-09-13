use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    intersection_type::IntersectionType, singleton_type::SingletonType,
    type_function_instance_type::TypeFunctionInstanceType, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

pub fn maybe_singleton(ty: TypeId) -> bool {
  let ty = follow_type_id(ty);

  if get_type_id::<SingletonType>(ty).is_some() {
    return true;
  }

  if let Some(utv) = get_type_id::<UnionType>(ty) {
    for &option in &utv.options {
      if get_type_id::<SingletonType>(follow_type_id(option)).is_some() {
        return true;
      }
    }
  }

  if let Some(itv) = get_type_id::<IntersectionType>(ty) {
    for &part in &itv.parts {
      if maybe_singleton(part) {
        // will i regret this?
        return true;
      }
    }
  }

  if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(ty) {
    // SAFETY: function 指向 TypeArena 中的 TypeFunction，会话期有效。
    let name = unsafe { tfit.function.as_ref() }.name.as_str();
    if name == "keyof" || name == "rawkeyof" {
      return true;
    }
  }
  false
}
