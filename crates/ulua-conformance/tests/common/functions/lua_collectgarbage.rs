use core::{ffi::c_int, ptr::null};

use ulua_vm::{
  enums::lua_gc_op::LuaGcOp,
  functions::{
    lua_gc::lua_gc, lua_l_checkoption::lua_l_checkoption, lua_l_optinteger::lua_l_optinteger,
    lua_pushboolean::lua_pushboolean, lua_pushnumber::lua_pushnumber,
  },
  records::lua_state::lua_State,
};
pub(crate) extern "C-unwind" fn lua_collectgarbage(l: *mut lua_State) -> i32 {
  let opts = [
    c"stop".as_ptr(),
    c"restart".as_ptr(),
    c"collect".as_ptr(),
    c"count".as_ptr(),
    c"isrunning".as_ptr(),
    c"step".as_ptr(),
    c"setgoal".as_ptr(),
    c"setstepmul".as_ptr(),
    c"setstepsize".as_ptr(),
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

  let o = unsafe { lua_l_checkoption(l, 1, c"collect".as_ptr(), opts.as_ptr()) };
  let ex = unsafe { lua_l_optinteger(l, 2, 0) };
  let op = optsnum[o as usize];
  let res = unsafe { lua_gc(l, op, ex) };

  if op == LuaGcOp::Step as c_int || op == LuaGcOp::Isrunning as c_int {
    unsafe {
      lua_pushboolean(l, res);
    }
    1
  } else {
    unsafe {
      lua_pushnumber(l, res as f64);
    }
    1
  }
}
