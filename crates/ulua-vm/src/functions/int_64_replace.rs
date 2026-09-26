use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_l_optinteger_64::lua_l_optinteger_64,
    lua_pushinteger_64::lua_pushinteger_64,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_error::luaL_error, mask_64::mask64},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 必须指向当前 int64 binary32 调用的存活 `lua_State`，实参 1..=3 可读且栈顶留有结果空间。
/// cpp lintlib.cpp `int64_replace`：把 r 的低 w 位写入 n 的第 f 位起字段。
pub unsafe extern "C-unwind" fn int64_replace(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧且实参 1..=3 可读可替换槽匹配，块内仅数值替换与压栈
  unsafe {
    let n = lua_l_checkinteger_64(l, 1);
    let r = lua_l_checkinteger_64(l, 2);
    let f = lua_l_checkinteger_64(l, 3);
    let w = lua_l_optinteger_64(l, 4, 1);

    // cpp lintlib.cpp:453：replace 签名为 (value, replacement, field, width)，
    // f 取自第 3 槽，报错 argnum 必须是 3
    luaL_argcheck!(l, (0..=63).contains(&f), 3, "field cannot be negative");
    luaL_argcheck!(l, 0 < w, 4, "width must be positive");
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
}
