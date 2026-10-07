use core::{ffi::c_int, ptr::null};

use ulua_vm::{enums::lua_gc_op::LuaGcOp, records::lua_state::LuaState};

use crate::common::functions::{
  cstr::cstr,
  safe_api::{gc, l_checkoption, l_optinteger, state_mut},
};
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

  let o = l_checkoption(l, 1, cstr(b"collect\0"), opts.as_ptr());
  let ex = l_optinteger(l, 2, 0);
  let op = optsnum[o as usize];
  let res = gc(l, op, ex);

  if op == LuaGcOp::Step as c_int || op == LuaGcOp::Isrunning as c_int {
    state_mut(l).push_boolean(res != 0);
    1
  } else {
    state_mut(l).push_number(res as f64);
    1
  }
}
