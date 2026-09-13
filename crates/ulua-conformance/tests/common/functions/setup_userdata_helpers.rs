use core::{
  ffi::{CStr, c_int},
  ptr::null,
};

use ulua_vm::{
  functions::{
    lua_l_newmetatable::lua_l_newmetatable, lua_pushcclosurek::lua_pushcclosurek,
    lua_pushvalue::lua_pushvalue, lua_setfield::lua_setfield, lua_setreadonly::lua_setreadonly,
    lua_setuserdatametatable::lua_setuserdatametatable,
  },
  macros::{lua_pop::lua_pop, lua_setglobal::lua_setglobal},
  records::lua_state::lua_State,
  type_aliases::lua_c_function::LuaCfunction,
};

use crate::common::{
  functions::{
    lua_vec_2::lua_vec_2, lua_vec_2_get::lua_vec_2_get, lua_vec_2_index::lua_vec_2_index,
    lua_vec_2_namecall::lua_vec_2_namecall, lua_vec_2_newindex::lua_vec_2_newindex,
    lua_vec_2_push::lua_vec_2_push, lua_vertex::lua_vertex, lua_vertex_index::lua_vertex_index,
    lua_vertex_namecall::lua_vertex_namecall, lua_vertex_newindex::lua_vertex_newindex,
  },
  records::{vec_2_conformance_ir_hooks::Vec2, vertex::Vertex},
};
#[inline]
unsafe fn vec2_binop(l: *mut lua_State, op: impl Fn(f32, f32, f32, f32) -> (f32, f32)) -> c_int {
  unsafe {
    let a = lua_vec_2_get(l, 1);
    let b = lua_vec_2_get(l, 2);
    let data = lua_vec_2_push(l);
    let (x, y) = op((*a).x, (*a).y, (*b).x, (*b).y);
    (*data).x = x;
    (*data).y = y;
    1
  }
}

unsafe extern "C-unwind" fn vec2_add(l: *mut lua_State) -> c_int {
  unsafe { vec2_binop(l, |ax, ay, bx, by| (ax + bx, ay + by)) }
}

unsafe extern "C-unwind" fn vec2_sub(l: *mut lua_State) -> c_int {
  unsafe { vec2_binop(l, |ax, ay, bx, by| (ax - bx, ay - by)) }
}

unsafe extern "C-unwind" fn vec2_mul(l: *mut lua_State) -> c_int {
  unsafe { vec2_binop(l, |ax, ay, bx, by| (ax * bx, ay * by)) }
}

unsafe extern "C-unwind" fn vec2_div(l: *mut lua_State) -> c_int {
  unsafe { vec2_binop(l, |ax, ay, bx, by| (ax / bx, ay / by)) }
}

unsafe extern "C-unwind" fn vec2_unm(l: *mut lua_State) -> c_int {
  unsafe {
    let a = lua_vec_2_get(l, 1);
    let data = lua_vec_2_push(l);
    (*data).x = -(*a).x;
    (*data).y = -(*a).y;
    1
  }
}

unsafe fn register_methods(l: *mut lua_State, methods: &[(&CStr, LuaCfunction)]) {
  unsafe {
    for &(name, func) in methods {
      lua_pushcclosurek(l, func, null(), 0, None);
      lua_setfield(l, -2, name.as_ptr());
    }
  }
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn setup_userdata_helpers(l: *mut lua_State) {
  unsafe {
    lua_l_newmetatable(l, c"vec2".as_ptr());
    lua_pushvalue(l, -1);
    lua_setuserdatametatable(l, Vec2::TAG);

    let vec2_methods: [(&CStr, LuaCfunction); 8] = [
      (c"__index", Some(lua_vec_2_index)),
      (c"__newindex", Some(lua_vec_2_newindex)),
      (c"__namecall", Some(lua_vec_2_namecall)),
      (c"__add", Some(vec2_add)),
      (c"__sub", Some(vec2_sub)),
      (c"__mul", Some(vec2_mul)),
      (c"__div", Some(vec2_div)),
      (c"__unm", Some(vec2_unm)),
    ];
    register_methods(l, &vec2_methods);

    lua_setreadonly(l, -1, 1);

    lua_pushcclosurek(l, Some(lua_vec_2), c"vec2".as_ptr(), 0, None);
    lua_setglobal(l, c"vec2".as_ptr());

    lua_pop(l, 1);

    lua_l_newmetatable(l, c"vertex".as_ptr());
    lua_pushvalue(l, -1);
    lua_setuserdatametatable(l, Vertex::TAG);

    let vertex_methods: [(&CStr, LuaCfunction); 3] = [
      (c"__index", Some(lua_vertex_index)),
      (c"__newindex", Some(lua_vertex_newindex)),
      (c"__namecall", Some(lua_vertex_namecall)),
    ];
    register_methods(l, &vertex_methods);

    lua_setreadonly(l, -1, 1);

    lua_pushcclosurek(l, Some(lua_vertex), c"vertex".as_ptr(), 0, None);
    lua_setglobal(l, c"vertex".as_ptr());

    lua_pop(l, 1);
  }
}
