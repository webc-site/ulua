//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:257:follow`
//! Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:257-312, hand-ported; principal overload)

use alloc::string::String;
use core::{ffi::c_void, ptr::null};
use std::panic::panic_any;

use crate::{
  functions::get_type_pack::get,
  records::{internal_compiler_error::InternalCompilerError, type_pack::TypePack},
  type_aliases::{bound_type_pack::BoundTypePack, type_pack_id::TypePackId},
};
type Mapper = fn(*const c_void, TypePackId) -> TypePackId;

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn follow_pack_full(
  mut tp: TypePackId,
  context: *const c_void,
  mapper: Mapper,
) -> TypePackId {
  let advance = |ty: TypePackId| -> Option<TypePackId> {
    let mapped = mapper(context, ty);

    if let Some(btv) = get::<BoundTypePack>(mapped) {
      return Some(btv.bound_to);
    }

    if let Some(pack) = get::<TypePack>(mapped)
      && pack.head.is_empty()
    {
      return pack.tail;
    }

    None
  };

  let mut cycle_tester: TypePackId = tp;
  if let Some(a) = advance(cycle_tester) {
    cycle_tester = a;
  } else {
    return tp;
  }

  if advance(cycle_tester).is_none() {
    return cycle_tester;
  }

  loop {
    match advance(tp) {
      Some(a1) => tp = a1,
      None => return tp,
    }

    if !cycle_tester.is_null() {
      match advance(cycle_tester) {
        Some(a2) => match advance(a2) {
          Some(a3) => cycle_tester = a3,
          None => cycle_tester = null(),
        },
        None => cycle_tester = null(),
      }

      if tp == cycle_tester {
        panic_any(InternalCompilerError::new(
          String::from("Luau::follow detected a TypePack cycle!!"),
          None,
          None,
        ));
      }
    }
  }
}
