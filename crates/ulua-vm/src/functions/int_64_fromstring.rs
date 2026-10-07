use crate::{
  functions::{
    lua_l_optinteger::lua_l_optinteger, lua_o_str_2_l::lua_o_str_2_l,
    lua_pushinteger_64::lua_pushinteger_64,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载；r16-v35 收形后
/// 读槽/校验/压栈全经安全门面与被调，无裸操作，本体降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧：栈 1 号位为字符串（`check_bytes` 返回本帧存活字节切片，
/// 非串实参抛 "string expected" 发散），2 号位可选基数经 `lua_l_optinteger`/`arg_check`
/// 校验落在 2..=36；`lua_pushinteger_64`/`push_nil` 写回可分配/GC。
/// cpp/VM/src/lintlib.cpp:37 int64_fromstring。
pub fn int64_fromstring(l: &mut LuaState) -> i32 {
  // 锚定形×收形交汇点（p28 A 类「二次派窗」判例，形制同 r16-v33 的 lua_b_tonumber）：
  // `check_bytes` 出参锚定 `&mut l` 借用，紧随的 `lua_l_optinteger`/`arg_check` 亦经
  // `&mut l`，两者不可重叠（裸指针时代 rustc 不查重叠借用，收形后即 E0499）。首次派窗
  // 只充当 cpp `luaL_checkstring` 的先位抛错件——「string expected」必先于取基数与
  // 「base out of range」，可观察序逐位不变——借用止于取长；`arg_check` 落定后二次派窗
  // 直达解析（同槽同值，观测等价），长度快照在此充当不变量件。
  let len = l.check_bytes(1).len();
  let base = lua_l_optinteger(l, 2, 10);
  l.arg_check((2..=36).contains(&base), 2, "base out of range");

  let s = l.check_bytes(1);
  debug_assert_eq!(s.len(), len);

  match lua_o_str_2_l(s, base) {
    Some(result) => lua_pushinteger_64(l, result),
    None => l.push_nil(),
  }

  1
}

lua_lib_fn!(pub fn int64_fromstring @ref, int64_fromstring_arm);
