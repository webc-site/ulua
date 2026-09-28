use crate::{
  functions::int_64_shared::INT64_SHIFT_ABS_MAX, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkinteger_64(l,1)`、`(l,2)` 要求索引 1、2 存在且可转成
/// i64（否则抛错回退）；`lua_pushinteger_64` 写回 1 结果需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lintlib.cpp:395
pub unsafe fn int64_arshift(l: *mut LuaState) -> i32 {
  // SAFETY: 本块仅做 C 臂 `l: *mut LuaState` → `&mut` 的边界转换；后续读槽与压栈
  // 皆走 safe 方法族。
  let l = unsafe { &mut *l };
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

lua_lib_fn!(pub fn int64_arshift, int64_arshift_arm);
