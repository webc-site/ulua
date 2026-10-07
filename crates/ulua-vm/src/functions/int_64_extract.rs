use crate::{
  functions::{lua_l_optinteger_64::lua_l_optinteger_64, lua_pushinteger_64::lua_pushinteger_64},
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn, mask_64::mask64},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载；r16-v35 收形后
/// 读槽/校验/压栈全经安全门面与被调，仅 Lua 报错路径按 `fieldargs`/`check_div_args_64`
/// 判例保留一处 `l.as_mut_ptr()` 窄重建 `unsafe` 块，故本体降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧：栈 1/2 号位为可转 i64 的值、3 号位可选宽度经
/// `lua_l_optinteger_64` 取值；`arg_check` 三项校验与越界报错均抛错发散，
/// `lua_pushinteger_64` 写回结果需 `top` 后 ≥1 空槽。cpp/VM/src/lintlib.cpp:430。
pub fn int64_extract(l: &mut LuaState) -> i32 {
  let n = l.check_integer_64(1);
  let f = l.check_integer_64(2);
  let w = lua_l_optinteger_64(l, 3, 1);

  l.arg_check((0..=63).contains(&f), 2, "field cannot be negative");
  l.arg_check(0 < w, 3, "width must be positive");
  // `f` is bounded to [0,63] above; compare `w > 64 - f` so a near-i64::MAX
  // width can't overflow the `f + w` addition.
  if w > 64 - f {
    luaL_error!(l, "trying to access non-existent bits");
  }

  lua_pushinteger_64(l, (((n as u64) >> f as u32) & mask64(w as i32)) as i64);

  1
}

lua_lib_fn!(pub fn int64_extract @ref, int64_extract_arm);
