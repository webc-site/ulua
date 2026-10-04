use core::fmt::Arguments;

use crate::{
  functions::{lua_error::lua_error, lua_l_where::lua_l_where, lua_pushvfstring::lua_pushvfstring},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活与独占已由 `&mut LuaState` 接收者类型承载，
/// wave-6d 收形降为安全 `pub fn`）：`l` 须为存活调用帧且栈顶至少留有 2 个空闲槽
/// （`lua_l_where` 的 level 值与 `lua_pushvfstring` 结果供 `l.concat(2)` 合并）；须在能捕获
/// 抛错的受保护帧内调用（末尾 `lua_error` 不返回）；`args` 由 `format_args!` 现场构造，
/// 占位符与实参在构造处即静态匹配，且不得有逃逸出本次调用的借用。
///
/// 剩余被调均为收形后的安全门面，各自契约由本函数调用序直传：`lua_l_where`/
/// `lua_pushvfstring` 经独占借用取帧、`l.concat` 为 `LuaState` 自有方法、`lua_error` 已是
/// 安全 `pub fn`，体内无裸解引用残留。
///
/// 末尾 `lua_error` 必然抛出（longjmp 等价物），故本函数不返回。
///
/// cpp `laux.cpp:88` `luaL_error(L, fmt, ...)` 的 `fmt` 形参在此端口无对应物：格式化完全由
/// `args` 承担，故 Rust 侧不再收该参数（DELIBERATE DEVIATION：cpp 保留 `const char*` 只为
/// 其 varargs 协议，照抄会诱导调用方书写 `c"..."` 字面量并误以为参与格式化）。
pub fn lua_l_error_l(l: &mut LuaState, args: Arguments<'_>) -> ! {
  lua_l_where(l, 1);
  lua_pushvfstring(l, args);
  l.concat(2);
  lua_error(l)
}
