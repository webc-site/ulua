use crate::{
  macros::{
    api_check::api_check, expandstacklimit::expandstacklimit, lua_d_checkstack::luaD_checkstack,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub unsafe fn lua_rawcheckstack(l: *mut LuaState, size: i32) {
  api_check!(l, size >= 0);

  // Safety: 契约保证 `l` 存活且 size 已在上限内校验，扩容经 resize_stack 重建 top/base/stack_last 指针后不保留旧引用
  unsafe {
    luaD_checkstack!(l, size);
    expandstacklimit!(l, (*l).top.wrapping_add(size as usize));
  }
}
