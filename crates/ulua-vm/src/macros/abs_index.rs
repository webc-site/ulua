use crate::{macros::lua_registryindex::LUA_REGISTRYINDEX, records::lua_state::LuaState};

/// 将相对栈索引换算为绝对索引。仅读 `LuaState` 的 `top`/`base` 指针字段，
/// 故以 `&LuaState` 接收者替代原 `*mut` 裸指针。
#[inline(always)]
pub(crate) fn abs_index(l: &LuaState, i: i32) -> i32 {
  if i > 0 || i <= LUA_REGISTRYINDEX {
    i
  } else {
    // Safety: l 为存活 LuaState 引用，base..top 同属一块栈区，offset_from 于同分配内合法
    let top = unsafe { l.top.offset_from(l.base) as i32 };
    top + i + 1
  }
}
