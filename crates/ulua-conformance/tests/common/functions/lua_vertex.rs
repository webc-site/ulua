use ulua_vm::{functions::lua_l_checkvector::lua_l_checkvector, records::lua_state::LuaState};

use crate::common::functions::{lua_vec_2_get::lua_vec_2_get, lua_vertex_push::lua_vertex_push};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vertex(l: *mut LuaState) -> i32 {
  // Safety: `l` 为本用例存活的 LuaState；参数 1 为 vector 时 `lua_l_checkvector` 返回
  // 其 3 分量缓冲指针（失配按 cpp 抛 Lua 错误）。
  let pos = unsafe { lua_l_checkvector(l, 1) };
  // Safety: 同上——参数 2 为 vector 时返回其 3 分量缓冲指针。
  let normal = unsafe { lua_l_checkvector(l, 2) };
  // Safety: 同上——`lua_vec_2_get` 校验参数 3 为 Vec2 userdata 并返回其数据指针。
  let uv = unsafe { lua_vec_2_get(l, 3) };

  // `lua_vertex_push` 是 safe 门面：新建 Vertex userdata 并返回其数据指针。
  let data = lua_vertex_push(l);

  // Safety: `data` 为刚新建 userdata 的数据指针可写，`pos` 为刚校验的 3 分量缓冲可读。
  unsafe {
    (*data).pos[0] = *pos;
    (*data).pos[1] = *pos.add(1);
    (*data).pos[2] = *pos.add(2);
  }

  // Safety: 同上——`normal` 为刚校验的 3 分量缓冲可读。
  unsafe {
    (*data).normal[0] = *normal;
    (*data).normal[1] = *normal.add(1);
    (*data).normal[2] = *normal.add(2);
  }

  // Safety: 同上——`uv` 指向存活 Vec2 数据，两分量可读。
  unsafe {
    (*data).uv[0] = (*uv).x;
    (*data).uv[1] = (*uv).y;
  };

  1
}
