use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v35 收形后
/// 体内全经安全门面方法，无裸操作，降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧且栈 index 1 为可转 64 位整数的值（`check_integer_64` 读槽并可抛错），
/// `push_number` 可能扩栈，须在受保护帧内由 C 侧调入。cpp `lintlib.cpp:52`。
pub fn int64_tonumber(l: &mut LuaState) -> i32 {
  let x = l.check_integer_64(1);
  l.push_number(x as f64);
  1
}

lua_lib_fn!(pub fn int64_tonumber @ref, int64_tonumber_arm);
