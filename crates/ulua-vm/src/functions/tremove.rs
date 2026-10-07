use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_optinteger::lua_l_optinteger, lua_rawseti::lua_rawseti, moveelements::moveelements,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；收形后判型/
/// 取长/取值/落值全经安全门面，`moveelements` 亦为引用形安全核心（r12-w6），体内已无裸操作）：`l`
/// 须处于可抛错的受保护帧，栈 1 号位为 table（`check_type` 校验、非表即抛错发散），故交与
/// `moveelements` 的 srct/dstt=1 槽必为表，且 `pos <= n` 前置保证其 `f <= e + 1` 入约；
/// `lua_l_optinteger` 非整数即抛错发散，`moveelements`/`raw_get_i`/`lua_rawseti` 可触发再哈希与 GC。
/// cpp/VM/src/ltablib.cpp:249 tremove。
pub fn tremove(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);
  let n = l.obj_len(1) as i32;
  let pos = lua_l_optinteger(l, 2, n);

  if !(1 <= pos && pos <= n) {
    return 0; // position is outside bounds: nothing to remove
  }

  l.raw_get_i(1, pos); // result = t[pos]

  // r12-w6 收形：`moveelements` 已前移 `&mut LuaState` 引用形安全核心，直传独占借用
  moveelements(l, 1, 1, pos + 1, n, pos, false);

  l.push_nil();
  lua_rawseti(l, 1, n); // t[n] = nil
  1
}

lua_lib_fn!(pub fn tremove @ref, tremove_arm);
