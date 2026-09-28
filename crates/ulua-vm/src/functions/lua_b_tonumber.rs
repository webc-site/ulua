use ulua_common::strtoull_shim::parse_c_ull;

use crate::{
  functions::{
    lua_l_optinteger::lua_l_optinteger, lua_o_str_2_d::num_parse_tail, lua_tonumberx::lua_tonumberx,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于可抛错的受保护帧；栈 index 1 为待转值、index 2 为可选进制
/// （`luaL_optinteger`/`checkany`/`check_bytes` 读槽，非串实参与进制越界均抛错发散）。
/// cpp `lbaselib.cpp:39`。
pub(crate) unsafe fn lua_b_tonumber(l: *mut LuaState) -> i32 {
  unsafe {
    let base = lua_l_optinteger(l, 2, 10);

    if base == 10 {
      // standard conversion
      if let Some(n) = lua_tonumberx(l, 1) {
        (*l).push_number(n);
        return 1;
      }
      (*l).check_any(1); // error if we don't have any argument
    } else {
      let bytes = (*l).check_bytes(1);
      (*l).arg_check((2..=36).contains(&base), 2, "base out of range");

      // cpp `strtoull(s1, &s2, base)` 按 NUL 结尾读取：内嵌 `\0` 即串终点（`endptr`
      // 落于其上、`*s2 == '\0'` 判成功），故取首 NUL 前的前缀切片作为解析域。切片
      // 下 `consumed` 即 `s2 - s1`，切片末尾（`get(end) == None`）即 `*s2 == '\0'`，
      // 与 endptr 语义逐位对应。
      // memchr 与 `iter().position(|&c| c == 0)` 逐位等价：同为首 NUL 下标，
      // 无 NUL 时 `unwrap_or` 收口为全长；memchr 走 SIMD（形态同 lua_l_traceback）。
      let s = &bytes[..memchr::memchr(0, bytes).unwrap_or(bytes.len())];

      let (n, consumed) = parse_c_ull(s, base as u32);
      if consumed != 0 {
        // at least one valid digit?（consumed == 0 ⇔ endptr == nptr，即无转换）
        // 尾随空白跳过 + 「无非法尾字符」判定与 cpp 的 `isspace(*s2)`/`*s2 == '\0'`
        // 循环同构，复用 `luaO_str2d` 的收尾件（NUL 非 `isspace`，两者等价）。
        if let Some(n) = num_parse_tail(s, n, consumed) {
          (*l).push_number(n as f64);
          return 1;
        }
      }
    }

    (*l).push_nil(); // else not a number
    1
  }
}

lua_lib_fn!(pub(crate) fn lua_b_tonumber, lua_b_tonumber_arm);
