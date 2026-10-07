use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{
  lua_vec_2_get::lua_vec_2_get, lua_vertex_push::lua_vertex_push, safe_api::l_checkvector,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vertex(l: *mut LuaState) -> i32 {
  // 参数 1/2 各为 vector（失配按 cpp 抛 Lua 错误），取 3 分量切片。
  let pos = l_checkvector(l, 1);
  let normal = l_checkvector(l, 2);
  // `lua_vec_2_get` 校验参数 3 为 Vec2 userdata 并返回其数据指针。
  let uv = lua_vec_2_get(l, 3);

  // `lua_vertex_push` 是 safe 门面：新建 Vertex userdata 并返回其数据指针。
  let data = lua_vertex_push(l);

  // Safety: `data` 为刚新建 userdata 的数据指针可写，两份 vector 切片为刚校验的
  // 3 分量缓冲、`uv` 指向存活 Vec2 数据（两分量可读）。
  unsafe {
    (*data).pos[0] = pos[0];
    (*data).pos[1] = pos[1];
    (*data).pos[2] = pos[2];
    (*data).normal[0] = normal[0];
    (*data).normal[1] = normal[1];
    (*data).normal[2] = normal[2];
    (*data).uv[0] = (*uv).x;
    (*data).uv[1] = (*uv).y;
  }

  1
}
