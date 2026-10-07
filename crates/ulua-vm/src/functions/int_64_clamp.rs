use crate::{
  functions::lua_pushinteger_64::lua_pushinteger_64, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v35 收形后
/// 体内全经安全门面/已收形被调，无裸操作，降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧：栈 1/2/3 号位分别经 `check_integer_64` 取被夹值/下界/上界
/// （非整数抛错，`mi <= mx` 由 `arg_check` 校验），结果经 `lua_pushinteger_64` 写回。
/// cpp/VM/src/lintlib.cpp:466。
pub fn int64_clamp(l: &mut LuaState) -> i32 {
  let a = l.check_integer_64(1);
  let mi = l.check_integer_64(2);
  let mx = l.check_integer_64(3);

  l.arg_check(mi <= mx, 3, "max must be greater than or equal to min");

  if a < mi {
    lua_pushinteger_64(l, mi);
  } else if a > mx {
    lua_pushinteger_64(l, mx);
  } else {
    lua_pushinteger_64(l, a);
  }

  1
}

lua_lib_fn!(pub fn int64_clamp @ref, int64_clamp_arm);
