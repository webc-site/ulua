use crate::{
  functions::{cstr_bytes, lua_l_typename::lua_l_typename},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// base 库 `typeof` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （1 号槽有值否则抛错回退；`lua_l_typename` 读类型名、名称串压栈可分配/GC）。
/// cpp/VM/src/lbaselib.cpp:208 luaB_typeof。
pub fn lua_b_typeof(l: &mut LuaState) -> i32 {
  l.check_any(1);
  // SAFETY: `l` 存活（引用形保证）；`lua_l_typename` 的 `# Safety` 其余前提（1 号槽
  // 为合法正索引）由库函数约定与上方 `check_any` 成立；返回的 `name` 指向存活
  // TString 的 NUL 串数据（本调用内被压栈持有），cstr_bytes 只读切片视图。
  let name = unsafe { lua_l_typename(l.as_mut_ptr(), 1) };
  // SAFETY: 同上，name 在本表达式求值期间存活。
  l.push_bytes(unsafe { cstr_bytes(name) });
  1
}

lua_lib_fn!(pub fn lua_b_typeof @ref, lua_b_typeof_arm);
