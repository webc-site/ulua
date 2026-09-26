use crate::{
  functions::{int_64_shared::int64_fold, lua_pushboolean::lua_pushboolean},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`lua_gettop` 取实参数 n，对索引 1..=n 逐个 `luaL_checkinteger_64`
/// （任一非整数即抛错回退，按位与累加）；`lua_pushboolean` 写回结果需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lintlib.cpp:484
pub unsafe fn int64_btest(l: *mut LuaState) -> i32 {
  // Safety: 契约同上，int64_fold 逐实参折叠后经 pushboolean 写回非零判定
  unsafe {
    lua_pushboolean(l, (int64_fold(l, u64::MAX, |acc, x| acc & x) != 0) as i32);
    1
  }
}

lua_lib_fn!(pub fn int64_btest, int64_btest_arm);
