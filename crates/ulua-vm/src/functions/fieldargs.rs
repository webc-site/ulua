use crate::{
  functions::lua_l_optinteger::lua_l_optinteger, macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

/// cpp `bit32.c:fieldargs`：取字段起点 `f` 与宽度 `w` 并校验范围。
///
/// cpp 用 `int* width` 出参 + 返回 `f`，Rust 版折叠为元组返回。
///
/// 调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调——`l` 为存活调用帧，
/// farg 与 farg+1 索引可读（越界或非数值由 check*/argerror 报错回退）。
pub(crate) fn fieldargs(l: &mut LuaState, farg: i32) -> (i32, i32) {
  let f = l.check_integer(farg);
  let w = lua_l_optinteger(l, farg + 1, 1);

  l.arg_check(0 <= f, farg, "field cannot be negative");
  l.arg_check(0 < w, farg + 1, "width must be positive");

  // Widen the add: `f`/`w` are user-supplied and (with f>=0, w>0) `f + w`
  // overflows `int` for huge f (UB in C++; panic with overflow-checks).
  if f as i64 + w as i64 > 32 {
    luaL_error!(l, "trying to access non-existent bits");
  }

  (f, w)
}
