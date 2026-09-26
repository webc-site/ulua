use alloc::string::String;
use core::ptr::{from_ref, null};
use std::panic::panic_any;

use crate::{
  enums::follow_option::FollowOption,
  functions::{get_mutable_type::get_mutable, get_type::get, unwrap_lazy::unwrap_lazy},
  records::{
    internal_compiler_error::InternalCompilerError, lazy_type::LazyType, table_type::TableType,
    txn_log::TxnLog,
  },
  type_aliases::{bound_type::BoundType, type_id::TypeId},
};

/// C++ `TypeId follow(TypeId t)`：解引用 Bound/Table/Lazy 链。
/// t 的有效性由调用方按 C++ 同契约保证。
pub fn follow(t: TypeId) -> TypeId {
  follow_full(t, FollowOption::Normal, FollowMapper::Identity)
}

/// C++ `TypeId follow(TypeId t, FollowOption option)`。
/// t 的有效性由调用方按 C++ 同契约保证。
pub fn follow_with_option(t: TypeId, follow_option: FollowOption) -> TypeId {
  follow_full(t, follow_option, FollowMapper::Identity)
}

/// follow 的映射器：C++ `void* context + 函数指针` 回调的 Rust 惯用替代，
/// 消除 context 裸指针往返与 `&*(context as *const TxnLog)` 重建引用的 unsafe。
pub(crate) enum FollowMapper<'a> {
  /// 恒等映射（无 TxnLog 参与）。
  Identity,
  /// 经 TxnLog 的 pending 类型重定向（C++ `follow(t, option, &log, mapper)`）。
  Log(&'a TxnLog),
}

// C++ `TypeId follow(TypeId, FollowOption, void*, Mapper)` 主 overload。
// 映射器收口为枚举后，context 往返的 unsafe 一并消失；节点有效性仍由
// 调用方按 C++ 同契约保证（对外的 `follow` / `follow_with_option` 同约定）。
pub(crate) fn follow_full(
  mut t: TypeId,
  follow_option: FollowOption,
  mapper: FollowMapper,
) -> TypeId {
  let advance = |ty: TypeId| -> Option<TypeId> {
    let mapped = match &mapper {
      FollowMapper::Identity => ty,
      FollowMapper::Log(log) => {
        let state = log.pending_type_id(ty);
        if state.is_null() {
          ty
        } else {
          // Safety: pending_type_id 非空命中时返回 log 事务链中持有节点的
          // Box<PendingType> 堆地址（Box 内容不随 log/Vec 移动，随 &log 借用
          // 'a 存活，覆盖本 advance 调用）；此处只取 pending 字段地址，语义
          // 同 C++ 回调 `&pending->pending`，只读、无别名。
          unsafe { from_ref(&(*state).pending) }
        }
      }
    };

    if let Some(btv) = get::<BoundType>(mapped) {
      return Some(btv.bound_to);
    }

    if let Some(ttv) = get::<TableType>(mapped) {
      return ttv.bound_to;
    }

    if follow_option != FollowOption::DisableLazyTypeThunks {
      // SAFETY: LazyType 节点有效性由 arena 契约保证（C++ 同款直取）
      if let Some(ltv) = get_mutable::<LazyType>(mapped) {
        // Safety: get_mutable 返回 Some ⇒ ltv 是指向 mapped 处 LazyType 变体的
        // 独占可变句柄（类型 arena 节点地址稳定）；&mut 到裸指针为隐式退化，
        // 不新建别名。unwrap_lazy 的契约（ltv 存活、可写）由上述独占借用满足，
        // 其内对 ltv 字段的读写是本次遍历对该节点的唯一访问，单线程串行。
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
