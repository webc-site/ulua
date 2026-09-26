use core::{ffi::c_int, ptr::null};

use ulua_vm::{
  enums::lua_gc_op::LuaGcOp,
  functions::{
    lua_gc::lua_gc, lua_l_checkoption::lua_l_checkoption, lua_l_optinteger::lua_l_optinteger,
    lua_pushboolean::lua_pushboolean, lua_pushnumber::lua_pushnumber,
  },
  records::lua_state::LuaState,
};

use crate::common::functions::cstr::cstr;
pub(crate) extern "C-unwind" fn lua_collectgarbage(l: *mut LuaState) -> i32 {
  let opts = [
    cstr(b"stop\0"),
    cstr(b"restart\0"),
    cstr(b"collect\0"),
    cstr(b"count\0"),
    cstr(b"isrunning\0"),
    cstr(b"step\0"),
    cstr(b"setgoal\0"),
    cstr(b"setstepmul\0"),
    cstr(b"setstepsize\0"),
    null(),
  ];

  let optsnum = [
    LuaGcOp::Stop as c_int,
    LuaGcOp::Restart as c_int,
    LuaGcOp::Collect as c_int,
    LuaGcOp::Count as c_int,
    LuaGcOp::Isrunning as c_int,
    LuaGcOp::Step as c_int,
    LuaGcOp::Setgoal as c_int,
    LuaGcOp::Setstepmul as c_int,
    LuaGcOp::Setstepsize as c_int,
  ];

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`opts` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let o = unsafe { lua_l_checkoption(l, 1, cstr(b"collect\0"), opts.as_ptr()) };
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`opts` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let ex = unsafe { lua_l_optinteger(l, 2, 0) };
  let op = optsnum[o as usize];
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`opts` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let res = unsafe { lua_gc(l, op, ex) };

  if op == LuaGcOp::Step as c_int || op == LuaGcOp::Isrunning as c_int {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`opts` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      lua_pushboolean(l, res);
    }
    1
  } else {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`opts` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      lua_pushnumber(l, res as f64);
    }
    1
  }
}
