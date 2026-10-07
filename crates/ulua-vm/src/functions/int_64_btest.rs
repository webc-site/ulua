use crate::{
  functions::int_64_shared::int64_fold, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v35 收形后
/// 体内全经安全门面/已收形被调，无裸操作，降为安全 `fn`）：
/// `l` 须处于受保护帧：`get_top` 取实参数 n，对索引 1..=n 逐个 `check_integer_64`
/// （任一非整数即抛错回退，按位与累加）；`push_boolean` 写回结果需 `top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lintlib.cpp:484
pub fn int64_btest(l: &mut LuaState) -> i32 {
  let acc = int64_fold(l, u64::MAX, |acc, x| acc & x);
  l.push_boolean(acc != 0);
  1
}

lua_lib_fn!(pub fn int64_btest @ref, int64_btest_arm);
