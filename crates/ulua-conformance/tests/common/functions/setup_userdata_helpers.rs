use core::ffi::c_int;

use ulua_vm::{records::lua_state::LuaState, type_aliases::lua_c_function::LuaCFunction};

use crate::common::{
  functions::{
    lua_vec_2::lua_vec_2,
    lua_vec_2_get::lua_vec_2_get,
    lua_vec_2_index::lua_vec_2_index,
    lua_vec_2_namecall::lua_vec_2_namecall,
    lua_vec_2_newindex::lua_vec_2_newindex,
    lua_vec_2_push::lua_vec_2_push,
    lua_vertex::lua_vertex,
    lua_vertex_index::lua_vertex_index,
    lua_vertex_namecall::lua_vertex_namecall,
    lua_vertex_newindex::lua_vertex_newindex,
    safe_api::{pushcclosurek, setuserdatametatable, state_mut},
  },
  records::{vec_2_conformance_ir_hooks::Vec2, vertex::Vertex},
};

/// 二元算术分派：两个 Vec2 操作数 → 新建 Vec2 结果（`vec2_add/sub/mul/div` 共用）。
fn vec2_binop(l: *mut LuaState, op: impl Fn(f32, f32, f32, f32) -> (f32, f32)) -> c_int {
  let a = lua_vec_2_get(l, 1);
  let b = lua_vec_2_get(l, 2);
  let data = lua_vec_2_push(l);
  // Safety: 两个操作数指针指向刚校验的 Vec2 数据区（读），`data` 为刚新建的数据区（写）。
  let (x, y) = unsafe { op((*a).x, (*a).y, (*b).x, (*b).y) };
  unsafe {
    (*data).x = x;
    (*data).y = y;
  }
  1
}

unsafe extern "C-unwind" fn vec2_add(l: *mut LuaState) -> c_int {
  vec2_binop(l, |ax, ay, bx, by| (ax + bx, ay + by))
}

unsafe extern "C-unwind" fn vec2_sub(l: *mut LuaState) -> c_int {
  vec2_binop(l, |ax, ay, bx, by| (ax - bx, ay - by))
}

unsafe extern "C-unwind" fn vec2_mul(l: *mut LuaState) -> c_int {
  vec2_binop(l, |ax, ay, bx, by| (ax * bx, ay * by))
}

unsafe extern "C-unwind" fn vec2_div(l: *mut LuaState) -> c_int {
  vec2_binop(l, |ax, ay, bx, by| (ax / bx, ay / by))
}

unsafe extern "C-unwind" fn vec2_unm(l: *mut LuaState) -> c_int {
  let a = lua_vec_2_get(l, 1);
  let data = lua_vec_2_push(l);
  // Safety: `a` 指向刚校验的 Vec2 数据区（读），`data` 为刚新建的数据区（写）。
  unsafe {
    (*data).x = -(*a).x;
    (*data).y = -(*a).y;
  }
  1
}

fn register_methods(l: *mut LuaState, methods: &[(&'static str, LuaCFunction)]) {
  for &(name, func) in methods {
    pushcclosurek(l, func, None, 0, None);
    state_mut(l).set_field_str(-2, name);
  }
}

/// 以 vec2/vertex 元表 + 全局构造函数装配 userdata 族帮手（cpp `setupUserdataHelpers`）。
pub fn setup_userdata_helpers(l: *mut LuaState) {
  // 方法表是纯 Rust 数组（`"…"`, 函数指针），构造不触碰 `l`。
  let vec2_methods: [(&'static str, LuaCFunction); 8] = [
    ("__index", Some(lua_vec_2_index)),
    ("__newindex", Some(lua_vec_2_newindex)),
    ("__namecall", Some(lua_vec_2_namecall)),
    ("__add", Some(vec2_add)),
    ("__sub", Some(vec2_sub)),
    ("__mul", Some(vec2_mul)),
    ("__div", Some(vec2_div)),
    ("__unm", Some(vec2_unm)),
  ];
  let vertex_methods: [(&'static str, LuaCFunction); 3] = [
    ("__index", Some(lua_vertex_index)),
    ("__newindex", Some(lua_vertex_newindex)),
    ("__namecall", Some(lua_vertex_namecall)),
  ];

  // 新建 vec2 metatable，绑到 Vec2::TAG；结束后栈顶留下该表
  //（后续 setreadonly/register_methods 都以 -1 取它）。
  state_mut(l).new_metatable_by_str("vec2");
  state_mut(l).push_value(-1);
  setuserdatametatable(l, Vec2::TAG);

  register_methods(l, &vec2_methods);

  // 置只读、注册全局构造函数后弹掉该表。
  state_mut(l).set_readonly(-1, true);
  pushcclosurek(l, Some(lua_vec_2), Some(b"vec2\0"), 0, None);
  state_mut(l).set_global_str("vec2");
  state_mut(l).pop(1);

  // 新建 vertex metatable 并绑到 Vertex::TAG，结束后栈顶留下该表。
  state_mut(l).new_metatable_by_str("vertex");
  state_mut(l).push_value(-1);
  setuserdatametatable(l, Vertex::TAG);

  register_methods(l, &vertex_methods);

  // 置只读、注册全局构造函数后弹掉该表。
  state_mut(l).set_readonly(-1, true);
  pushcclosurek(l, Some(lua_vertex), Some(b"vertex\0"), 0, None);
  state_mut(l).set_global_str("vertex");
  state_mut(l).pop(1);
}
