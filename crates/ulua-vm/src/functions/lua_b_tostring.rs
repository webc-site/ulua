use crate::{
  functions::lua_l_tolstring::lua_l_tolstring_ref, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// base 库 `tostring` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （1 号槽有值否则抛错回退；受保护帧、栈顶留 1 空槽；`luaL_tolstring` 可回跑
/// `__tostring` 元方法、可抛错/触发 GC，结果串压栈即目的）。
/// cpp VM/src/lbaselib.cpp:405
pub fn lua_b_tostring(l: &mut LuaState) -> i32 {
  l.check_any(1);
  // SAFETY: `l` 存活（引用形保证）；`lua_l_tolstring_ref` 的 `# Safety` 其余前提
  // （1 号槽为合法正索引、受保护帧）由库函数约定成立。结果串压栈即目的（返回 1
  // 即栈顶该串），切片引用不外传。
  let _ = unsafe { lua_l_tolstring_ref(l.as_mut_ptr(), 1) };
  1
}

lua_lib_fn!(pub fn lua_b_tostring @ref, lua_b_tostring_arm);
