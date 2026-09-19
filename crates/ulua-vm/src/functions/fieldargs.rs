use crate::{
  functions::{lua_l_checkinteger::lua_l_checkinteger, lua_l_optinteger::lua_l_optinteger},
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_error::luaL_error},
  type_aliases::lua_state::lua_State,
};

/// cpp `bit32.c:fieldargs`：取字段起点 `f` 与宽度 `w` 并校验范围。
///
/// cpp 用 `int* width` 出参 + 返回 `f`，Rust 版折叠为元组返回。
///
/// # Safety
///
/// `l` 必须指向有效、活跃的 `lua_State`。
pub(crate) unsafe fn fieldargs(l: *mut lua_State, farg: i32) -> (i32, i32) {
  unsafe {
    let f = lua_l_checkinteger(l, farg);
    let w = lua_l_optinteger(l, farg + 1, 1);

    luaL_argcheck!(l, 0 <= f, farg, "field cannot be negative");
    luaL_argcheck!(l, 0 < w, farg + 1, "width must be positive");

    // Widen the add: `f`/`w` are user-supplied and (with f>=0, w>0) `f + w`
    // overflows `int` for huge f (UB in C++; panic with overflow-checks).
    if f as i64 + w as i64 > 32 {
      luaL_error!(l, "trying to access non-existent bits");
    }

    (f, w)
  }
}
