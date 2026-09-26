use crate::{
  functions::{
    begin_type::{begin_intersection_type, begin_union_type},
    follow_type, get_type,
  },
  records::{
    intersection_type::IntersectionType, singleton_type::SingletonType,
    type_function_instance_type::TypeFunctionInstanceType, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

pub fn maybe_singleton(ty: TypeId) -> bool {
  let ty = follow_type::follow(ty);

  if get_type::get::<SingletonType>(ty).is_some() {
    return true;
  }

  // C++ `for (TypeId option : utv)` / `for (TypeId part : itv)`——迭代器
  // follow Bound 并展平嵌套同类型,裸遍历 options/parts 会漏掉嵌套成员。
  if let Some(utv) = get_type::get::<UnionType>(ty) {
    for option in begin_union_type(utv) {
      if get_type::get::<SingletonType>(follow_type::follow(option)).is_some() {
        return true;
      }
    }
  }

  if let Some(itv) = get_type::get::<IntersectionType>(ty) {
    for part in begin_intersection_type(itv) {
      if maybe_singleton(part) {
        // will i regret this?
        return true;
      }
    }
  }

  if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(ty) {
    // SAFETY: function 指向 TypeArena 中的 TypeFunction，会话期有效。
    let name = unsafe { tfit.function.as_ref() }.name.as_str();
    if matches!(name, "keyof" | "rawkeyof") {
      return true;
    }
  }
  false
}
