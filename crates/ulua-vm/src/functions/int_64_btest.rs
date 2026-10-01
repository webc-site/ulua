use crate::{
  functions::int_64_shared::int64_fold, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`lua_gettop` 取实参数 n，对索引 1..=n 逐个 `luaL_checkinteger_64`
/// （任一非整数即抛错回退，按位与累加）；`lua_pushboolean` 写回结果需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lintlib.cpp:484
pub unsafe fn int64_btest(l: *mut LuaState) -> i32 {
  // SAFETY: 本块仅做 C 臂 `l: *mut LuaState` → `&mut` 的边界转换；后续折叠读槽与
  // 压布尔皆走 safe 方法族。
  let l = unsafe { &mut *l };
  let acc = int64_fold(l, u64::MAX, |acc, x| acc & x);
  l.push_boolean(acc != 0);
  1
}

lua_lib_fn!(pub fn int64_btest, int64_btest_arm);
