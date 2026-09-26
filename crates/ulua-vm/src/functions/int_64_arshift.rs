use crate::{
  functions::{
    int_64_shared::INT64_SHIFT_ABS_MAX, lua_l_checkinteger_64::lua_l_checkinteger_64,
    lua_pushinteger_64::lua_pushinteger_64,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkinteger_64(l,1)`、`(l,2)` 要求索引 1、2 存在且可转成
/// i64（否则抛错回退）；`lua_pushinteger_64` 写回 1 结果需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lintlib.cpp:395
pub unsafe extern "C-unwind" fn int64_arshift(l: *mut LuaState) -> i32 {
  unsafe {
    let n = lua_l_checkinteger_64(l, 1);
    let i = lua_l_checkinteger_64(l, 2);

    if (-INT64_SHIFT_ABS_MAX..=INT64_SHIFT_ABS_MAX).contains(&i) {
      lua_pushinteger_64(
        l,
        if i < 0 {
          ((n as u64) << (-i)) as i64
        } else {
          n >> i
        },
      );
    } else if i < -INT64_SHIFT_ABS_MAX {
      lua_pushinteger_64(l, 0);
    } else {
      lua_pushinteger_64(l, if n < 0 { -1 } else { 0 });
    }

    1
  }
}
