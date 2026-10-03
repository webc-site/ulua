use crate::{
  functions::int_64_shared::INT64_SHIFT_ABS_MAX, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v35 收形后
/// 体内全经安全门面方法，无裸操作，降为安全 `fn`）：
/// `l` 须处于受保护帧：`check_integer_64(1)`、`(2)` 要求索引 1、2 存在且可转成
/// i64（否则抛错回退）；`push_integer_64` 写回 1 结果需 `top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lintlib.cpp:395
pub fn int64_arshift(l: &mut LuaState) -> i32 {
  let n = l.check_integer_64(1);
  let i = l.check_integer_64(2);

  if (-INT64_SHIFT_ABS_MAX..=INT64_SHIFT_ABS_MAX).contains(&i) {
    l.push_integer_64(if i < 0 {
      ((n as u64) << (-i)) as i64
    } else {
      n >> i
    });
  } else if i < -INT64_SHIFT_ABS_MAX {
    l.push_integer_64(0);
  } else {
    l.push_integer_64(if n < 0 { -1 } else { 0 });
  }

  1
}

lua_lib_fn!(pub fn int64_arshift @ref, int64_arshift_arm);
