use ulua_vm::{
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checkstack::lua_l_checkstack,
    lua_pushinteger::lua_pushinteger, lua_settop::lua_settop, lua_yield::lua_yield,
  },
  records::lua_state::LuaState,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn multiple_yields(l: *mut LuaState) -> i32 {
  // Safety: `l` 为本用例存活的 LuaState；截断到 1 个实参后校验其为整数（失配按 cpp 抛
  // Lua 错误）并取基值。
  unsafe { lua_settop(l, 1) };
  let base = unsafe { lua_l_checkinteger(l, 1) };

  // Safety: `l` 存活；本步要压「位置 + 值」两个结果，先留 2 个栈位。
  unsafe { lua_l_checkstack(l, 2, "cmultiyield") };

  let pos: i32 = 1;

  // Safety: `l` 存活；压入起始位置与首个 y 值，随后以 1 个结果 yield 回宿主。
  unsafe {
    lua_pushinteger(l, pos);
    lua_pushinteger(l, base + pos);
  }

  // Safety: `l` 存活；栈顶已备好 1 个 yield 值。
  unsafe { lua_yield(l, 1) }
}
