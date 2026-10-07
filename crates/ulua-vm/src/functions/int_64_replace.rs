use crate::{
  functions::{lua_l_optinteger_64::lua_l_optinteger_64, lua_pushinteger_64::lua_pushinteger_64},
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn, mask_64::mask64},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载；r16-v35 收形后
/// 读槽/校验/数值替换/压栈全经安全门面与被调，仅 Lua 报错路径按
/// `fieldargs`/`check_div_args_64` 判例保留一处 `l.as_mut_ptr()` 窄重建 `unsafe` 块，
/// 故本体降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧且实参 1..=4 可读、栈顶留有结果空间。
/// cpp lintlib.cpp:446 `int64_replace`：把 r 的低 w 位写入 n 的第 f 位起字段。
pub fn int64_replace(l: &mut LuaState) -> i32 {
  let n = l.check_integer_64(1);
  let r = l.check_integer_64(2);
  let f = l.check_integer_64(3);
  let w = lua_l_optinteger_64(l, 4, 1);

  // cpp lintlib.cpp:453：replace 签名为 (value, replacement, field, width)，
  // f 取自第 3 槽，报错 argnum 必须是 3
  l.arg_check((0..=63).contains(&f), 3, "field cannot be negative");
  l.arg_check(0 < w, 4, "width must be positive");
  if f + w > 64 {
    luaL_error!(l, "trying to access non-existent bits");
  }

  let n = n as u64;
  let r = r as u64;
  let f = f as u32;
  let w = w as u32;

  // cpp lintlib.cpp:465 的 `0xFFFFFFFFFFFFFFFFULL >> (64 - w)`：本函数已在
  // 上方把 w 收口到 [1, 64]（`0 < w` 且 `f + w <= 64`、`f >= 0`），故与
  // 共享的 `mask64`（对 w<=0 返回 0、w>=64 返回全 1 的加固版）结果一致；
  // 复用共享实现可避免两份 mask64 边界语义漂移。
  let base_mask = mask64(w as i32);
  let replacement = (r & base_mask) << f;
  let mask = u64::MAX ^ (base_mask << f);

  lua_pushinteger_64(l, ((n & mask) | replacement) as i64);

  1
}

lua_lib_fn!(pub fn int64_replace @ref, int64_replace_arm);
