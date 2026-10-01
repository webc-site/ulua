use core::{ffi::c_int, mem::size_of};

use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

use crate::common::{
  functions::{
    lua_vec_2_push::lua_vec_2_push,
    lua_vertex_get::lua_vertex_get,
    safe_api::{pushvector3, state_mut},
  },
  records::vertex::Vertex,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vertex_index(l: *mut LuaState) -> c_int {
  // `lua_vertex_get` 是 safe 门面：校验参数 1 为 Vertex userdata 并返回其数据指针。
  let v = lua_vertex_get(l, 1);
  let name = state_mut(l).check_str(2);

  if name == "pos" {
    // `v` 指向存活 Vertex userdata 的数据，pos 三分量可读。
    // Safety: userdata 数据指针由 lua_vertex_get 的 tag 校验兜底，仅本行读一次。
    let pos = unsafe { (*v).pos };
    pushvector3(l, pos[0], pos[1], pos[2]);
    return 1;
  }

  if name == "normal" {
    // Safety: 同上——normal 三分量可读。
    let normal = unsafe { (*v).normal };
    pushvector3(l, normal[0], normal[1], normal[2]);
    return 1;
  }

  if name == "uv" {
    // Safety: 同上——uv 两分量可读。
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
    // 压入静态 `size_of` 结果，无指针解引用。
    state_mut(l).push_number(size_of::<Vertex>() as f64);
    return 1;
  }

  // 末分支按 cpp 抛 Lua 错误（宏内为 C ABI `luaL_error`，格式串为已校验的
  // `name`）；该调用不返回。
  // Safety: `l` 存活；`luaL_error` 以 long-jump 终止本回调。
  unsafe { luaL_error!(l, "{name} is not a valid member of vertex") }
}
