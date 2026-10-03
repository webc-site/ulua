use ulua_common::strtoull_shim::parse_c_ull;

use crate::{
  functions::{
    lua_l_optinteger::lua_l_optinteger, lua_o_str_2_d::num_parse_tail, lua_tonumberx::lua_tonumberx,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v29 收形后
/// 体内全经安全门面/已收形被调，无裸操作，降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧；栈 index 1 为待转值、index 2 为可选进制
/// （`lua_l_optinteger`/`check_any`/`check_bytes` 读槽，非串实参与进制越界均抛错发散）。
/// cpp `lbaselib.cpp:39`。
pub(crate) fn lua_b_tonumber(l: &mut LuaState) -> i32 {
  let base = lua_l_optinteger(l, 2, 10);

  if base == 10 {
    // standard conversion
    if let Some(n) = lua_tonumberx(l, 1) {
      l.push_number(n);
      return 1;
    }
    l.check_any(1); // error if we don't have any argument
  } else {
    // r16-v33 锚定形×收形交汇点（p28 A 类「二次派窗」判例）：`check_bytes` 出参锚定
    // `&mut l` 借用，`arg_check` 亦经 `&mut l`，二者不可重叠。首次派窗只取景校验——
    // 兼作 cpp `luaL_checkstring` 的先位抛错（"string expected" 必先于
    // "base out of range"，可观察序逐位不变），借用止于取长；随后校验落定再二次派窗
    // 直达解析（同槽同值，观测等价）。
    let len = l.check_bytes(1).len();
    l.arg_check((2..=36).contains(&base), 2, "base out of range");

    let bytes = l.check_bytes(1);

    // cpp `strtoull(s1, &s2, base)` 按 NUL 结尾读取：内嵌 `\0` 即串终点（`endptr`
    // 落于其上、`*s2 == '\0'` 判成功），故取首 NUL 前的前缀切片作为解析域。切片
    // 下 `consumed` 即 `s2 - s1`，切片末尾（`get(end) == None`）即 `*s2 == '\0'`，
    // 与 endptr 语义逐位对应。
    // memchr 与 `iter().position(|&c| c == 0)` 逐位等价：同为首 NUL 下标，
    // 无 NUL 时 `unwrap_or` 收口为全长；memchr 走 SIMD（形态同 lua_l_traceback）。
    let s = &bytes[..memchr::memchr(0, bytes).unwrap_or(len)];

    let (n, consumed) = parse_c_ull(s, base as u32);
    if consumed != 0 {
      // at least one valid digit?（consumed == 0 ⇔ endptr == nptr，即无转换）
      // 尾随空白跳过 + 「无非法尾字符」判定与 cpp 的 `isspace(*s2)`/`*s2 == '\0'`
      // 循环同构，复用 `luaO_str2d` 的收尾件（NUL 非 `isspace`，两者等价）。
      if let Some(n) = num_parse_tail(s, n, consumed) {
        l.push_number(n as f64);
        return 1;
      }
    }
  }

  l.push_nil(); // else not a number
  1
}

lua_lib_fn!(pub(crate) fn lua_b_tonumber @ref, lua_b_tonumber_arm);
