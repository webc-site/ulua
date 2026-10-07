use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

use crate::common::functions::{
  lua_vec_2_get::lua_vec_2_get,
  lua_vertex_get::lua_vertex_get,
  safe_api::{l_checkvector, state_mut},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vertex_newindex(l: *mut LuaState) -> i32 {
  // `lua_vertex_get` 是 safe 门面：校验参数 1 为 Vertex userdata 并返回其数据指针。
  let v = lua_vertex_get(l, 1);
  let name = state_mut(l).check_str(2);

  if name == "pos" {
    // 参数 3 为 vector（失配抛 Lua 错误），取 3 分量切片。
    let pos = l_checkvector(l, 3);
    // Safety: `v` 指向存活 Vertex 数据可写，`pos` 三分量可读。
    unsafe {
      (*v).pos[0] = pos[0];
      (*v).pos[1] = pos[1];
      (*v).pos[2] = pos[2];
    }
  } else if name == "normal" {
    // 参数 3 为 vector（失配抛 Lua 错误），取 3 分量切片。
    let normal = l_checkvector(l, 3);
    // Safety: `v` 指向存活 Vertex 数据可写，`normal` 三分量可读。
    unsafe {
      (*v).normal[0] = normal[0];
      (*v).normal[1] = normal[1];
      (*v).normal[2] = normal[2];
    }
  } else if name == "uv" {
    // `lua_vec_2_get` 校验参数 3 为 Vec2 userdata 并交回数据指针。
    let uv = lua_vec_2_get(l, 3);
    // Safety: `uv` 指向存活 Vec2 数据（两分量可读），`v` 存活可写。
    unsafe {
      (*v).uv[0] = (*uv).x;
      (*v).uv[1] = (*uv).y;
    }
  } else {
    // Safety: 按 cpp 抛「不可写成员」Lua 错误（`l` 存活、格式串为已校验的 `name`），
    // 该调用不返回。
    unsafe { luaL_error!(&mut *l, "{name} is not a writable member of vertex") }
  }

  0
}
