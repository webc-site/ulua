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
    // 存活栈窗口 [stack, top)：一次槽距定界后切成切片正序标记，
    // 取代逐格 o.add(j) 指针游走；元素序与 cpp 完全一致（GC 遍历顺序即语义）
    // SAFETY: top 在 stack 之上（栈不变式），区间 [stack, top) ⊆ [stack, stack+stacksize)
    // r16-b2 收编：stack 锚槽距读数落 slot_distance 边界原语（stack.rs:53，本票升
    // pub(crate) 解锁）——本体即被替代式 `to.offset_from(from) as i32` 的同址镜像；
    // 栈不变量已证窗非负，isize→i32 折形在 stacksize 约束域无截差，非负值 `as usize`
    // 收窄与原式逐位同值（本点位原形无 max，不补）
    let count = LuaState::slot_distance((*l).stack, (*l).top) as usize;
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
