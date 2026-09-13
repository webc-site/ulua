//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Type.cpp:79:follow`
//! Source: `Analysis/src/Type.cpp` (Type.cpp:79-141, hand-ported; the principal overload)

use alloc::string::String;
use core::{ffi::c_void, ptr::null};
use std::panic::panic_any;

use crate::{
  enums::follow_option::FollowOption,
  functions::{get_mutable_type::get_mutable, get_type_alt_j::get, unwrap_lazy::unwrap_lazy},
  records::{
    internal_compiler_error::InternalCompilerError, lazy_type::LazyType, table_type::TableType,
  },
  type_aliases::{bound_type::BoundType, type_id::TypeId},
};
type Mapper = fn(*const c_void, TypeId) -> TypeId;

// C++ `TypeId follow(TypeId, FollowOption, void*, Mapper)` 主 overload。
// crate 内使用：裸指针解引用由内部 unsafe 块承担，t 有效性由调用方按
// C++ 同契约保证（对外的 `follow` / `follow_with_option` 封装同一约定）。
pub(crate) fn follow_full(
  mut t: TypeId,
  follow_option: FollowOption,
  context: *const c_void,
  mapper: Mapper,
) -> TypeId {
  let advance = |ty: TypeId| -> Option<TypeId> {
    let mapped = mapper(context, ty);

    if let Some(btv) = get::<BoundType>(mapped) {
      return Some(btv.bound_to);
    }

    if let Some(ttv) = get::<TableType>(mapped) {
      return ttv.bound_to;
    }

    if follow_option != FollowOption::DisableLazyTypeThunks {
      // SAFETY: LazyType 节点有效性由 arena 契约保证（C++ 同款直取）
      if let Some(ltv) = get_mutable::<LazyType>(mapped) {
        return Some(unsafe { unwrap_lazy(ltv) });
      }
    }

    None
  };

  // Null once we've determined that there is no cycle
  let mut cycle_tester: TypeId = t;
  if let Some(a) = advance(cycle_tester) {
    cycle_tester = a;
  } else {
    return t;
  }

  // Short circuit traversal for the rather common case when advance(advance(t)) == null
  if advance(cycle_tester).is_none() {
    return cycle_tester;
  }

  loop {
    match advance(t) {
      Some(a1) => t = a1,
      None => return t,
    }

    if !cycle_tester.is_null() {
      match advance(cycle_tester) {
        Some(a2) => match advance(a2) {
          Some(a3) => cycle_tester = a3,
          None => cycle_tester = null(),
        },
        None => cycle_tester = null(),
      }

      if t == cycle_tester {
        panic_any(InternalCompilerError::new(
          String::from("Luau::follow detected a Type cycle!!"),
          None,
          None,
        ));
      }
    }
  }
}
