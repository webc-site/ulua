use crate::{
  functions::lua_pushinteger_64::lua_pushinteger_64, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v35 收形后
/// 体内全经安全门面/已收形被调，无裸操作，降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧且栈 index 1 为可转 `double` 的值（`check_number` 读槽并可抛错），
/// `lua_pushinteger_64`/`push_nil` 可能扩栈，须在受保护帧内由 C 侧调入。cpp `lintlib.cpp:19`。
pub fn int64_create(l: &mut LuaState) -> i32 {
  let x = l.check_number(1);

  // C++: if (x >= -9223372036854775808.0 && x < 9223372036854775808.0)
  // These constants are exactly -2^63 and 2^63.
  if (-9223372036854775808.0..9223372036854775808.0).contains(&x) {
    let val = x as i64;

    // C++: if (((double)l) == x)
    if (val as f64) == x {
      lua_pushinteger_64(l, val);
      return 1;
    }
  }

  l.push_nil();
  1
}

lua_lib_fn!(pub fn int64_create @ref, int64_create_arm);
