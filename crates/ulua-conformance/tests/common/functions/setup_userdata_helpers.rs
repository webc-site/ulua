use core::{ffi::c_int, ptr::null};

use ulua_vm::{
  functions::{
    lua_pushcclosurek::lua_pushcclosurek, lua_setuserdatametatable::lua_setuserdatametatable,
  },
  records::lua_state::LuaState,
  type_aliases::lua_c_function::LuaCFunction,
};

use crate::common::{
  functions::{
    cstr::cstr, lua_vec_2::lua_vec_2, lua_vec_2_get::lua_vec_2_get,
    lua_vec_2_index::lua_vec_2_index, lua_vec_2_namecall::lua_vec_2_namecall,
    lua_vec_2_newindex::lua_vec_2_newindex, lua_vec_2_push::lua_vec_2_push, lua_vertex::lua_vertex,
    lua_vertex_index::lua_vertex_index, lua_vertex_namecall::lua_vertex_namecall,
    lua_vertex_newindex::lua_vertex_newindex,
  },
  records::{vec_2_conformance_ir_hooks::Vec2, vertex::Vertex},
};
#[inline]
unsafe fn vec2_binop(l: *mut LuaState, op: impl Fn(f32, f32, f32, f32) -> (f32, f32)) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
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

unsafe extern "C-unwind" fn vec2_add(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { vec2_binop(l, |ax, ay, bx, by| (ax + bx, ay + by)) }
}

unsafe extern "C-unwind" fn vec2_sub(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { vec2_binop(l, |ax, ay, bx, by| (ax - bx, ay - by)) }
}

unsafe extern "C-unwind" fn vec2_mul(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { vec2_binop(l, |ax, ay, bx, by| (ax * bx, ay * by)) }
}

unsafe extern "C-unwind" fn vec2_div(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { vec2_binop(l, |ax, ay, bx, by| (ax / bx, ay / by)) }
}

unsafe extern "C-unwind" fn vec2_unm(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let a = lua_vec_2_get(l, 1);
    let data = lua_vec_2_push(l);
    (*data).x = -(*a).x;
    (*data).y = -(*a).y;
    1
  }
}

unsafe fn register_methods(l: *mut LuaState, methods: &[(&'static str, LuaCFunction)]) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    for &(name, func) in methods {
      lua_pushcclosurek(l, func, null(), 0, None);
      (*l).set_field_str(-2, name);
    }
  }
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn setup_userdata_helpers(l: *mut LuaState) {
  // 方法表是纯 Rust 数组（`"…"`, 函数指针），构造不触碰 `l`，先于 unsafe 段成形。
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

  // Safety: `l` 为存活 LuaState；本段新建 vec2 metatable 并把它绑到 Vec2::TAG，
  // 结束后栈顶留下该表（后续 setreadonly/register_methods 都以 -1 取它）。
  unsafe {
    (*l).new_metatable_by_str("vec2");
    (*l).push_value(-1);
    lua_setuserdatametatable(l, Vec2::TAG);
  }

  // Safety: `l` 存活且栈顶为刚创建的 vec2 metatable；`vec2_methods` 由本帧持有，
  // 元素是字符串与 `extern "C-unwind"` 函数指针。
  unsafe { register_methods(l, &vec2_methods) };

  // Safety: `l` 存活且栈顶为 vec2 metatable；置只读、注册全局构造函数后 `lua_pop` 弹掉该表。
  unsafe {
    (*l).set_readonly(-1, true);

    lua_pushcclosurek(l, Some(lua_vec_2), cstr(b"vec2\0"), 0, None);
    (*l).set_global_str("vec2");

    (*l).pop(1);
  }

  // Safety: `l` 存活；本段新建 vertex metatable 并绑到 Vertex::TAG，结束后栈顶留下该表。
  unsafe {
    (*l).new_metatable_by_str("vertex");
    (*l).push_value(-1);
    lua_setuserdatametatable(l, Vertex::TAG);
  }

  // Safety: `l` 存活且栈顶为刚创建的 vertex metatable；`vertex_methods` 由本帧持有，
  // 元素是字符串与 `extern "C-unwind"` 函数指针。
  unsafe { register_methods(l, &vertex_methods) };

  // Safety: `l` 存活且栈顶为 vertex metatable；置只读、注册全局构造函数后 `lua_pop` 弹掉该表。
  unsafe {
    (*l).set_readonly(-1, true);

    lua_pushcclosurek(l, Some(lua_vertex), cstr(b"vertex\0"), 0, None);
    (*l).set_global_str("vertex");

    (*l).pop(1);
  }
}
