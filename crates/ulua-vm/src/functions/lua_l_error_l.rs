use core::{ffi::c_char, fmt::Arguments};

use crate::{
  functions::{
    lua_concat::lua_concat, lua_error::lua_error, lua_l_where::lua_l_where,
    lua_pushvfstring::lua_pushvfstring,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// 调用方须保证：`l` 为存活调用帧且栈顶至少留有 2 个空闲槽（lWhere 的 level 值与 pushfstring 结果供
/// `lua_concat` 合并）；须在能捕获抛错的受保护帧内调用（末尾 `lua_error` 不返回）；`args` 的格式化参数
/// 须与占位符一致（`lua_pushvfstring` 按约定消费，错配即 UB）。
///
/// 末尾 `lua_error` 必然抛出（longjmp 等价物），故本函数不返回。
///
/// `_fmt` 仅为镜像 cpp `laux.cpp:88` `luaL_error` 的公开签名（外部 crate 以
/// `c"..."` 实参调用）；实际格式化由 `args`（`format_args!` 产物）完成。
pub unsafe fn lua_l_error_l(l: *mut LuaState, _fmt: *const c_char, args: Arguments<'_>) -> ! {
  // Safety: fmt 与可变参按 `%s/%d/%f` 约定严格匹配（错配即 UB），块内经 lua_o_pushvfstring 格式化后经 `l` 抛出、不返回
  unsafe {
    lua_l_where(l, 1);
    lua_pushvfstring(l, args);
    lua_concat(l, 2);
    lua_error(l)
  }
}
