use ulua_vm::{
  functions::{
    lua_l_checkvector::lua_l_checkvector,
    lua_pushvector_lapi::{
      lua_pushvector_lua_state_f32_f32_f32, lua_pushvector_lua_state_f32_f32_f32_f32,
    },
  },
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  records::lua_state::LuaState,
};
pub(crate) fn lua_vector_cross(l: *mut LuaState) -> i32 {
  // Safety: `l` 为本用例存活的 LuaState；参数 1 为 vector 时 `lua_l_checkvector` 返回
  // 其前 3 分量缓冲指针（失配按 cpp 抛 Lua 错误）。
  let a = unsafe { lua_l_checkvector(l, 1) };
  // Safety: 同上——参数 2 为 vector 时返回其前 3 分量缓冲指针。
  let b = unsafe { lua_l_checkvector(l, 2) };

  // Safety: 上两步保证两指针各指向存活 vector 的前 3 分量，`add(0..=2)` 均在界内。
  let (x, y, z) = unsafe {
    (
      (*a.add(1)) * (*b.add(2)) - (*a.add(2)) * (*b.add(1)),
      (*a.add(2)) * (*b.add(0)) - (*a.add(0)) * (*b.add(2)),
      (*a.add(0)) * (*b.add(1)) - (*a.add(1)) * (*b.add(0)),
    )
  };

  if LUA_VECTOR_SIZE == 4 {
    // Safety: `l` 存活；按构建期 4 分量布局压入叉积结果（第四分量置 0）。
    unsafe { lua_pushvector_lua_state_f32_f32_f32_f32(l, x, y, z, 0.0) };
  } else {
    // Safety: `l` 存活；按构建期 3 分量布局压入叉积结果。
    unsafe { lua_pushvector_lua_state_f32_f32_f32(l, x, y, z) };
  }

  1
}
