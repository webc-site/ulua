use crate::{
  enums::lua_type::LuaType, functions::lua_rawset::lua_rawset, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// base 库 `rawset` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （1 号槽为 table、2/3 号槽有值，不符即抛错；受保护帧、`lua_rawset` 弹两槽并可 GC）。
/// cpp `lbaselib.cpp:175`。
pub fn lua_b_rawset(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);
  l.check_any(2);
  l.check_any(3);
  l.set_top(3);
  // SAFETY: `l` 存活（引用形保证）；`lua_rawset` 的 `# Safety` 其余前提（1 号槽为
  // 合法正索引且上方已校验为 table、栈顶 key/value 就位）由库函数约定与本函数前四句成立。
  unsafe { lua_rawset(l.as_mut_ptr(), 1) };
  1
}

lua_lib_fn!(pub fn lua_b_rawset @ref, lua_b_rawset_arm);
