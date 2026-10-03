use core::fmt::Arguments;

use crate::{
  functions::{lua_error::lua_error, lua_l_where::lua_l_where, lua_pushvfstring::lua_pushvfstring},
  records::lua_state::LuaState,
};

/// # Safety
/// 调用方须保证：`l` 为存活调用帧且栈顶至少留有 2 个空闲槽（`lua_l_where` 的 level 值与
/// `lua_pushvfstring` 结果供 `lua_concat` 合并）；须在能捕获抛错的受保护帧内调用（末尾
/// `lua_error` 不返回）；`args` 由 `format_args!` 现场构造，占位符与实参在构造处即静态匹配，
/// 且不得有逃逸出本次调用的借用。
///
/// 末尾 `lua_error` 必然抛出（longjmp 等价物），故本函数不返回。
///
/// cpp `laux.cpp:88` `luaL_error(L, fmt, ...)` 的 `fmt` 形参在此端口无对应物：格式化完全由
/// `args` 承担，故 Rust 侧不再收该参数（DELIBERATE DEVIATION：cpp 保留 `const char*` 只为
/// 其 varargs 协议，照抄会诱导调用方书写 `c"..."` 字面量并误以为参与格式化）。
pub unsafe fn lua_l_error_l(l: *mut LuaState, args: Arguments<'_>) -> ! {
  // SAFETY: `l` 由本函数入口 `# Safety` 契约保证为存活、可抛错的受保护帧且留有 2 空槽，
  // 该前提原样透传给 `lua_l_where`/`lua_pushvfstring`/`lua_error`。
  unsafe {
    let l = &mut *l;
    lua_l_where(l, 1);
    lua_pushvfstring(l, args);
    l.concat(2);
    lua_error(l)
  }
}
