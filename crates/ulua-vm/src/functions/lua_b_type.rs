use crate::{
  functions::{cstr_bytes, lua_typename::lua_typename},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// base 库 `type` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （1 号槽有值否则抛错回退），类型名串压栈、可分配/GC。
pub fn lua_b_type(l: &mut LuaState) -> i32 {
  l.check_any(1);
  // resulting name doesn't differentiate between userdata types
  let t = l.type_of(1);
  let name = lua_typename(l.as_mut_ptr(), t as i32);
  // SAFETY: `name` 指向类型名静态 NUL 串（lua_typename 纯查表），cstr_bytes 只读切片视图。
  l.push_bytes(unsafe { cstr_bytes(name) });
  1
}

lua_lib_fn!(pub fn lua_b_type @ref, lua_b_type_arm);
