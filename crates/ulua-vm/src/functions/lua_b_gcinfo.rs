use crate::{
  enums::lua_gc_op::LuaGcOp, functions::lua_gc::lua_gc, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// base 库 `gcinfo` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （`lua_gc(LUA_GCCOUNT)` 读 GC 统计、可步进 GC），结果压栈需栈顶留 1 空槽；
/// 处于受保护帧。cpp VM/src/lbaselib.cpp:194
pub fn lua_b_gcinfo(l: &mut LuaState) -> i32 {
  // SAFETY: `l` 存活（引用形保证）；`lua_gc` 的 `# Safety` 其余前提（受保护帧、
  // 常量 op 合法）由库函数调用约定与 `LUA_GCCOUNT` 字面量成立。
  let n = unsafe { lua_gc(l.as_mut_ptr(), LuaGcOp::Count as i32, 0) };
  l.push_integer(n);
  1
}

lua_lib_fn!(pub fn lua_b_gcinfo @ref, lua_b_gcinfo_arm);
