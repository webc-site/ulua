use crate::{
  macros::{
    api_check::api_check, expandstacklimit::expandstacklimit, lua_d_checkstack::luaD_checkstack,
  },
  records::lua_state::LuaState,
};

/// `lua_rawcheckstack`（cpp `lapi.cpp` 同名）：为当前帧直接预留 `size` 层原始头寸。
/// 调用序契约（正确性，非内存安全）：`l` 须指向存活 `LuaState`、`size` 已在上限内
/// 由调用方校验；扩容经 `lua_d_growstack`/resize 重建 `top`/`base`/`stack_last`
/// 指针，调用后不保留旧引用。宏体涉及的裸指针读写与 `ci->top` 落笔属本实现内部
/// unsafe（r16-v3 引用形前移，unsafe 收进实现、不再外包给调用方）。
pub fn lua_rawcheckstack(l: &mut LuaState, size: i32) {
  api_check!(l, size >= 0);

  // SAFETY: 契约保证 `l` 存活且 size 已在上限内校验，扩容经 resize_stack 重建 top/base/stack_last 指针后不保留旧引用
  unsafe {
    luaD_checkstack!(l, size);
    expandstacklimit!(l, (*l).top.wrapping_add(size as usize));
  }
}
