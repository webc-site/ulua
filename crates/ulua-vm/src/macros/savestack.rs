//! B2-2b 帧协议族收口：cpp `VM/src/ldo.h` `#define savestack(L,p) ((char*)(p) - (char*)L->stack)`
//! （槽指针 → 相对栈基的有符号字节偏移，跨栈重分配的暂存惯用法）。
//!
//! §3 出参→返回值改写：宏体的裸指针解引用/arena 边界断言折叠为 `&LuaState` 入参、
//! `isize` 出参的 safe fn（数值进出、函数体无解引用），宏壳仅存导出面兼容转发——
//! 展开点不再复制 `(*l).stack`/`stacksize` 指针算式（消费者文件属各自票面，壳签名不动）。

use core::mem::size_of;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// 槽指针 → 栈基 `l.stack` 的有符号字节偏移（cpp `savestack`，ldo.h）。
///
/// safe fn：读 `l.stack`/`l.stacksize` 经共享引用，偏移为纯数值输出，函数体无解引用。
/// 出参形态即「存偏移后跨扩容恢复」契约——返回的偏移经
/// [`restorestack_slot`](crate::macros::restorestack::restorestack_slot) 在栈重分配后
/// 重派生槽指针；原 `check_exp!` 的界内断言（debug 生效，release 恒真短路）随体收口。
#[inline(always)]
pub(crate) fn savestack_offset(l: &LuaState, p: StkId) -> isize {
  let stack = l.stack as usize;
  // SAFETY 面为零：usize 地址比较与减法不做解引用，界内断言仅供 debug 校验
  LUAU_ASSERT!(
    (p as usize >= stack) && (p as usize <= stack + (l.stacksize as usize) * size_of::<TValue>())
  );
  (p as isize) - (stack as isize)
}

#[macro_export]
macro_rules! savestack {
  ($l:expr, $p:expr) => {
    $crate::macros::savestack::savestack_offset(&*$l, $p)
  };
}

pub use savestack;
