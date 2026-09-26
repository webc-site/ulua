use ulua_vm::{
  functions::lua_l_checkvector::lua_l_checkvector,
  macros::{lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

use crate::common::functions::{
  cstr_text::cstr_text, lua_vec_2_get::lua_vec_2_get, lua_vertex_get::lua_vertex_get,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vertex_newindex(l: *mut LuaState) -> i32 {
  // `lua_vertex_get` 是 safe 门面：校验参数 1 为 Vertex userdata 并返回其数据指针。
  let v = lua_vertex_get(l, 1);
  // Safety: `l` 为本用例存活的 LuaState；`luaL_checkstring!` 在参数 2 为串时返回
  // NUL 结尾缓冲（否则抛 Lua 错误）。
  let name_ptr = unsafe { luaL_checkstring!(l, 2) };
  // Safety: 上一行的宏契约保证 `name_ptr` 为 NUL 结尾合法 C 串。
  let name = unsafe { cstr_text(name_ptr.cast()) };

  if name == "pos" {
    // Safety: `l` 存活；参数 3 为 vector 时返回其 3 分量缓冲指针（否则抛 Lua 错误）。
    let pos = unsafe { lua_l_checkvector(l, 3) };
    // Safety: `v` 指向存活 Vertex 数据可写，`pos` 三分量可读。
    unsafe {
      (*v).pos[0] = *pos;
      (*v).pos[1] = *pos.add(1);
      (*v).pos[2] = *pos.add(2);
    }
  } else if name == "normal" {
    // Safety: `l` 存活；参数 3 为 vector 时返回其 3 分量缓冲指针（否则抛 Lua 错误）。
    let normal = unsafe { lua_l_checkvector(l, 3) };
    // Safety: `v` 指向存活 Vertex 数据可写，`normal` 三分量可读。
    unsafe {
      (*v).normal[0] = *normal;
      (*v).normal[1] = *normal.add(1);
      (*v).normal[2] = *normal.add(2);
    }
  } else if name == "uv" {
    // Safety: `l` 存活；`lua_vec_2_get` 校验参数 3 为 Vec2 userdata 并交回数据指针。
    let uv = unsafe { lua_vec_2_get(l, 3) };
    // Safety: `uv` 指向存活 Vec2 数据（两分量可读），`v` 存活可写。
    unsafe {
      (*v).uv[0] = (*uv).x;
      (*v).uv[1] = (*uv).y;
    }
  } else {
    // Safety: 按 cpp 抛「不可写成员」Lua 错误（`l` 存活、格式串为已校验的 `name`），
    // 该调用不返回。
    unsafe { luaL_error!(l, "{name} is not a writable member of vertex") }
  }

  0
}
