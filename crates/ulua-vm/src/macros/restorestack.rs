//! B2-2b 帧协议族收口：cpp `VM/src/ldo.h`
//! `#define restorestack(l, n) ((TValue*)((char*)l->stack + (n)))`
//! （栈基有符号字节偏移 → 槽指针重派生，与 [`savestack`](crate::macros::savestack) 互逆）。
//!
//! §3 收口：宏体的 `(*$l)` 解引用与裸 `offset` 折叠为共享引用入参、`isize` 入参的
//! safe fn（数值入参；偏移算术走 safe 的 `wrapping_offset`，函数体无解引用）。指针
//! 输出是 StkId arena 的固有形态——CallInfo/base/top 字段保留裸 `StkId`（
//! `records/call_info.rs:8-29` DELIBERATE DEVIATION 裁决），解引用归调用侧 unsafe 上下文。

use crate::{
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// 栈基有符号字节偏移 → 槽指针（cpp `restorestack`，ldo.h）。
///
/// safe fn：`l.stack` 经共享引用读出，偏移算术不解除引用；返回指针仅在
/// `savestack_offset` 记录偏移以来栈未再移动时界内，落库/解引用前的校验归
/// 调用侧契约（既有「扩容先行、偏移暂存、边界一处重建」约定）。
#[inline(always)]
pub(crate) fn restorestack_slot(l: &LuaState, n: isize) -> StkId {
  // 与旧宏体 `(stack as *mut u8).offset(n)` 值语义一致；wrapping_offset 免 UB 面且 safe
  l.stack.cast::<u8>().wrapping_offset(n).cast::<TValue>()
}

#[macro_export]
macro_rules! restorestack {
  ($l:expr, $n:expr) => {
    $crate::macros::restorestack::restorestack_slot(&*$l, $n as isize)
  };
}

pub use restorestack;
