use alloc::string::String;
use core::ptr::{from_ref, null};
use std::panic::panic_any;

use crate::{
  functions::get_type_pack::get,
  records::{internal_compiler_error::InternalCompilerError, txn_log::TxnLog, type_pack::TypePack},
  type_aliases::{bound_type_pack::BoundTypePack, type_pack_id::TypePackId},
};

/// C++ `TypePackId follow(TypePackId tp)`。
/// tp 的有效性由调用方按 C++ 同契约保证。
pub fn follow(tp: TypePackId) -> TypePackId {
  follow_pack_full(tp, FollowPackMapper::Identity)
}

/// pack 版映射器：C++ `void* context + 函数指针` 回调的 Rust 惯用替代。
pub(crate) enum FollowPackMapper<'a> {
  /// 恒等映射（无 TxnLog 参与）。
  Identity,
  /// 经 TxnLog 的 pending pack 重定向。
  Log(&'a TxnLog),
}

/// 映射器收口为枚举后，context 往返的 unsafe 一并消失；
/// tp 有效性仍由调用方按 C++ 同契约保证。
pub(crate) fn follow_pack_full(mut tp: TypePackId, mapper: FollowPackMapper) -> TypePackId {
  let advance = |ty: TypePackId| -> Option<TypePackId> {
    let mapped = match &mapper {
      FollowPackMapper::Identity => ty,
      FollowPackMapper::Log(log) => {
        let state = log.pending_type_pack_id(ty);
        if state.is_null() {
          ty
        } else {
          // Safety: `pending_type_pack_id` 命中时返回 log 持有的 Box<PendingTypePack>
          // 内的指针，其目标地址稳定且随 `&log`（借自 mapper，'a）存活；上面的
          // `state.is_null()` 已守卫非空，故 `(*state).pending` 解引用合法，
          // `from_ref` 取该字段地址与原 C++ 回调语义一致。
          unsafe { from_ref(&(*state).pending) }
        }
      }
    };

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
