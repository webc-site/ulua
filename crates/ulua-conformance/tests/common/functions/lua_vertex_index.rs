use core::{ffi::c_int, mem::size_of};

use ulua_vm::{
  functions::lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32,
  macros::lua_l_error::luaL_error, records::lua_state::LuaState,
};

use crate::common::{
  functions::{lua_vec_2_push::lua_vec_2_push, lua_vertex_get::lua_vertex_get},
  records::vertex::Vertex,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vertex_index(l: *mut LuaState) -> c_int {
  // `lua_vertex_get` 是 safe 门面：校验参数 1 为 Vertex userdata 并返回其数据指针。
  let v = lua_vertex_get(l, 1);
  let name = unsafe { (*l).check_str(2) };

  if name == "pos" {
    // Safety: `v` 指向存活 Vertex userdata 的数据，pos 三分量可读。
    let pos = unsafe { (*v).pos };
    // Safety: `l` 存活；pushvector 只接收已取出的三个分量值。
    unsafe { lua_pushvector_lua_state_f32_f32_f32(l, pos[0], pos[1], pos[2]) };
    return 1;
  }

  if name == "normal" {
    // Safety: `v` 指向存活 Vertex userdata 的数据，normal 三分量可读。
    let normal = unsafe { (*v).normal };
    // Safety: `l` 存活；pushvector 只接收已取出的三个分量值。
    unsafe { lua_pushvector_lua_state_f32_f32_f32(l, normal[0], normal[1], normal[2]) };
    return 1;
  }

  if name == "uv" {
    // Safety: `v` 指向存活 Vertex userdata 的数据，uv 两分量可读。
    let (u, vv) = unsafe { ((*v).uv[0], (*v).uv[1]) };
    // `lua_vec_2_push` 是 safe 门面：新建 Vec2 userdata 并返回其数据指针。
    let uv = lua_vec_2_push(l);
    // Safety: `uv` 为刚新建 userdata 的数据指针，可写两分量。
    unsafe {
      (*uv).x = u;
      (*uv).y = vv;
    };
    return 1;
  }

  if name == "sizeof" {
    // Safety: `l` 存活；压入静态 `size_of` 结果，无指针解引用。
    unsafe { (*l).push_number(size_of::<Vertex>() as f64) };
    return 1;
  }

  // Safety: 末分支按 cpp 抛 Lua 错误（宏内为 C ABI `luaL_error`，`l` 存活、格式串为
  // 已校验的 `name`）；该调用不返回。
  unsafe { luaL_error!(l, "{name} is not a valid member of vertex") }
}
