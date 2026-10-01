use crate::{
  enums::lua_type::LuaType, functions::lua_rawgeti::lua_rawgeti, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// `ipairs` 迭代步进核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （2 号槽为整数下标、1 号槽为 table，不符即抛错；受保护帧）。
/// cpp `lbaselib.cpp:238`。
pub fn lua_b_inext(l: &mut LuaState) -> i32 {
  let mut i = l.check_integer(2);
  l.check_type(1, LuaType::Table);
  i += 1; // next value
  l.push_integer(i);
  // SAFETY: `l` 存活（引用形保证）；`lua_rawgeti` 的 `# Safety` 其余前提（1 号槽为
  // 合法正索引且上方已校验为 table）由库函数约定与 `check_type` 成立。
  lua_rawgeti(l, 1, i);

  if l.is_nil(-1) { 0 } else { 2 }
}

lua_lib_fn!(pub fn lua_b_inext @ref, lua_b_inext_arm);
