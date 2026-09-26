//! Source: `VM/src/lgc.cpp` (lgc.cpp:417-430, hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::c_slice,
  macros::{
    markobject::markobject, markvalue::markvalue, stringmark::stringmark, upisopen::upisopen,
  },
  records::{global_state::global_State, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn traversestack(g: *mut global_State, l: *mut LuaState) {
  unsafe {
    markobject!(g, (*l).gt);
    if !(*l).namecall.is_null() {
      stringmark!((*l).namecall);
    }
    // 存活栈窗口 [stack, top)：一次 offset_from 定界后切成切片正序标记，
    // 取代逐格 o.add(j) 指针游走；元素序与 cpp 完全一致（GC 遍历顺序即语义）
    // Safety: top 在 stack 之上（栈不变式），区间 [stack, top) ⊆ [stack, stack+stacksize)
    let count = (*l).top.offset_from((*l).stack) as usize;
    for slot in c_slice((*l).stack, count).iter() {
      markvalue!(g, slot);
    }
    // openupval 链是链表数据结构本身（threadnext 串接），保留指针推进
    let mut uv = (*l).openupval;
    while !uv.is_null() {
      LUAU_ASSERT!(upisopen!(uv));
      (*uv).markedopen = 1;
      markobject!(g, uv);
      uv = (*uv).u.open.threadnext;
    }
  }
}
