#[cfg(test)]
mod common;

use core::{
  mem::zeroed,
  ptr::{null, null_mut, write_bytes},
};
use std::ptr::fn_addr_eq;

use crate::common::records::direct_field_access_handler_hit_count::DIRECT_FIELD_ACCESS_TEST_MUTEX;
extern crate alloc;

// Port of `cpp/tests/Conformance.test.cpp`.
// Conformance tests.

#[cfg(test)]
#[test]
fn conformance_api_alloc() {
  use core::ffi::c_void;

  use ulua_vm::functions::{lua_getallocf::lua_getallocf, lua_newstate::lua_newstate};

  use crate::common::{
    functions::limited_realloc::limited_realloc, type_aliases::state_ref::StateRef,
  };

  let mut ud = 0;
  let global_state =
    StateRef::new(unsafe { lua_newstate(Some(limited_realloc), (&mut ud as *mut i32).cast()) })
      .expect("lua state allocation failed");
  let l = global_state.as_ptr();

  let mut ud_check: *mut c_void = null_mut();
  let allocf = unsafe { lua_getallocf(l, &mut ud_check) };
  let expected =
    limited_realloc as unsafe extern "C-unwind" fn(*mut c_void, *mut u8, usize, usize) -> *mut u8;

  assert!(matches!(
      allocf,
      Some(f) if fn_addr_eq(f, expected)
  ));
  assert_eq!(ud_check, (&mut ud as *mut i32).cast());
}

#[cfg(test)]
#[test]
fn conformance_api_atoms() {
  use core::ffi::c_int;
  use std::ffi::CStr;

  use ulua_vm::functions::{
    lua_callbacks::lua_callbacks, lua_concat::lua_concat, lua_l_newstate::lua_l_newstate,
    lua_pushstring::lua_pushstring, lua_tostringatom::lua_tostringatom,
  };

  use crate::common::{
    functions::conformance_api_atoms_useratom::conformance_api_atoms_useratom,
    type_aliases::state_ref::StateRef,
  };

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    (*lua_callbacks(l)).useratom = Some(conformance_api_atoms_useratom);

    lua_pushstring(l, c"string".as_ptr());
    lua_pushstring(l, c"import".as_ptr());
    lua_pushstring(l, c"ant".as_ptr());
    lua_concat(l, 2);
    lua_pushstring(l, c"unimportant".as_ptr());

    let mut a1: c_int = 0;
    let mut a2: c_int = 0;
    let mut a3: c_int = 0;

    let s1 = lua_tostringatom(l, -3, &mut a1);
    let s2 = lua_tostringatom(l, -2, &mut a2);
    let s3 = lua_tostringatom(l, -1, &mut a3);

    assert_eq!(CStr::from_ptr(s1), c"string");
    assert_eq!(a1, 0);

    assert_eq!(CStr::from_ptr(s2), c"important");
    assert_eq!(a2, 1);

    assert_eq!(CStr::from_ptr(s3), c"unimportant");
    assert_eq!(a3, -1);
  }
}

#[cfg(test)]
#[test]
fn conformance_api_buffer() {
  use std::ffi::CStr;

  use ulua_vm::{
    enums::lua_type::LuaType,
    functions::{
      lua_equal::lua_equal, lua_l_checkbuffer::lua_l_checkbuffer, lua_l_newstate::lua_l_newstate,
      lua_l_typename::lua_l_typename, lua_newbuffer::lua_newbuffer, lua_objlen::lua_objlen,
      lua_pushvalue::lua_pushvalue, lua_tobuffer::lua_tobuffer, lua_topointer::lua_topointer,
      lua_type::lua_type, lua_typename::lua_typename,
    },
    macros::{lua_isbuffer::lua_isbuffer, lua_pop::lua_pop},
  };

  use crate::common::type_aliases::state_ref::StateRef;

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    lua_newbuffer(l, 1000);

    assert_eq!(lua_type(l, -1), LuaType::Buffer as i32);

    assert!(lua_isbuffer!(l, -1));
    assert_eq!(lua_objlen(l, -1), 1000);

    assert_eq!(
      CStr::from_ptr(lua_typename(l, LuaType::Buffer as i32)),
      c"buffer"
    );

    assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"buffer");

    let p1 = lua_tobuffer(l, -1, null_mut());

    let mut len = 0usize;
    let p2 = lua_tobuffer(l, -1, &mut len);
    assert_eq!(len, 1000);
    assert_eq!(p1, p2);

    let p3 = lua_l_checkbuffer(l, -1, null_mut());
    assert_eq!(p1, p3);

    len = 0;
    let p4 = lua_l_checkbuffer(l, -1, &mut len);
    assert_eq!(len, 1000);
    assert_eq!(p1, p4);

    write_bytes(p1.cast::<u8>(), 0xab, 1000);

    assert!(!lua_topointer(l, -1).is_null());

    lua_newbuffer(l, 0);

    lua_pushvalue(l, -2);

    assert_ne!(lua_equal(l, -3, -1), 0);
    assert_eq!(lua_equal(l, -2, -1), 0);

    lua_pop(l, 1);
  }
}

/// apicalls.luau 中人为截断的 `pi = 3.1415926`（非 math.pi），按位精确比较。
const APICALLS_LUAU_PI: f64 = f64::from_bits(0x4009_21fb_4d12_d84a);

#[cfg(test)]
#[test]
fn conformance_api_calls() {
  use core::ffi::{CStr, c_void};

  use ulua_vm::{
    enums::{lua_gc_op::LuaGcOp, lua_status::LuaStatus},
    functions::{
      lua_call::lua_call, lua_clonefunction::lua_clonefunction, lua_cpcall::lua_cpcall,
      lua_equal::lua_equal, lua_gc::lua_gc, lua_getfield::lua_getfield, lua_gettop::lua_gettop,
      lua_isnumber::lua_isnumber, lua_isstring::lua_isstring,
      lua_l_checkinteger::lua_l_checkinteger, lua_l_checkstack::lua_l_checkstack,
      lua_newstate::lua_newstate, lua_newthread::lua_newthread, lua_pcall::lua_pcall,
      lua_pushcclosurek::lua_pushcclosurek, lua_pushinteger::lua_pushinteger,
      lua_pushnumber::lua_pushnumber, lua_resume::lua_resume, lua_setfenv::lua_setfenv,
      lua_setfield::lua_setfield, lua_status::lua_status, lua_toboolean::lua_toboolean,
    },
    macros::{
      lua_globalsindex::LUA_GLOBALSINDEX, lua_multret::LUA_MULTRET, lua_newtable::lua_newtable,
      lua_pop::lua_pop, lua_tonumber::lua_tonumber, lua_tostring::lua_tostring,
      luai_maxcstack::LUAI_MAXCSTACK,
    },
  };

  use crate::common::functions::{
    conformance_api_calls_check_not_yieldable::conformance_api_calls_check_not_yieldable,
    cpcall_test::cpcall_test, limited_realloc::limited_realloc, run_conformance::run_conformance,
  };

  let global_state = unsafe {
    run_conformance(
      c"apicalls.luau".as_ptr(),
      None,
      None,
      lua_newstate(Some(limited_realloc), null_mut()),
      null_mut(),
      false,
      null_mut(),
    )
  };
  let l = global_state.as_ptr();

  unsafe {
    lua_getfield(l, LUA_GLOBALSINDEX, c"add".as_ptr());
    lua_pushnumber(l, 40.0);
    lua_pushnumber(l, 2.0);
    lua_call(l, 2, 1);
    assert_ne!(lua_isnumber(l, -1), 0);
    assert_eq!(lua_tonumber!(l, -1), 42.0);
    lua_pop(l, 1);

    lua_getfield(l, LUA_GLOBALSINDEX, c"getnresults".as_ptr());
    lua_pushinteger(l, 200);
    lua_call(l, 1, LUA_MULTRET);
    assert_eq!(lua_gettop(l), 200);
    lua_pop(l, 200);

    lua_getfield(l, LUA_GLOBALSINDEX, c"add".as_ptr());
    lua_pushnumber(l, 40.0);
    lua_pushnumber(l, 2.0);
    let status = lua_pcall(l, 2, 1, 0);
    assert_eq!(status, LuaStatus::Ok as i32);
    assert_ne!(lua_isnumber(l, -1), 0);
    assert_eq!(lua_tonumber!(l, -1), 42.0);
    lua_pop(l, 1);

    lua_getfield(l, LUA_GLOBALSINDEX, c"getnresults".as_ptr());
    lua_pushinteger(l, 200);
    let status = lua_pcall(l, 1, LUA_MULTRET, 0);
    assert_eq!(status, LuaStatus::Ok as i32);
    assert_eq!(lua_gettop(l), 200);
    lua_pop(l, 200);

    lua_getfield(l, LUA_GLOBALSINDEX, c"pcall".as_ptr());
    lua_getfield(l, LUA_GLOBALSINDEX, c"getnresults".as_ptr());
    lua_pushinteger(l, 200);
    lua_call(l, 2, LUA_MULTRET);
    assert_eq!(lua_gettop(l), 201);
    lua_pop(l, 200);
    assert_eq!(lua_toboolean(l, -1), 1);
    lua_pop(l, 1);

    let mut should_fail = false;
    assert_eq!(
      lua_cpcall(
        l,
        Some(cpcall_test),
        (&mut should_fail as *mut bool).cast::<c_void>(),
      ),
      LuaStatus::Ok as i32
    );
    assert_eq!(lua_status(l), LuaStatus::Ok as i32);

    lua_getfield(l, LUA_GLOBALSINDEX, c"cpcallvalue".as_ptr());
    assert_eq!(lua_l_checkinteger(l, -1), 123);
    lua_pop(l, 1);

    let mut should_fail = true;
    assert_eq!(
      lua_cpcall(
        l,
        Some(cpcall_test),
        (&mut should_fail as *mut bool).cast::<c_void>(),
      ),
      LuaStatus::ErrRun as i32
    );
    assert_ne!(lua_isstring(l, -1), 0);
    assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"Failed");
    lua_pop(l, 1);

    assert_eq!(lua_status(l), LuaStatus::Ok as i32);

    let mut should_fail = false;
    assert_eq!(lua_gettop(l), 0);
    lua_l_checkstack(l, LUAI_MAXCSTACK - 1, "must succeed");

    for _ in 0..LUAI_MAXCSTACK - 1 {
      lua_pushnumber(l, 1.0);
    }

    assert_eq!(
      lua_cpcall(
        l,
        Some(cpcall_test),
        (&mut should_fail as *mut bool).cast::<c_void>(),
      ),
      LuaStatus::ErrRun as i32
    );
    assert_ne!(lua_isstring(l, -1), 0);
    assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"stack limit");
    lua_pop(l, 1);

    assert_eq!(lua_status(l), LuaStatus::Ok as i32);
    lua_pop(l, LUAI_MAXCSTACK - 1);

    let l2 = lua_newthread(l);
    lua_pushcclosurek(
      l2,
      Some(conformance_api_calls_check_not_yieldable),
      null(),
      0,
      None,
    );
    lua_call(l2, 0, 0);

    lua_getfield(l2, LUA_GLOBALSINDEX, c"getnresults".as_ptr());
    lua_pushinteger(l2, 1);
    let status = lua_resume(l2, null_mut(), 1);
    assert_eq!(status, LuaStatus::Ok as i32);
    assert_eq!(lua_gettop(l2), 1);
    lua_pop(l2, 1);

    lua_pushcclosurek(
      l2,
      Some(conformance_api_calls_check_not_yieldable),
      null(),
      0,
      None,
    );
    lua_call(l2, 0, 0);

    lua_pop(l, 1);

    let l2 = lua_newthread(l);

    lua_getfield(l2, LUA_GLOBALSINDEX, c"create_with_tm".as_ptr());
    lua_pushnumber(l2, 42.0);
    lua_pcall(l2, 1, 1, 0);

    lua_getfield(l2, LUA_GLOBALSINDEX, c"create_with_tm".as_ptr());
    lua_pushnumber(l2, 42.0);
    lua_pcall(l2, 1, 1, 0);

    lua_gc(l2, LuaGcOp::Collect as i32, 0);
    lua_gc(l2, LuaGcOp::Step as i32, 8);

    assert_eq!(lua_equal(l2, -1, -2), 1);
    lua_pop(l2, 2);

    lua_pop(l, 1);

    lua_getfield(l, LUA_GLOBALSINDEX, c"getpi".as_ptr());
    lua_call(l, 0, 1);
    assert_eq!(lua_tonumber!(l, -1), APICALLS_LUAU_PI);
    lua_pop(l, 1);

    lua_getfield(l, LUA_GLOBALSINDEX, c"getpi".as_ptr());

    lua_clonefunction(l, -1);
    lua_newtable(l);
    lua_pushnumber(l, 42.0);
    lua_setfield(l, -2, c"pi".as_ptr());
    lua_setfenv(l, -2);

    lua_call(l, 0, 1);
    assert_eq!(lua_tonumber!(l, -1), 42.0);
    lua_pop(l, 1);

    lua_call(l, 0, 1);
    assert_eq!(lua_tonumber!(l, -1), APICALLS_LUAU_PI);
    lua_pop(l, 1);

    lua_getfield(l, LUA_GLOBALSINDEX, c"incuv".as_ptr());
    lua_call(l, 0, 1);
    assert_eq!(lua_tonumber!(l, -1), 1.0);
    lua_pop(l, 1);

    lua_getfield(l, LUA_GLOBALSINDEX, c"incuv".as_ptr());
    lua_clonefunction(l, -1);
    lua_clonefunction(l, -2);

    lua_call(l, 0, 1);
    assert_eq!(lua_tonumber!(l, -1), 2.0);
    lua_pop(l, 1);

    lua_call(l, 0, 1);
    assert_eq!(lua_tonumber!(l, -1), 3.0);
    lua_pop(l, 1);

    lua_call(l, 0, 1);
    assert_eq!(lua_tonumber!(l, -1), 4.0);
    lua_pop(l, 1);

    lua_getfield(l, LUA_GLOBALSINDEX, c"largealloc".as_ptr());
    let res = lua_pcall(l, 0, 0, 0);
    assert_eq!(res, LuaStatus::ErrMem as i32);
    lua_pop(l, 1);

    lua_getfield(l, LUA_GLOBALSINDEX, c"oops".as_ptr());
    lua_getfield(l, LUA_GLOBALSINDEX, c"largealloc".as_ptr());
    let res = lua_pcall(l, 0, 1, -2);
    assert_eq!(res, LuaStatus::ErrMem as i32);
    assert_ne!(lua_isstring(l, -1), 0);
    assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"oops");
    lua_pop(l, 2);

    lua_getfield(l, LUA_GLOBALSINDEX, c"error".as_ptr());
    lua_getfield(l, LUA_GLOBALSINDEX, c"largealloc".as_ptr());
    let res = lua_pcall(l, 0, 1, -2);
    assert_eq!(res, LuaStatus::ErrErr as i32);
    assert_ne!(lua_isstring(l, -1), 0);
    assert_eq!(
      CStr::from_ptr(lua_tostring!(l, -1)),
      c"error in error handling"
    );
    lua_pop(l, 2);

    lua_getfield(l, LUA_GLOBALSINDEX, c"largealloc".as_ptr());
    lua_getfield(l, LUA_GLOBALSINDEX, c"largealloc".as_ptr());
    let res = lua_pcall(l, 0, 1, -2);
    assert_eq!(res, LuaStatus::ErrMem as i32);
    assert_ne!(lua_isstring(l, -1), 0);
    assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"not enough memory");
    lua_pop(l, 2);

    lua_getfield(l, LUA_GLOBALSINDEX, c"largealloc".as_ptr());
    lua_getfield(l, LUA_GLOBALSINDEX, c"error".as_ptr());
    let res = lua_pcall(l, 0, 1, -2);
    assert_eq!(res, LuaStatus::ErrErr as i32);
    assert_ne!(lua_isstring(l, -1), 0);
    assert_eq!(
      CStr::from_ptr(lua_tostring!(l, -1)),
      c"error in error handling"
    );
    lua_pop(l, 2);

    assert_eq!(lua_gettop(l), 0);
  }
}

#[cfg(test)]
#[test]
fn conformance_api_iter() {
  use ulua_vm::{
    functions::{
      lua_checkstack::lua_checkstack, lua_gettop::lua_gettop, lua_l_newstate::lua_l_newstate,
      lua_next::lua_next, lua_pushnil::lua_pushnil, lua_pushnumber::lua_pushnumber,
      lua_pushstring::lua_pushstring, lua_pushvalue::lua_pushvalue, lua_rawiter::lua_rawiter,
      lua_rawsetfield::lua_rawsetfield, lua_rawseti::lua_rawseti, lua_setfield::lua_setfield,
      lua_settop::lua_settop,
    },
    macros::{lua_newtable::lua_newtable, lua_pop::lua_pop, lua_tonumber::lua_tonumber},
  };

  use crate::common::type_aliases::state_ref::StateRef;

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    lua_newtable(l);
    lua_pushnumber(l, 123.0);
    lua_setfield(l, -2, c"key".as_ptr());
    lua_pushnumber(l, 456.0);
    lua_rawsetfield(l, -2, c"key2".as_ptr());
    lua_pushstring(l, c"test".as_ptr());
    lua_rawseti(l, -2, 1);

    let mut sum1 = 0.0;
    lua_pushnil(l);
    while lua_next(l, -2) != 0 {
      sum1 += lua_tonumber!(l, -2);
      sum1 += lua_tonumber!(l, -1);
      lua_pop(l, 1);
    }
    assert_eq!(sum1, 580.0);

    let mut sum2 = 0.0;
    let mut index = 0;
    loop {
      index = lua_rawiter(l, -1, index);
      if index < 0 {
        break;
      }

      sum2 += lua_tonumber!(l, -2);
      sum2 += lua_tonumber!(l, -1);
      lua_pop(l, 2);
    }
    assert_eq!(sum2, 580.0);

    lua_settop(l, 18);
    lua_pushvalue(l, 1);

    assert_eq!(lua_gettop(l), 19);
    assert_ne!(lua_checkstack(l, 2), 0);

    let mut sum3 = 0.0;
    let mut index = 0;
    loop {
      index = lua_rawiter(l, -1, index);
      if index < 0 {
        break;
      }

      sum3 += lua_tonumber!(l, -2);
      sum3 += lua_tonumber!(l, -1);
      lua_pop(l, 2);
    }
    assert_eq!(sum3, 580.0);

    lua_pop(l, 19);
  }
}

#[cfg(test)]
#[test]
fn conformance_api_stack() {
  use std::ffi::CStr;

  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_checkstack::lua_checkstack, lua_newstate::lua_newstate, lua_newthread::lua_newthread,
      lua_pcall::lua_pcall,
    },
    macros::{
      lua_l_checkstring::luaL_checkstring, lua_pushcfunction::LUA_PUSHCFUNCTION,
      luai_maxcstack::LUAI_MAXCSTACK,
    },
  };

  use crate::common::{
    functions::{
      blockable_realloc::blockable_realloc, blockable_realloc_allowed::BLOCKABLE_REALLOC_ALLOWED,
      slowly_overflow_stack::slowly_overflow_stack,
    },
    type_aliases::state_ref::StateRef,
  };

  let global_state = StateRef::new(unsafe { lua_newstate(Some(blockable_realloc), null_mut()) })
    .expect("lua state allocation failed");
  let gl = global_state.as_ptr();

  unsafe {
    let l = lua_newthread(gl);

    LUA_PUSHCFUNCTION(l, Some(slowly_overflow_stack), c"foo".as_ptr());
    let result = lua_pcall(l, 0, 0, 0);
    assert_eq!(result, LuaStatus::ErrRun as i32);
    assert_eq!(
      CStr::from_ptr(luaL_checkstring!(l, -1)),
      c"stack overflow (test)"
    );
  }

  unsafe {
    let l = lua_newthread(gl);

    assert_eq!(lua_checkstack(l, 100), 1);

    BLOCKABLE_REALLOC_ALLOWED = false;
    assert_eq!(lua_checkstack(l, 1000), 0);
    BLOCKABLE_REALLOC_ALLOWED = true;

    assert_eq!(lua_checkstack(l, 1000), 1);

    assert_eq!(lua_checkstack(l, LUAI_MAXCSTACK * 2), 0);
  }
}

#[cfg(test)]
#[test]
fn conformance_api_tables() {
  use core::ffi::c_void;
  use std::ffi::CStr;

  use ulua_vm::{
    enums::lua_type::LuaType,
    functions::{
      lua_cleartable::lua_cleartable, lua_clonetable::lua_clonetable, lua_getfield::lua_getfield,
      lua_gettable::lua_gettable, lua_l_newstate::lua_l_newstate, lua_next::lua_next,
      lua_pushnil::lua_pushnil, lua_pushnumber::lua_pushnumber, lua_pushstring::lua_pushstring,
      lua_rawget::lua_rawget, lua_rawgetfield::lua_rawgetfield, lua_rawgeti::lua_rawgeti,
      lua_rawgetptagged::lua_rawgetptagged, lua_rawsetfield::lua_rawsetfield,
      lua_rawseti::lua_rawseti, lua_rawsetptagged::lua_rawsetptagged, lua_setfield::lua_setfield,
    },
    macros::{
      lua_newtable::lua_newtable, lua_pop::lua_pop, lua_rawgetp::lua_rawgetp,
      lua_rawsetp::lua_rawsetp, lua_tonumber::lua_tonumber, lua_tostring::lua_tostring,
    },
  };

  use crate::common::type_aliases::state_ref::StateRef;

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();
  let mut lu1 = 1;
  let mut lu2 = 2;
  let lu1p = (&mut lu1 as *mut i32).cast::<c_void>();
  let lu2p = (&mut lu2 as *mut i32).cast::<c_void>();

  unsafe {
    lua_newtable(l);
    lua_pushnumber(l, 123.0);
    lua_setfield(l, -2, c"key".as_ptr());
    lua_pushnumber(l, 456.0);
    lua_rawsetfield(l, -2, c"key2".as_ptr());
    lua_pushstring(l, c"key3".as_ptr());
    lua_rawseti(l, -2, 5);
    lua_pushstring(l, c"key4".as_ptr());
    lua_rawsetp!(l, -2, lu1p);
    lua_pushstring(l, c"key5".as_ptr());
    lua_rawsetptagged(l, -2, lu2p, 1);
    lua_pushstring(l, c"key6".as_ptr());
    lua_rawsetptagged(l, -2, lu2p, 2);

    lua_pushstring(l, c"key".as_ptr());
    assert_eq!(lua_gettable(l, -2), LuaType::Number as i32);
    assert_eq!(lua_tonumber!(l, -1), 123.0);
    lua_pop(l, 1);

    assert_eq!(lua_getfield(l, -1, c"key".as_ptr()), LuaType::Number as i32);
    assert_eq!(lua_tonumber!(l, -1), 123.0);
    lua_pop(l, 1);

    assert_eq!(
      lua_rawgetfield(l, -1, c"key2".as_ptr()),
      LuaType::Number as i32
    );
    assert_eq!(lua_tonumber!(l, -1), 456.0);
    lua_pop(l, 1);

    lua_pushstring(l, c"key".as_ptr());
    assert_eq!(lua_rawget(l, -2), LuaType::Number as i32);
    assert_eq!(lua_tonumber!(l, -1), 123.0);
    lua_pop(l, 1);

    assert_eq!(lua_rawgeti(l, -1, 5), LuaType::String as i32);
    assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"key3");
    lua_pop(l, 1);

    assert_eq!(lua_rawgetp(l, -1, lu1p), LuaType::String as i32);
    assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"key4");
    lua_pop(l, 1);

    assert_eq!(lua_rawgetptagged(l, -1, lu2p, 1), LuaType::String as i32);
    assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"key5");
    lua_pop(l, 1);

    assert_eq!(lua_rawgetptagged(l, -1, lu2p, 2), LuaType::String as i32);
    assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"key6");
    lua_pop(l, 1);

    assert_eq!(lua_rawgetptagged(l, -1, lu2p, 0), LuaType::Nil as i32);
    lua_pop(l, 1);

    lua_clonetable(l, -1);

    assert_eq!(lua_getfield(l, -1, c"key".as_ptr()), LuaType::Number as i32);
    assert_eq!(lua_tonumber!(l, -1), 123.0);
    lua_pop(l, 1);

    lua_pushnumber(l, 456.0);
    lua_rawsetfield(l, -2, c"key".as_ptr());

    lua_pop(l, 1);

    assert_eq!(lua_getfield(l, -1, c"key".as_ptr()), LuaType::Number as i32);
    assert_eq!(lua_tonumber!(l, -1), 123.0);
    lua_pop(l, 1);

    lua_cleartable(l, -1);
    lua_pushnil(l);
    assert_eq!(lua_next(l, -2), 0);

    lua_pop(l, 1);
  }
}

#[cfg(test)]
#[test]
fn conformance_api_type() {
  use std::ffi::CStr;

  use ulua_vm::{
    enums::lua_type::LuaType,
    functions::{
      lua_l_newstate::lua_l_newstate, lua_l_typename::lua_l_typename,
      lua_pushnumber::lua_pushnumber, lua_pushstring::lua_pushstring, lua_setfield::lua_setfield,
      lua_setmetatable::lua_setmetatable, lua_type::lua_type, lua_typename::lua_typename,
    },
    macros::{lua_newtable::lua_newtable, lua_newuserdata::lua_newuserdata},
  };

  use crate::common::type_aliases::state_ref::StateRef;

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    lua_pushnumber(l, 2.0);
    assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"number");
    assert_eq!(CStr::from_ptr(lua_l_typename(l, 1)), c"number");
    assert_eq!(lua_type(l, -1), LuaType::Number as i32);
    assert_eq!(lua_type(l, 1), LuaType::Number as i32);

    assert_eq!(CStr::from_ptr(lua_l_typename(l, 2)), c"no value");
    assert_eq!(lua_type(l, 2), LuaType::None as i32);
    assert_eq!(CStr::from_ptr(lua_typename(l, lua_type(l, 2))), c"no value");

    lua_newuserdata(l, 0);
    assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"userdata");
    assert_eq!(lua_type(l, -1), LuaType::UserData as i32);

    lua_newtable(l);
    lua_pushstring(l, c"hello".as_ptr());
    lua_setfield(l, -2, c"__type".as_ptr());
    lua_setmetatable(l, -2);

    assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"hello");
    assert_eq!(lua_type(l, -1), LuaType::UserData as i32);
  }
}

#[cfg(test)]
#[test]
fn conformance_assert() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"assert.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_attrib() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"attrib.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_basic() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"basic.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_bitwise() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"bitwise.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_buffers() {
  use crate::common::functions::{
    run_conformance::run_conformance, setup_native_helpers::setup_native_helpers,
  };

  unsafe {
    run_conformance(
      c"buffers.luau".as_ptr(),
      Some(setup_native_helpers),
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_bytecode_distribution_per_function_test() {
  use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;
  use ulua_common::enums::luau_opcode::LuauOpcode;

  use crate::common::functions::analyze_file::analyze_file;

  let source = r#"
local function first(n, p)
  local t = {}
  for i=1,p do t[i] = i*10 end

  local function inner(_,n)
    if n > 0 then
      n = n-1
      return n, unpack(t)
    end
  end
  return inner, nil, n
end

local function second(x)
 return x[1]
end
"#;

  let total_count =
    |summary: &FunctionBytecodeSummary| -> u32 { summary.get_counts(0).iter().copied().sum() };

  let summaries = analyze_file(source, 0, 1);

  assert_eq!("inner", summaries[0].get_name());
  assert_eq!(6, summaries[0].get_line());
  assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_LOADN as u8));
  assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_MOVE as u8));
  assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_GETUPVAL as u8));
  assert_eq!(
    1,
    summaries[0].get_count(0, LuauOpcode::LOP_GETIMPORT as u8)
  );
  assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_CALL as u8));
  assert_eq!(2, summaries[0].get_count(0, LuauOpcode::LOP_RETURN as u8));
  assert_eq!(
    1,
    summaries[0].get_count(0, LuauOpcode::LOP_JUMPIFNOTLT as u8)
  );
  assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_SUBK as u8));
  assert_eq!(
    1,
    summaries[0].get_count(0, LuauOpcode::LOP_FASTCALL1 as u8)
  );
  assert_eq!(10, total_count(&summaries[0]));

  assert_eq!("first", summaries[1].get_name());
  assert_eq!(2, summaries[1].get_line());
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_LOADNIL as u8));
  assert_eq!(2, summaries[1].get_count(0, LuauOpcode::LOP_LOADN as u8));
  assert_eq!(3, summaries[1].get_count(0, LuauOpcode::LOP_MOVE as u8));
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_SETTABLE as u8));
  assert_eq!(
    1,
    summaries[1].get_count(0, LuauOpcode::LOP_NEWCLOSURE as u8)
  );
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_RETURN as u8));
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_MULK as u8));
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_NEWTABLE as u8));
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_FORNPREP as u8));
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_FORNLOOP as u8));
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_CAPTURE as u8));
  assert_eq!(14, total_count(&summaries[1]));

  assert_eq!("second", summaries[2].get_name());
  assert_eq!(15, summaries[2].get_line());
  assert_eq!(
    1,
    summaries[2].get_count(0, LuauOpcode::LOP_GETTABLEN as u8)
  );
  assert_eq!(1, summaries[2].get_count(0, LuauOpcode::LOP_RETURN as u8));
  assert_eq!(2, total_count(&summaries[2]));

  assert_eq!("", summaries[3].get_name());
  assert_eq!(1, summaries[3].get_line());
  assert_eq!(1, summaries[3].get_count(0, LuauOpcode::LOP_RETURN as u8));
  assert_eq!(
    2,
    summaries[3].get_count(0, LuauOpcode::LOP_DUPCLOSURE as u8)
  );
  assert_eq!(
    1,
    summaries[3].get_count(0, LuauOpcode::LOP_PREPVARARGS as u8)
  );
  assert_eq!(4, total_count(&summaries[3]));
}

#[cfg(test)]
#[test]
fn conformance_c_yield() {
  use ulua_common::FFlag;

  use crate::common::{
    functions::{
      conformance_c_yield_setup::conformance_c_yield_setup, run_conformance::run_conformance,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _luau_resume_restore_c_calls = ScopedFastFlag::new(&FFlag::LuauResumeRestoreCcalls, true);

  unsafe {
    run_conformance(
      c"cyield.luau".as_ptr(),
      Some(conformance_c_yield_setup),
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_calls() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"calls.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_classes() {
  use ulua_common::FFlag;

  use crate::common::{
    functions::run_conformance::run_conformance, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _debug_luau_user_defined_classes =
    ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
  let _debug_luau_user_defined_classes_runtime =
    ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClassesRuntime, true);

  unsafe {
    run_conformance(
      c"classes.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_clear() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"clear.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_closure() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"closure.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_codegen_nop_padding_deterministic_off() {
  use core::ffi::{c_char, c_void};

  use ulua_code_gen::{
    functions::{
      compile_internal::compile_internal, luau_codegen_create::luau_codegen_create,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{compilation_options::CompilationOptions, compilation_stats::CompilationStats},
  };
  use ulua_compiler::functions::luau_compile::luau_compile;
  use ulua_vm::functions::{lua_l_newstate::lua_l_newstate, luau_load::luau_load};

  use crate::common::{functions::run_conformance::CODEGEN, type_aliases::state_ref::StateRef};

  unsafe extern "C" {
    fn free(ptr: *mut c_void);
  }

  if unsafe { !CODEGEN } || luau_codegen_supported() == 0 {
    return;
  }

  let source = r#"
        local function add(a, b) return a + b end
        return add(1, 2)
    "#;

  let compile = || -> usize {
    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
      luau_codegen_create(l);

      let mut bytecode_size = 0usize;
      let bytecode = luau_compile(
        source.as_ptr() as *const c_char,
        source.len(),
        null_mut(),
        &mut bytecode_size,
      );
      assert!(!bytecode.is_null());

      let result = luau_load(l, c"=test".as_ptr(), bytecode, bytecode_size, 0);
      free(bytecode as *mut c_void);
      assert_eq!(0, result);

      let mut stats = CompilationStats::default();
      let options = CompilationOptions::default();
      let _ = compile_internal(&None, l, -1, &options, &mut stats);
      stats.native_code_size_bytes
    }
  };

  assert_eq!(compile(), compile());
}

#[cfg(test)]
#[test]
fn conformance_codegen_randomize_code_size_non_decreasing() {
  use core::ffi::{c_char, c_void};

  use ulua_code_gen::{
    functions::{
      compile_internal::compile_internal, luau_codegen_create::luau_codegen_create,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{compilation_options::CompilationOptions, compilation_stats::CompilationStats},
  };
  use ulua_compiler::functions::luau_compile::luau_compile;
  use ulua_vm::functions::{lua_l_newstate::lua_l_newstate, luau_load::luau_load};

  use crate::common::{functions::run_conformance::CODEGEN, type_aliases::state_ref::StateRef};

  unsafe extern "C" {
    fn free(ptr: *mut c_void);
  }

  if unsafe { !CODEGEN } || luau_codegen_supported() == 0 {
    return;
  }

  let source = r#"
        local function classify(x)
            if x > 0 then
                return "positive"
            elseif x < 0 then
                return "negative"
            else
                return "zero"
            end
        end
        return classify(1)
    "#;

  let compile = |nop_padding: bool| -> usize {
    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
      luau_codegen_create(l);

      let mut bytecode_size = 0usize;
      let bytecode = luau_compile(
        source.as_ptr() as *const c_char,
        source.len(),
        null_mut(),
        &mut bytecode_size,
      );
      assert!(!bytecode.is_null());

      let result = luau_load(l, c"=test".as_ptr(), bytecode, bytecode_size, 0);
      free(bytecode as *mut c_void);
      assert_eq!(0, result);

      let options = CompilationOptions {
        nop_padding,
        ..Default::default()
      };
      let mut stats = CompilationStats::default();
      let _ = compile_internal(&None, l, -1, &options, &mut stats);
      stats.native_code_size_bytes
    }
  };

  assert!(compile(true) >= compile(false));
}

#[cfg(test)]
#[test]
fn conformance_codegen_randomize_functional_correctness() {
  use core::ffi::{CStr, c_char, c_void};

  use ulua_code_gen::{
    functions::{
      compile_internal::compile_internal, luau_codegen_create::luau_codegen_create,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::compilation_options::CompilationOptions,
  };
  use ulua_compiler::functions::luau_compile::luau_compile;
  use ulua_vm::{
    functions::{
      lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
      lua_l_sandboxthread::lua_l_sandboxthread, lua_pcall::lua_pcall, luau_load::luau_load,
    },
    macros::{lua_tonumber::lua_tonumber, lua_tostring::lua_tostring},
  };

  use crate::common::{functions::run_conformance::CODEGEN, type_aliases::state_ref::StateRef};

  unsafe extern "C" {
    fn free(ptr: *mut c_void);
  }

  if unsafe { !CODEGEN } || luau_codegen_supported() == 0 {
    return;
  }

  let source = r#"
        local function add(a, b) return a + b end
        return add(10, 32)
    "#;

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    luau_codegen_create(l);
    lua_l_openlibs(l);
    lua_l_sandbox(l);
    lua_l_sandboxthread(l);

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      null_mut(),
      &mut bytecode_size,
    );
    assert!(!bytecode.is_null());

    let load_result = luau_load(l, c"=test".as_ptr(), bytecode, bytecode_size, 0);
    free(bytecode as *mut c_void);
    assert_eq!(0, load_result);

    let nop_options = CompilationOptions {
      nop_padding: true,
      ..Default::default()
    };
    let _ = compile_internal(&None, l, -1, &nop_options, null_mut());

    let call_result = lua_pcall(l, 0, 1, 0);
    if call_result != 0 {
      let message = CStr::from_ptr(lua_tostring!(l, -1)).to_string_lossy();
      panic!("lua_pcall failed: {message}");
    }

    assert_eq!(42.0, lua_tonumber!(l, -1));
  }
}

#[cfg(test)]
#[test]
fn conformance_codegen_supported() {
  use ulua_code_gen::functions::luau_codegen_supported::luau_codegen_supported;

  use crate::common::functions::run_conformance::CODEGEN;

  if unsafe { CODEGEN } && luau_codegen_supported() == 0 {
    eprintln!(
      "Native code generation is not supported by the current configuration and will be disabled"
    );
  }
}

#[cfg(test)]
#[test]
fn conformance_constructs() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"constructs.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_coroutine() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"coroutine.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_coverage() {
  use ulua_compiler::records::lua_compile_options::LuaCompileOptions;

  use crate::common::functions::{
    conformance_coverage_setup::conformance_coverage_setup, run_conformance::run_conformance,
  };

  let mut copts = LuaCompileOptions {
    optimization_level: 1,
    debug_level: 1,
    type_info_level: 1,
    coverage_level: 2,
    vector_lib: null(),
    vector_ctor: null(),
    vector_type: null(),
    mutable_globals: null(),
    userdata_types: null(),
    libraries_with_known_members: null(),
    library_member_type_cb: None,
    library_member_constant_cb: None,
    disabled_builtins: null(),
  };

  unsafe {
    run_conformance(
      c"coverage.luau".as_ptr(),
      Some(conformance_coverage_setup),
      None,
      null_mut(),
      &mut copts,
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_date_time() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"datetime.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_debug() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"debug.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_debug_api() {
  use ulua_vm::{
    functions::{
      lua_getinfo::lua_getinfo, lua_l_newstate::lua_l_newstate, lua_pushnumber::lua_pushnumber,
    },
    records::lua_debug::LuaDebug,
  };

  use crate::common::type_aliases::state_ref::StateRef;

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    lua_pushnumber(l, 10.0);

    let mut ar: LuaDebug = zeroed();
    assert_eq!(lua_getinfo(l, -1, c"f".as_ptr(), &mut ar), 0);
    assert_eq!(lua_getinfo(l, -10, c"f".as_ptr(), &mut ar), 0);
  }
}

#[cfg(test)]
#[test]
fn conformance_debugger() {
  use core::sync::atomic::Ordering;

  use ulua_compiler::records::lua_compile_options::LuaCompileOptions;

  use crate::common::{
    functions::{
      conformance_debugger_setup::conformance_debugger_setup,
      conformance_debugger_yield::conformance_debugger_yield, run_conformance::run_conformance,
    },
    records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE,
  };

  for singlestep in [false, true] {
    CONFORMANCE_DEBUGGER_STATE.reset(singlestep);

    let mut copts = LuaCompileOptions {
      optimization_level: 1,
      debug_level: 2,
      type_info_level: 1,
      coverage_level: 0,
      vector_lib: null(),
      vector_ctor: null(),
      vector_type: null(),
      mutable_globals: null(),
      userdata_types: null(),
      libraries_with_known_members: null(),
      library_member_type_cb: None,
      library_member_constant_cb: None,
      disabled_builtins: null(),
    };

    unsafe {
      run_conformance(
        c"debugger.luau".as_ptr(),
        Some(conformance_debugger_setup),
        Some(conformance_debugger_yield),
        null_mut(),
        &mut copts,
        true,
        null_mut(),
      )
    };
    assert_eq!(
      CONFORMANCE_DEBUGGER_STATE.breakhits.load(Ordering::SeqCst),
      16
    );

    if singlestep {
      assert!(CONFORMANCE_DEBUGGER_STATE.stephits.load(Ordering::SeqCst) > 100);
    }
  }
}

#[cfg(test)]
#[test]
fn conformance_errors() {
  use ulua_common::FFlag;

  use crate::common::{
    functions::run_conformance::run_conformance, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _luau_xpcall_fix = ScopedFastFlag::new(&FFlag::LuauXpcallFixMessageYieldPath, true);

  unsafe {
    run_conformance(
      c"errors.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_events() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"events.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_exception_object() {
  use ulua_vm::functions::lua_newstate::lua_newstate;

  use crate::common::functions::{
    conformance_exception_object_capture_exception::conformance_exception_object_capture_exception,
    ends_with::ends_with, limited_realloc::limited_realloc, run_conformance::run_conformance,
  };

  let global_state = unsafe {
    run_conformance(
      c"exceptions.luau".as_ptr(),
      None,
      None,
      lua_newstate(Some(limited_realloc), null_mut()),
      null_mut(),
      false,
      null_mut(),
    )
  };
  let l = global_state.as_ptr();

  unsafe {
    let result =
      conformance_exception_object_capture_exception(l, c"infinite_recursion_error".as_ptr());
    assert!(result.exception_generated);

    let result = conformance_exception_object_capture_exception(l, c"empty_function".as_ptr());
    assert!(!result.exception_generated);

    let result =
      conformance_exception_object_capture_exception(l, c"pass_number_to_error".as_ptr());
    assert!(result.exception_generated);
    assert!(ends_with(&result.description, "42"));

    let result =
      conformance_exception_object_capture_exception(l, c"pass_string_to_error".as_ptr());
    assert!(result.exception_generated);
    assert!(ends_with(&result.description, "string argument"));

    let result = conformance_exception_object_capture_exception(l, c"pass_table_to_error".as_ptr());
    assert!(result.exception_generated);

    let result =
      conformance_exception_object_capture_exception(l, c"large_allocation_error".as_ptr());
    assert!(result.exception_generated);
  }
}

#[cfg(test)]
#[test]
fn conformance_explicit_type_instantiations() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"explicit_type_instantiations.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_gc() {
  use ulua_vm::functions::lua_newstate::lua_newstate;

  use crate::common::functions::{
    blockable_realloc::blockable_realloc, conformance_gc_setup::conformance_gc_setup,
    run_conformance::run_conformance,
  };

  let initial_lua_state = unsafe { lua_newstate(Some(blockable_realloc), null_mut()) };

  unsafe {
    run_conformance(
      c"gc.luau".as_ptr(),
      Some(conformance_gc_setup),
      None,
      initial_lua_state,
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_gc_dump() {
  use core::ffi::{c_char, c_int, c_void};
  use std::ffi::{CStr, CString};

  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_c_dump::lua_c_dump, lua_c_enumheap::lua_c_enumheap, lua_c_fullgc::luaC_fullgc,
      lua_createtable::lua_createtable, lua_l_newstate::lua_l_newstate,
      lua_newbuffer::lua_newbuffer, lua_newthread::lua_newthread, lua_pushinteger::lua_pushinteger,
      lua_pushstring::lua_pushstring, lua_pushvalue::lua_pushvalue, lua_rawseti::lua_rawseti,
      lua_resume::lua_resume, lua_setfield::lua_setfield, lua_setmetatable::lua_setmetatable,
    },
    macros::{
      LUA_PUSHCCLOSURE::LUA_PUSHCCLOSURE, lua_newuserdata::lua_newuserdata,
      lua_tostring::lua_tostring,
    },
    records::lua_state::lua_State,
  };

  use crate::common::{
    functions::{
      conformance_gc_dump_edge::conformance_gc_dump_edge,
      conformance_gc_dump_node::conformance_gc_dump_node, lua_loadstring::lua_loadstring,
      lua_silence::lua_silence,
    },
    records::conformance_gc_dump_enum_context::ConformanceGcDumpEnumContext,
    type_aliases::state_ref::StateRef,
  };

  unsafe extern "C" {
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(file: *mut c_void) -> c_int;
  }

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    lua_createtable(l, 1, 2);
    lua_pushstring(l, c"value".as_ptr());
    lua_setfield(l, -2, c"key".as_ptr());

    lua_pushstring(l, c"u42".as_ptr());
    lua_setfield(l, -2, c"__type".as_ptr());

    lua_pushinteger(l, 42);
    lua_rawseti(l, -2, 1000);

    lua_pushinteger(l, 42);
    lua_rawseti(l, -2, 1);

    lua_pushvalue(l, -1);
    lua_setmetatable(l, -2);

    lua_newuserdata(l, 42);
    lua_pushvalue(l, -2);
    lua_setmetatable(l, -2);

    lua_pushinteger(l, 1);
    LUA_PUSHCCLOSURE(l, Some(lua_silence), c"test".as_ptr(), 1);

    lua_newbuffer(l, 100);

    let cl: *mut lua_State = lua_newthread(l);
    let source = CString::new(
      r#"
local x
x = {}
local function f()
    x[1] = math.abs(42)
end
function foo()
    x[2] = ''
    for i = 1, 10000 do x[2] ..= '1234567890' end
end
foo()
return f
"#,
    )
    .expect("script source contains nul");

    lua_pushstring(cl, source.as_ptr());
    lua_pushstring(cl, c"=GCDump".as_ptr());
    assert_eq!(lua_loadstring(cl), 1);

    let status = lua_resume(cl, null_mut(), 0);
    if status != LuaStatus::Ok as c_int {
      let error = CStr::from_ptr(lua_tostring!(cl, -1)).to_string_lossy();
      panic!("GCDump setup chunk failed: {error}");
    }

    #[cfg(windows)]
    let path = c"NUL".as_ptr();
    #[cfg(not(windows))]
    let path = c"/dev/null".as_ptr();

    let file = fopen(path, c"w".as_ptr());
    assert!(!file.is_null());

    luaC_fullgc(l);
    lua_c_dump(l, file, None);
    assert_eq!(fclose(file), 0);

    let mut context = ConformanceGcDumpEnumContext::default();
    lua_c_enumheap(
      l,
      &mut context as *mut ConformanceGcDumpEnumContext as *mut c_void,
      Some(conformance_gc_dump_node),
      Some(conformance_gc_dump_edge),
    );

    assert!(
      context.errors.is_empty(),
      "GCDump enum validation errors: {:?}",
      context.errors
    );
    assert!(!context.nodes.is_empty());
    assert!(!context.edges.is_empty());
    assert!(context.seen_target_string);
  }
}

#[cfg(test)]
#[test]
fn conformance_huge_constant_table() {
  use core::ffi::{c_char, c_void};
  use std::string::String;

  use ulua_code_gen::{
    enums::code_gen_flags::CodeGenFlags,
    functions::{
      compile_internal::compile_internal, luau_codegen_create::luau_codegen_create,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::compilation_options::CompilationOptions,
  };
  use ulua_compiler::functions::luau_compile::luau_compile;
  use ulua_vm::{
    functions::{
      lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
      lua_l_sandboxthread::lua_l_sandboxthread, lua_resume::lua_resume, luau_load::luau_load,
    },
    macros::lua_tonumber::lua_tonumber,
  };

  use crate::common::{functions::run_conformance::CODEGEN, type_aliases::state_ref::StateRef};

  unsafe extern "C" {
    fn free(ptr: *mut c_void);
  }

  let mut source = String::from("function foo(...)\n");
  source.push_str("    local args = ...\n");
  source.push_str("    local t = args and {\n");

  for i in 0..400 {
    for k in 0..100 {
      source.push_str("call(");
      source.push_str(&format!("{}", i * 100 + k));
      source.push_str(".125), ");
    }

    source.push_str("\n        ");
  }

  source.push_str("    }\n");
  source.push_str("    return { a = 1, b = 2, c = 3 }\n");
  source.push_str("end\n");
  source.push_str("return foo().a + foo().b\n");

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    if CODEGEN && luau_codegen_supported() != 0 {
      luau_codegen_create(l);
    }

    lua_l_openlibs(l);
    lua_l_sandbox(l);
    lua_l_sandboxthread(l);

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      null_mut(),
      &mut bytecode_size,
    );
    assert!(!bytecode.is_null());

    let result = luau_load(
      l,
      c"=HugeConstantTable".as_ptr(),
      bytecode,
      bytecode_size,
      0,
    );
    free(bytecode as *mut c_void);

    assert_eq!(0, result);

    if CODEGEN && luau_codegen_supported() != 0 {
      let native_options = CompilationOptions {
        flags: CodeGenFlags::CodeGenColdFunctions as u32,
        ..Default::default()
      };
      let _ = compile_internal(&None, l, -1, &native_options, null_mut());
    }

    let status = lua_resume(l, null_mut(), 0);
    assert_eq!(0, status);

    assert_eq!(3.0, lua_tonumber!(l, -1));
  }
}

#[cfg(test)]
#[test]
fn conformance_huge_function() {
  use core::ffi::{c_char, c_void};

  use ulua_code_gen::{
    enums::code_gen_flags::CodeGenFlags,
    functions::{
      compile_internal::compile_internal, luau_codegen_create::luau_codegen_create,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::compilation_options::CompilationOptions,
  };
  use ulua_compiler::functions::luau_compile::luau_compile;
  use ulua_vm::{
    functions::{
      lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
      lua_l_sandboxthread::lua_l_sandboxthread, lua_resume::lua_resume, luau_load::luau_load,
    },
    macros::lua_tonumber::lua_tonumber,
  };

  use crate::common::{
    functions::{make_huge_function_source::make_huge_function_source, run_conformance::CODEGEN},
    type_aliases::state_ref::StateRef,
  };

  unsafe extern "C" {
    fn free(ptr: *mut c_void);
  }

  let source = make_huge_function_source();
  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    if CODEGEN && luau_codegen_supported() != 0 {
      luau_codegen_create(l);
    }

    lua_l_openlibs(l);
    lua_l_sandbox(l);
    lua_l_sandboxthread(l);

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      null_mut(),
      &mut bytecode_size,
    );
    assert!(!bytecode.is_null());

    let result = luau_load(l, c"=HugeFunction".as_ptr(), bytecode, bytecode_size, 0);
    free(bytecode as *mut c_void);

    assert_eq!(0, result);

    if CODEGEN && luau_codegen_supported() != 0 {
      let native_options = CompilationOptions {
        flags: CodeGenFlags::CodeGenColdFunctions as u32,
        ..Default::default()
      };
      let _ = compile_internal(&None, l, -1, &native_options, null_mut());
    }

    let status = lua_resume(l, null_mut(), 0);
    assert_eq!(0, status);

    assert_eq!(42.0, lua_tonumber!(l, -1));
  }
}

#[cfg(test)]
#[test]
fn conformance_huge_function_load_failure() {
  use core::{
    ffi::{CStr, c_char, c_void},
    sync::atomic::Ordering,
  };

  use ulua_compiler::functions::luau_compile::luau_compile;
  use ulua_vm::{
    functions::{
      lua_c_fullgc::luaC_fullgc, lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
      lua_l_sandboxthread::lua_l_sandboxthread, lua_newstate::lua_newstate, luau_load::luau_load,
    },
    macros::lua_tostring::lua_tostring,
  };

  use crate::common::{
    functions::{
      huge_function_load_failure_test_allocate::{
        HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT,
        HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_TO_FAIL,
        huge_function_load_failure_test_allocate,
      },
      make_huge_function_source::make_huge_function_source,
    },
    type_aliases::state_ref::StateRef,
  };

  unsafe extern "C" {
    fn free(ptr: *mut c_void);
  }

  let source = make_huge_function_source();
  let expected_total_large_allocations = 2usize;

  unsafe {
    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      null_mut(),
      &mut bytecode_size,
    );
    assert!(!bytecode.is_null());

    let mut large_allocation_to_fail = 0usize;
    while large_allocation_to_fail != expected_total_large_allocations {
      HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_TO_FAIL
        .store(large_allocation_to_fail, Ordering::SeqCst);
      HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT.store(0, Ordering::SeqCst);

      let global_state = StateRef::new(lua_newstate(
        Some(huge_function_load_failure_test_allocate),
        null_mut(),
      ))
      .expect("lua state allocation failed");
      let l = global_state.as_ptr();

      lua_l_openlibs(l);
      lua_l_sandbox(l);
      lua_l_sandboxthread(l);

      let status = luau_load(l, c"=HugeFunction".as_ptr(), bytecode, bytecode_size, 0);
      assert_eq!(status, 1);

      assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"not enough memory");

      luaC_fullgc(l);

      large_allocation_to_fail += 1;
    }

    free(bytecode as *mut c_void);

    assert_eq!(large_allocation_to_fail, expected_total_large_allocations);
  }
}

#[cfg(test)]
#[test]
fn conformance_if_else_expression() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"ifelseexpr.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_integers() {
  use ulua_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
  use ulua_common::FFlag;

  use crate::common::{
    functions::{
      run_conformance::{CODEGEN, run_conformance},
      setup_native_helpers::setup_native_helpers,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _ncg_buffer_integer = ScopedFastFlag::new(&FFlag::LuauCodegenBufferInteger, true);
  let _luau_codegen_fix_buffer_len_check =
    ScopedFastFlag::new(&FFlag::LuauCodegenFixBufferLenCheck, true);

  if FFlag::LuauIntegerType2.get() && FFlag::LuauIntegerLibrary.get() {
    unsafe {
      run_conformance(
        c"integers.luau".as_ptr(),
        Some(setup_native_helpers),
        None,
        null_mut(),
        null_mut(),
        false,
        null_mut(),
      )
    };
    if unsafe { CODEGEN } && luau_codegen_supported() != 0 {
      unsafe {
        run_conformance(
          c"integers_regspill.luau".as_ptr(),
          Some(setup_native_helpers),
          None,
          null_mut(),
          null_mut(),
          false,
          null_mut(),
        )
      };
    }
  }
}

#[cfg(test)]
#[test]
fn conformance_interrupt() {
  use core::ffi::c_int;
  use std::ffi::{CStr, CString};

  use ulua_compiler::records::lua_compile_options::LuaCompileOptions;
  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_callbacks::lua_callbacks, lua_getfield::lua_getfield,
      lua_l_checklstring::lua_l_checklstring, lua_newthread::lua_newthread, lua_resume::lua_resume,
    },
    macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_pop::lua_pop},
  };

  use crate::common::{
    functions::{
      conformance_interrupt_interrupt::conformance_interrupt_interrupt,
      run_conformance::run_conformance,
    },
    records::conformance_interrupt_state::{
      CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS, CONFORMANCE_INTERRUPT_MODE_INFLOOP,
      CONFORMANCE_INTERRUPT_MODE_TIMEOUT, CONFORMANCE_INTERRUPT_STATE,
    },
  };

  let mut copts = LuaCompileOptions {
    optimization_level: 1,
    debug_level: 1,
    type_info_level: 1,
    coverage_level: 0,
    vector_lib: null(),
    vector_ctor: null(),
    vector_type: null(),
    mutable_globals: null(),
    userdata_types: null(),
    libraries_with_known_members: null(),
    library_member_type_cb: None,
    library_member_constant_cb: None,
    disabled_builtins: null(),
  };

  let global_state = unsafe {
    run_conformance(
      c"interrupt.luau".as_ptr(),
      None,
      None,
      null_mut(),
      &mut copts,
      false,
      null_mut(),
    )
  };
  let l = global_state.as_ptr();

  unsafe {
    (*lua_callbacks(l)).interrupt = Some(conformance_interrupt_interrupt);

    {
      let t = lua_newthread(l);
      lua_getfield(t, LUA_GLOBALSINDEX, c"test".as_ptr());

      CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS);
      let mut status = lua_resume(t, null_mut(), 0);
      assert_eq!(status, LuaStatus::Yield as c_int);
      assert_eq!(CONFORMANCE_INTERRUPT_STATE.index(), 4);

      status = lua_resume(t, null_mut(), 0);
      assert_eq!(status, LuaStatus::Ok as c_int);
      assert_eq!(CONFORMANCE_INTERRUPT_STATE.index(), 22);

      lua_pop(l, 1);
    }

    for test in 1..=10 {
      let t = lua_newthread(l);
      let name = CString::new(format!("infloop{test}")).expect("name contains nul");
      lua_getfield(t, LUA_GLOBALSINDEX, name.as_ptr());

      CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_INFLOOP);
      let status = lua_resume(t, null_mut(), 0);
      assert_eq!(status, LuaStatus::Yield as c_int);
      assert_eq!(CONFORMANCE_INTERRUPT_STATE.index(), 11);

      lua_pop(l, 1);
    }

    CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_TIMEOUT);

    for test in 1..=6 {
      let t = lua_newthread(l);
      let name = CString::new(format!("hang{test}")).expect("name contains nul");
      lua_getfield(t, LUA_GLOBALSINDEX, name.as_ptr());

      CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_TIMEOUT);
      let status = lua_resume(t, null_mut(), 0);
      assert_eq!(status, LuaStatus::ErrRun as c_int);

      let mut len = 0usize;
      let error = lua_l_checklstring(t, -1, &mut len);
      let error = CStr::from_ptr(error).to_string_lossy();
      assert!(
        error.contains("timeout"),
        "expected timeout error, got {error}"
      );

      lua_pop(l, 1);
    }

    {
      let t = lua_newthread(l);
      lua_getfield(t, LUA_GLOBALSINDEX, c"hangpcall".as_ptr());

      CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_TIMEOUT);
      let status = lua_resume(t, null_mut(), 0);
      assert_eq!(status, LuaStatus::Ok as c_int);

      lua_pop(l, 1);
    }
  }
}

#[cfg(test)]
#[test]
fn conformance_interrupt_error_inspection() {
  use core::ffi::{c_char, c_void};

  use ulua_compiler::functions::luau_compile::luau_compile;
  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_callbacks::lua_callbacks, lua_getinfo::lua_getinfo, lua_l_newstate::lua_l_newstate,
      lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
      lua_l_sandboxthread::lua_l_sandboxthread, lua_resume::lua_resume,
      luau_callhook::luau_callhook, luau_load::luau_load,
    },
    records::lua_debug::LuaDebug,
  };

  use crate::common::{
    functions::{
      conformance_interrupt_error_inspection_interrupt::conformance_interrupt_error_inspection_interrupt,
      conformance_interrupt_inspection_hook::conformance_interrupt_inspection_hook,
    },
    records::conformance_interrupt_error_inspection_state::CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE,
    type_aliases::state_ref::StateRef,
  };

  unsafe extern "C" {
    fn free(ptr: *mut c_void);
  }

  let source = r#"
function fib(n)
    return n < 2 and 1 or fib(n - 1) + fib(n - 2)
end

fib(5)
"#;

  for target in 0..20 {
    CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE.reset(target);

    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
      lua_l_openlibs(l);
      lua_l_sandbox(l);
      lua_l_sandboxthread(l);

      let mut bytecode_size = 0usize;
      let bytecode = luau_compile(
        source.as_ptr() as *const c_char,
        source.len(),
        null_mut(),
        &mut bytecode_size,
      );
      assert!(!bytecode.is_null());

      let result = luau_load(
        l,
        c"=InterruptErrorInspection".as_ptr(),
        bytecode,
        bytecode_size,
        0,
      );
      free(bytecode as *mut c_void);

      assert_eq!(LuaStatus::Ok as i32, result);

      (*lua_callbacks(l)).interrupt = Some(conformance_interrupt_error_inspection_interrupt);

      lua_resume(l, null_mut(), 0);

      let mut ar: LuaDebug = zeroed();
      assert_ne!(0, lua_getinfo(l, 0, c"nsl".as_ptr(), &mut ar));

      luau_callhook(l, Some(conformance_interrupt_inspection_hook), null_mut());
    }
  }
}

#[cfg(test)]
#[test]
fn conformance_interrupt_inspection() {
  use crate::common::functions::{
    conformance_interrupt_inspection_setup::conformance_interrupt_inspection_setup,
    conformance_interrupt_inspection_yield::conformance_interrupt_inspection_yield,
    run_conformance::run_conformance,
  };

  unsafe {
    run_conformance(
      c"basic.luau".as_ptr(),
      Some(conformance_interrupt_inspection_setup),
      Some(conformance_interrupt_inspection_yield),
      null_mut(),
      null_mut(),
      true,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_ir_instruction_limit() {
  use core::ffi::{c_char, c_void};
  use std::string::String;

  use ulua_code_gen::{
    enums::{code_gen_compilation_result::CodeGenCompilationResult, code_gen_flags::CodeGenFlags},
    functions::{
      compile_internal::compile_internal, luau_codegen_create::luau_codegen_create,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{compilation_options::CompilationOptions, compilation_stats::CompilationStats},
  };
  use ulua_common::FInt;
  use ulua_compiler::functions::luau_compile::luau_compile;
  use ulua_vm::functions::{
    lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
    lua_l_sandboxthread::lua_l_sandboxthread, luau_load::luau_load,
  };

  use crate::common::{
    functions::run_conformance::CODEGEN,
    type_aliases::{scoped_fast_int::ScopedFastInt, state_ref::StateRef},
  };

  unsafe extern "C" {
    fn free(ptr: *mut c_void);
  }

  if unsafe { !CODEGEN } || luau_codegen_supported() == 0 {
    return;
  }

  let _codegen_heuristics_instruction_limit =
    ScopedFastInt::new(&FInt::CodegenHeuristicsInstructionLimit, 50_000);

  let mut source = String::new();

  for function_index in 0..100 {
    source.push_str("local function fn");
    source.push_str(&format!("{}", function_index));
    source.push_str("(...)\n");
    source.push_str("if ... then\n");
    source.push_str("local p1, p2 = ...\n");
    source.push_str("local _ = {\n");

    for i in 0..100 {
      source.push_str("p1*0.");
      source.push_str(&format!("{}", i));
      source.push(',');
      source.push_str("p2+0.");
      source.push_str(&format!("{}", i));
      source.push(',');
    }

    source.push_str("}\n");
    source.push_str("return _\n");
    source.push_str("end\n");
    source.push_str("end\n");
  }

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    luau_codegen_create(l);

    lua_l_openlibs(l);
    lua_l_sandbox(l);
    lua_l_sandboxthread(l);

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      null_mut(),
      &mut bytecode_size,
    );
    assert!(!bytecode.is_null());

    let result = luau_load(l, c"=HugeFunction".as_ptr(), bytecode, bytecode_size, 0);
    free(bytecode as *mut c_void);

    assert_eq!(0, result);

    let native_options = CompilationOptions {
      flags: CodeGenFlags::CodeGenColdFunctions as u32,
      ..Default::default()
    };
    let mut native_stats = CompilationStats::default();
    let native_result = compile_internal(&None, l, -1, &native_options, &mut native_stats);

    assert_eq!(CodeGenCompilationResult::Success, native_result.result);
    assert!(native_result.has_errors());
    assert!(!native_result.proto_failures.is_empty());

    let first_failure = &native_result.proto_failures[0];
    assert_eq!(
      CodeGenCompilationResult::CodeGenOverflowInstructionLimit,
      first_failure.result
    );
    assert_ne!(-1, first_failure.line);
    assert_ne!("", first_failure.debugname);

    assert!(native_stats.functions_compiled > 0);
    assert!(native_stats.functions_compiled < 101);
  }
}

#[cfg(test)]
#[test]
fn conformance_iter() {
  use ulua_common::FFlag;

  use crate::common::{
    functions::{conformance_iter_setup::conformance_iter_setup, run_conformance::run_conformance},
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _luau_yield_iter = ScopedFastFlag::new(&FFlag::LuauYieldIter2, true);

  unsafe {
    run_conformance(
      c"iter.luau".as_ptr(),
      Some(conformance_iter_setup),
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_jit_inliner() {
  use core::{ffi::c_int, ptr::null_mut, sync::atomic::Ordering};
  use std::ffi::{CStr, CString};

  use ulua_common::FFlag;
  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_callbacks::lua_callbacks, lua_newthread::lua_newthread, lua_resume::lua_resume,
    },
    macros::{lua_getglobal::lua_getglobal, lua_pop::lua_pop, lua_tostring::lua_tostring},
  };

  use crate::common::{
    functions::{
      conformance_jit_inliner_interrupt::{JIT_INLINER_INDEX, conformance_jit_inliner_interrupt},
      run_conformance::run_conformance,
    },
    type_aliases::{scoped_fast_flag::ScopedFastFlag, state_ref::StateRef},
  };

  let _luau_emit_call_feedback = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);

  let global_state: StateRef = unsafe {
    run_conformance(
      c"jit_inliner.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      true,
      null_mut(),
    )
  };
  let l = global_state.as_ptr();

  unsafe {
    (*lua_callbacks(l)).interrupt = Some(conformance_jit_inliner_interrupt);
  }

  // 对应 C++ 的 "fuzzfail_infinite" + std::to_string(test)
  let global_name = |test: u32| CString::new(format!("fuzzfail_infinite{test}"));
  for test in 1..=2 {
    let t = unsafe { lua_newthread(l) };

    let name = global_name(test).expect("global name contains nul");
    unsafe { lua_getglobal(t, name.as_ptr()) };

    JIT_INLINER_INDEX.store(0, Ordering::SeqCst);
    let status = unsafe { lua_resume(t, null_mut(), 0) };
    assert_eq!(status, LuaStatus::ErrRun as c_int);

    let top = unsafe { lua_tostring!(t, -1) };
    assert!(!top.is_null());
    let text = unsafe { CStr::from_ptr(top) }.to_string_lossy();
    assert!(
      text.contains("timeout"),
      "expected timeout error, got {text}"
    );

    unsafe { lua_pop(l, 1) };
  }
}

#[cfg(test)]
#[test]
fn conformance_iter_fenv() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"iter_fenv.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_large_nested_closure() {
  use core::ffi::{c_char, c_void};
  use std::string::String;

  use ulua_code_gen::{
    enums::code_gen_flags::CodeGenFlags,
    functions::{
      compile_internal::compile_internal, luau_codegen_create::luau_codegen_create,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::compilation_options::CompilationOptions,
  };
  use ulua_compiler::functions::luau_compile::luau_compile;
  use ulua_vm::{
    functions::{
      lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
      lua_l_sandboxthread::lua_l_sandboxthread, lua_resume::lua_resume, luau_load::luau_load,
    },
    macros::lua_tonumber::lua_tonumber,
  };

  use crate::common::{functions::run_conformance::CODEGEN, type_aliases::state_ref::StateRef};

  unsafe extern "C" {
    fn free(ptr: *mut c_void);
  }

  const K_COUNT: usize = 2048;

  let mut source = String::new();
  source.push_str("local function test()\n");
  source.push_str("local x = 0\n");

  for i in 0..K_COUNT {
    let n = i + 1;
    source.push_str("    function f");
    source.push_str(&format!("{}", n));
    source.push_str("() x = x + 1; return ");
    source.push_str(&format!("{}", n));
    source.push_str(" end\n");
  }

  source.push_str("    return f");
  source.push_str(&format!("{}", K_COUNT));
  source.push('\n');
  source.push_str("end\n");
  source.push_str("return test()()\n");

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    if CODEGEN && luau_codegen_supported() != 0 {
      luau_codegen_create(l);
    }

    lua_l_openlibs(l);
    lua_l_sandbox(l);
    lua_l_sandboxthread(l);

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      null_mut(),
      &mut bytecode_size,
    );
    assert!(!bytecode.is_null());

    let result = luau_load(
      l,
      c"=LargeNestedClosure".as_ptr(),
      bytecode,
      bytecode_size,
      0,
    );
    free(bytecode as *mut c_void);

    assert_eq!(0, result);

    if CODEGEN && luau_codegen_supported() != 0 {
      let native_options = CompilationOptions {
        flags: CodeGenFlags::CodeGenColdFunctions as u32,
        ..Default::default()
      };
      let _ = compile_internal(&None, l, -1, &native_options, null_mut());
    }

    let status = lua_resume(l, null_mut(), 0);
    assert_eq!(0, status);

    assert_eq!(K_COUNT as f64, lua_tonumber!(l, -1));
  }
}

#[cfg(test)]
#[test]
fn conformance_lightuserdata_api() {
  use core::ffi::{CStr, c_void};

  use ulua_vm::{
    functions::{
      lua_createtable::lua_createtable, lua_getlightuserdataname::lua_getlightuserdataname,
      lua_gettable::lua_gettable, lua_l_newstate::lua_l_newstate, lua_l_typename::lua_l_typename,
      lua_lightuserdatatag::lua_lightuserdatatag, lua_pushinteger::lua_pushinteger,
      lua_pushlightuserdatatagged::lua_pushlightuserdatatagged, lua_pushstring::lua_pushstring,
      lua_rawequal::lua_rawequal, lua_setfield::lua_setfield,
      lua_setlightuserdataname::lua_setlightuserdataname, lua_setmetatable::lua_setmetatable,
      lua_settable::lua_settable, lua_tolightuserdatatagged::lua_tolightuserdatatagged,
    },
    macros::lua_pop::lua_pop,
  };

  use crate::common::type_aliases::state_ref::StateRef;

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  let value = 0x12345678usize as *mut c_void;

  unsafe {
    lua_pushlightuserdatatagged(l, value, 1);
    assert_eq!(lua_lightuserdatatag(l, -1), 1);
    assert!(lua_tolightuserdatatagged(l, -1, 0).is_null());
    assert_eq!(lua_tolightuserdatatagged(l, -1, 1), value);

    lua_setlightuserdataname(l, 1, c"id".as_ptr());
    assert!(lua_getlightuserdataname(l, 0).is_null());
    assert_eq!(CStr::from_ptr(lua_getlightuserdataname(l, 1)), c"id");
    assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"id");
    lua_pop(l, 1);

    lua_pushlightuserdatatagged(l, value, 0);
    lua_pushlightuserdatatagged(l, value, 1);
    assert_eq!(lua_rawequal(l, -1, -2), 0);
    lua_pop(l, 2);

    lua_createtable(l, 0, 0);

    lua_pushlightuserdatatagged(l, value, 2);
    lua_pushinteger(l, 20);
    lua_settable(l, -3);
    lua_pushlightuserdatatagged(l, value, 3);
    lua_pushinteger(l, 30);
    lua_settable(l, -3);

    lua_pushlightuserdatatagged(l, value, 2);
    lua_gettable(l, -2);
    lua_pushinteger(l, 20);
    assert_eq!(lua_rawequal(l, -1, -2), 1);
    lua_pop(l, 2);

    lua_pushlightuserdatatagged(l, value, 3);
    lua_gettable(l, -2);
    lua_pushinteger(l, 30);
    assert_eq!(lua_rawequal(l, -1, -2), 1);
    lua_pop(l, 2);

    lua_pop(l, 1);

    lua_pushlightuserdatatagged(l, value, 0);
    assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"userdata");

    lua_createtable(l, 0, 1);
    lua_pushstring(l, c"luserdata".as_ptr());
    lua_setfield(l, -2, c"__type".as_ptr());
    assert_eq!(lua_setmetatable(l, -2), 1);

    assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"luserdata");
    lua_pop(l, 1);
  }
}

#[cfg(test)]
#[test]
fn conformance_literals() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"literals.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_locals() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"locals.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_math() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"math.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_move() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"move.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_n_debug_get_up_value() {
  use ulua_compiler::records::lua_compile_options::LuaCompileOptions;

  use crate::common::functions::{
    conformance_n_debug_get_up_value_yield::conformance_n_debug_get_up_value_yield,
    run_conformance::run_conformance,
  };

  let mut copts = LuaCompileOptions {
    optimization_level: 0,
    debug_level: 0,
    type_info_level: 1,
    coverage_level: 0,
    vector_lib: null(),
    vector_ctor: null(),
    vector_type: null(),
    mutable_globals: null(),
    userdata_types: null(),
    libraries_with_known_members: null(),
    library_member_type_cb: None,
    library_member_constant_cb: None,
    disabled_builtins: null(),
  };

  unsafe {
    run_conformance(
      c"ndebug_upvalues.luau".as_ptr(),
      None,
      Some(conformance_n_debug_get_up_value_yield),
      null_mut(),
      &mut copts,
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_native() {
  use ulua_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
  use ulua_common::FFlag;
  use ulua_compiler::records::lua_compile_options::LuaCompileOptions;

  use crate::common::{
    functions::{
      run_conformance::{CODEGEN, run_conformance},
      setup_native_helpers::setup_native_helpers,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _luau_codegen_fix_buffer_len_check =
    ScopedFastFlag::new(&FFlag::LuauCodegenFixBufferLenCheck, true);

  if unsafe { !CODEGEN } || luau_codegen_supported() == 0 {
    return;
  }

  for debug_luau_aborting_checks in [true, false] {
    for optimization_level in 0..=2 {
      let _debug_luau_aborting_checks =
        ScopedFastFlag::new(&FFlag::DebugLuauAbortingChecks, debug_luau_aborting_checks);

      let mut copts = LuaCompileOptions {
        optimization_level,
        debug_level: 1,
        type_info_level: 1,
        coverage_level: 0,
        vector_lib: null(),
        vector_ctor: null(),
        vector_type: null(),
        mutable_globals: null(),
        userdata_types: null(),
        libraries_with_known_members: null(),
        library_member_type_cb: None,
        library_member_constant_cb: None,
        disabled_builtins: null(),
      };

      unsafe {
        run_conformance(
          c"native.luau".as_ptr(),
          Some(setup_native_helpers),
          None,
          null_mut(),
          &mut copts,
          false,
          null_mut(),
        )
      };
    }
  }
}

#[cfg(test)]
#[test]
fn conformance_native_attribute() {
  use core::ffi::{c_char, c_void};

  use ulua_code_gen::{
    enums::{code_gen_compilation_result::CodeGenCompilationResult, code_gen_flags::CodeGenFlags},
    functions::{
      compile_internal::compile_internal, luau_codegen_create::luau_codegen_create,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{compilation_options::CompilationOptions, compilation_stats::CompilationStats},
  };
  use ulua_compiler::functions::luau_compile::luau_compile;
  use ulua_vm::functions::{
    lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
    lua_l_sandboxthread::lua_l_sandboxthread, luau_load::luau_load,
  };

  use crate::common::{functions::run_conformance::CODEGEN, type_aliases::state_ref::StateRef};

  unsafe extern "C" {
    fn free(ptr: *mut c_void);
  }

  if unsafe { !CODEGEN } || luau_codegen_supported() == 0 {
    return;
  }

  let source = r#"
        @native
        local function sum(x, y)
            local function sumHelper(z)
                return (x+y+z)
            end
            return sumHelper
        end

        local function sub(x, y)
            @native
            local function subHelper(z)
                return (x+y-z)
            end
            return subHelper
        end"#;

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    luau_codegen_create(l);

    lua_l_openlibs(l);
    lua_l_sandbox(l);
    lua_l_sandboxthread(l);

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      null_mut(),
      &mut bytecode_size,
    );
    assert!(!bytecode.is_null());

    let result = luau_load(l, c"=Code".as_ptr(), bytecode, bytecode_size, 0);
    free(bytecode as *mut c_void);

    assert_eq!(0, result);

    let native_options = CompilationOptions {
      flags: CodeGenFlags::CodeGenColdFunctions as u32,
      ..Default::default()
    };
    let mut native_stats = CompilationStats::default();
    let native_result = compile_internal(&None, l, -1, &native_options, &mut native_stats);

    assert_eq!(CodeGenCompilationResult::Success, native_result.result);
    assert!(!native_result.has_errors());
    assert!(native_result.proto_failures.is_empty());

    assert_eq!(2, native_stats.functions_compiled);
  }
}

#[cfg(test)]
#[test]
fn conformance_native_integer_spills() {
  use ulua_common::FFlag;
  use ulua_compiler::records::lua_compile_options::LuaCompileOptions;

  use crate::common::{
    functions::run_conformance::run_conformance, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _integer_type = ScopedFastFlag::new(&FFlag::LuauIntegerType2, true);
  let _codegen_dse = ScopedFastFlag::new(&FFlag::LuauCodegenDseRestoreHintUpdate, true);

  for optimization_level in 0..=2 {
    let mut copts = LuaCompileOptions {
      optimization_level,
      debug_level: 1,
      type_info_level: 1,
      coverage_level: 0,
      vector_lib: null(),
      vector_ctor: null(),
      vector_type: null(),
      mutable_globals: null(),
      userdata_types: null(),
      libraries_with_known_members: null(),
      library_member_type_cb: None,
      library_member_constant_cb: None,
      disabled_builtins: null(),
    };

    unsafe {
      run_conformance(
        c"native_integer_spills.luau".as_ptr(),
        None,
        None,
        null_mut(),
        &mut copts,
        false,
        null_mut(),
      )
    };
  }
}

#[cfg(test)]
#[test]
fn conformance_native_type_annotations() {
  use ulua_code_gen::functions::luau_codegen_supported::luau_codegen_supported;

  use crate::common::functions::{
    conformance_native_type_annotations_setup::conformance_native_type_annotations_setup,
    run_conformance::{CODEGEN, run_conformance},
  };

  if unsafe { !CODEGEN } || luau_codegen_supported() == 0 {
    return;
  }

  unsafe {
    run_conformance(
      c"native_types.luau".as_ptr(),
      Some(conformance_native_type_annotations_setup),
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_native_userdata() {
  use ulua_compiler::records::lua_compile_options::LuaCompileOptions;

  use crate::common::{
    functions::{
      conformance_native_userdata_setup::conformance_native_userdata_setup,
      default_codegen_options::default_codegen_options, run_conformance::run_conformance,
    },
    methods::lowering_fixture_initialize_codegen::{
      userdata_access_bytecode_type_callback, userdata_access_callback,
      userdata_metamethod_bytecode_type_callback, userdata_metamethod_callback,
      userdata_namecall_bytecode_type_callback, userdata_namecall_callback,
      vector_access_bytecode_type_callback, vector_access_callback,
      vector_namecall_bytecode_type_callback, vector_namecall_callback,
    },
  };

  let userdata_compile_types = [
    c"vec2".as_ptr(),
    c"color".as_ptr(),
    c"mat3".as_ptr(),
    c"vertex".as_ptr(),
    null(),
  ];
  let userdata_run_types = [
    c"extra".as_ptr(),
    c"color".as_ptr(),
    c"vec2".as_ptr(),
    c"mat3".as_ptr(),
    c"vertex".as_ptr(),
    null(),
  ];

  for use_ir_hooks in [false, true] {
    for optimization_level in 0..=2 {
      let mut copts = LuaCompileOptions {
        optimization_level,
        debug_level: 1,
        type_info_level: 1,
        coverage_level: 0,
        vector_lib: null(),
        vector_ctor: null(),
        vector_type: null(),
        mutable_globals: null(),
        userdata_types: userdata_compile_types.as_ptr(),
        libraries_with_known_members: null(),
        library_member_type_cb: None,
        library_member_constant_cb: None,
        disabled_builtins: null(),
      };
      let mut native_options = default_codegen_options();

      if use_ir_hooks {
        native_options.hooks.vector_access_bytecode_type =
          Some(vector_access_bytecode_type_callback);
        native_options.hooks.vector_namecall_bytecode_type =
          Some(vector_namecall_bytecode_type_callback);
        native_options.hooks.vector_access = Some(vector_access_callback);
        native_options.hooks.vector_namecall = Some(vector_namecall_callback);

        native_options.hooks.userdata_access_bytecode_type =
          Some(userdata_access_bytecode_type_callback);
        native_options.hooks.userdata_metamethod_bytecode_type =
          Some(userdata_metamethod_bytecode_type_callback);
        native_options.hooks.userdata_namecall_bytecode_type =
          Some(userdata_namecall_bytecode_type_callback);
        native_options.hooks.userdata_access = Some(userdata_access_callback);
        native_options.hooks.userdata_metamethod = Some(userdata_metamethod_callback);
        native_options.hooks.userdata_namecall = Some(userdata_namecall_callback);

        native_options.userdata_types = userdata_run_types.as_ptr();
      }

      unsafe {
        run_conformance(
          c"native_userdata.luau".as_ptr(),
          Some(conformance_native_userdata_setup),
          None,
          null_mut(),
          &mut copts,
          false,
          &mut native_options,
        )
      };
    }
  }
}

#[cfg(test)]
#[test]
fn conformance_new_userdata_overflow() {
  use std::ffi::CStr;

  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{lua_l_newstate::lua_l_newstate, lua_pcall::lua_pcall},
    macros::{lua_pushcfunction::LUA_PUSHCFUNCTION, lua_tostring::lua_tostring},
  };

  use crate::common::{
    functions::conformance_new_userdata_overflow_callback::conformance_new_userdata_overflow_callback,
    type_aliases::state_ref::StateRef,
  };

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    LUA_PUSHCFUNCTION(l, Some(conformance_new_userdata_overflow_callback), null());

    assert_eq!(lua_pcall(l, 0, 0, 0), LuaStatus::ErrRun as i32);
    assert_eq!(
      CStr::from_ptr(lua_tostring!(l, -1)),
      c"memory allocation error: block too big"
    );
  }
}

#[cfg(test)]
#[test]
fn conformance_p_call() {
  use ulua_common::FFlag;
  use ulua_vm::functions::lua_newstate::lua_newstate;

  use crate::common::{
    functions::{
      conformance_p_call_setup::conformance_p_call_setup, limited_realloc::limited_realloc,
      run_conformance::run_conformance,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _luau_resume_restore_c_calls = ScopedFastFlag::new(&FFlag::LuauResumeRestoreCcalls, true);
  let initial_lua_state = unsafe { lua_newstate(Some(limited_realloc), null_mut()) };

  unsafe {
    run_conformance(
      c"pcall.luau".as_ptr(),
      Some(conformance_p_call_setup),
      None,
      initial_lua_state,
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_pack() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"tpack.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_pattern_match() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"pm.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_reference() {
  use std::sync::atomic::Ordering;

  use ulua_vm::{
    enums::lua_gc_op::LuaGcOp,
    functions::{
      lua_gc::lua_gc, lua_isuserdata::lua_isuserdata, lua_l_newstate::lua_l_newstate,
      lua_newuserdatadtor::lua_newuserdatadtor, lua_ref::lua_ref, lua_unref::lua_unref,
    },
    macros::{lua_getref::lua_getref, lua_pop::lua_pop},
  };

  use crate::common::{
    functions::{
      conformance_reference_dtor::conformance_reference_dtor,
      conformance_reference_dtor_hits::CONFORMANCE_REFERENCE_DTOR_HITS,
    },
    type_aliases::state_ref::StateRef,
  };

  CONFORMANCE_REFERENCE_DTOR_HITS.store(0, Ordering::SeqCst);

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    lua_newuserdatadtor(l, 0, Some(conformance_reference_dtor));
    lua_newuserdatadtor(l, 0, Some(conformance_reference_dtor));

    lua_gc(l, LuaGcOp::Collect as i32, 0);
    assert_eq!(CONFORMANCE_REFERENCE_DTOR_HITS.load(Ordering::SeqCst), 0);

    let reference = lua_ref(l, -2);
    lua_pop(l, 2);

    lua_gc(l, LuaGcOp::Collect as i32, 0);
    assert_eq!(CONFORMANCE_REFERENCE_DTOR_HITS.load(Ordering::SeqCst), 1);

    lua_getref(l, reference);
    assert_ne!(lua_isuserdata(l, -1), 0);
    lua_pop(l, 1);

    lua_gc(l, LuaGcOp::Collect as i32, 0);
    assert_eq!(CONFORMANCE_REFERENCE_DTOR_HITS.load(Ordering::SeqCst), 1);

    lua_unref(l, reference);

    lua_gc(l, LuaGcOp::Collect as i32, 0);
    assert_eq!(CONFORMANCE_REFERENCE_DTOR_HITS.load(Ordering::SeqCst), 2);
  }
}

#[cfg(test)]
#[test]
fn conformance_safe_env() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"safeenv.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_same_hash() {
  use ulua_bytecode::records::{bytecode_builder::BytecodeBuilder, string_ref::StringRef};
  use ulua_vm::functions::lua_s_hash::lua_s_hash;

  fn string_ref(s: &'static [u8]) -> StringRef {
    StringRef::from_slice(s)
  }

  unsafe {
    assert_eq!(
      lua_s_hash(c"".as_ptr(), 0),
      BytecodeBuilder::get_string_hash(string_ref(b""))
    );
    assert_eq!(
      lua_s_hash(c"lua".as_ptr(), 3),
      BytecodeBuilder::get_string_hash(string_ref(b"lua"))
    );
    assert_eq!(
      lua_s_hash(c"luau".as_ptr(), 4),
      BytecodeBuilder::get_string_hash(string_ref(b"luau"))
    );
    assert_eq!(
      lua_s_hash(c"luaubytecode".as_ptr(), 12),
      BytecodeBuilder::get_string_hash(string_ref(b"luaubytecode"))
    );
    assert_eq!(
      lua_s_hash(c"luaubytecodehash".as_ptr(), 16),
      BytecodeBuilder::get_string_hash(string_ref(b"luaubytecodehash"))
    );
  }

  let buf = [0u8; 128];
  unsafe {
    assert_eq!(
      lua_s_hash(buf.as_ptr().add(1).cast(), 120),
      lua_s_hash(buf.as_ptr().add(2).cast(), 120)
    );
  }
}

#[cfg(test)]
#[test]
fn conformance_sandbox_without_libs() {
  use ulua_vm::{
    functions::{
      lua_getreadonly::lua_getreadonly, lua_l_newstate::lua_l_newstate,
      lua_l_sandbox::lua_l_sandbox, luaopen_base::luaopen_base,
    },
    macros::lua_globalsindex::LUA_GLOBALSINDEX,
  };

  use crate::common::type_aliases::state_ref::StateRef;

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    luaopen_base(l);
    lua_l_sandbox(l);

    assert_ne!(lua_getreadonly(l, LUA_GLOBALSINDEX), 0);
  }
}

#[cfg(test)]
#[test]
fn conformance_sort() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"sort.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_string_conversion() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"strconv.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_string_interp() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"stringinterp.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_strings() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"strings.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_tables() {
  use crate::common::functions::{
    conformance_tables_setup::conformance_tables_setup, run_conformance::run_conformance,
  };

  unsafe {
    run_conformance(
      c"tables.luau".as_ptr(),
      Some(conformance_tables_setup),
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_tag_method_error() {
  use core::sync::atomic::Ordering;

  use crate::common::{
    functions::{
      conformance_tag_method_error_setup::conformance_tag_method_error_setup,
      conformance_tag_method_error_yield::conformance_tag_method_error_yield,
      run_conformance::run_conformance,
    },
    records::conformance_tag_method_error_state::CONFORMANCE_TAG_METHOD_ERROR_STATE,
  };

  for lua_break in [false, true] {
    CONFORMANCE_TAG_METHOD_ERROR_STATE.reset(lua_break);

    unsafe {
      run_conformance(
        c"tmerror.luau".as_ptr(),
        Some(conformance_tag_method_error_setup),
        Some(conformance_tag_method_error_yield),
        null_mut(),
        null_mut(),
        false,
        null_mut(),
      )
    };
    assert_eq!(
      CONFORMANCE_TAG_METHOD_ERROR_STATE
        .index
        .load(Ordering::SeqCst),
      3
    );
  }
}

#[cfg(test)]
#[test]
fn conformance_types() {
  use crate::common::functions::{
    conformance_types_setup::conformance_types_setup, run_conformance::run_conformance,
  };

  unsafe {
    run_conformance(
      c"types.luau".as_ptr(),
      Some(conformance_types_setup),
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_userdata() {
  use crate::common::functions::{
    conformance_userdata_setup::conformance_userdata_setup, run_conformance::run_conformance,
  };

  unsafe {
    run_conformance(
      c"userdata.luau".as_ptr(),
      Some(conformance_userdata_setup),
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_userdata_alignment() {
  use ulua_vm::{
    functions::{lua_newstate::lua_newstate, lua_newuserdatadtor::lua_newuserdatadtor},
    macros::{lua_newuserdata::lua_newuserdata, lua_pop::lua_pop},
  };

  use crate::common::{
    functions::{
      userdata_alignment_alloc::userdata_alignment_alloc,
      userdata_alignment_dtor::userdata_alignment_dtor,
    },
    type_aliases::state_ref::StateRef,
  };

  let global_state =
    StateRef::new(unsafe { lua_newstate(Some(userdata_alignment_alloc), null_mut()) })
      .expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    for size in (16..=4096).step_by(4) {
      for _ in 0..10 {
        let data = lua_newuserdata(l, size);
        assert_eq!((data as usize) % 16, 0);
        lua_pop(l, 1);
      }

      for _ in 0..10 {
        let data = lua_newuserdatadtor(l, size, Some(userdata_alignment_dtor));
        assert_eq!((data as usize) % 16, 0);
        lua_pop(l, 1);
      }
    }
  }
}

#[cfg(test)]
#[test]
fn conformance_userdata_api() {
  use core::{ffi::c_void, sync::atomic::Ordering};

  use ulua_vm::{
    functions::{
      lua_getuserdatadtor::lua_getuserdatadtor, lua_getuserdatametatable::lua_getuserdatametatable,
      lua_l_checkudata::lua_l_checkudata, lua_l_newmetatable::lua_l_newmetatable,
      lua_l_newstate::lua_l_newstate, lua_newuserdatadtor::lua_newuserdatadtor,
      lua_newuserdatatagged::lua_newuserdatatagged,
      lua_newuserdatataggedwithmetatable::lua_newuserdatataggedwithmetatable,
      lua_pushlightuserdatatagged::lua_pushlightuserdatatagged, lua_pushvalue::lua_pushvalue,
      lua_setmetatable::lua_setmetatable, lua_setuserdatadtor::lua_setuserdatadtor,
      lua_setuserdatametatable::lua_setuserdatametatable, lua_setuserdatatag::lua_setuserdatatag,
      lua_tolightuserdata::lua_tolightuserdata, lua_topointer::lua_topointer,
      lua_touserdata::lua_touserdata, lua_touserdatatagged::lua_touserdatatagged,
      lua_userdatatag::lua_userdatatag,
    },
    macros::lua_l_getmetatable::lua_l_getmetatable,
  };

  use crate::common::{
    functions::{
      userdata_api_dtor_hits::USERDATA_API_DTOR_HITS,
      userdata_api_inline_char_dtor::userdata_api_inline_char_dtor,
      userdata_api_inline_int_dtor::userdata_api_inline_int_dtor,
      userdata_api_tag_dtor::userdata_api_tag_dtor,
    },
    type_aliases::state_ref::StateRef,
  };

  USERDATA_API_DTOR_HITS.store(0, Ordering::SeqCst);

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    let dtor_is_null = lua_getuserdatadtor(l, 42).is_none();
    assert!(dtor_is_null);
    lua_setuserdatadtor(l, 42, Some(userdata_api_tag_dtor));
    let dtor_is_set = lua_getuserdatadtor(l, 42).map(|dtor| dtor as *const ())
      == Some(userdata_api_tag_dtor as *const ());
    assert!(dtor_is_set);

    let mut lud = 0i32;
    let lud_ptr = (&mut lud as *mut i32).cast::<c_void>();
    lua_pushlightuserdatatagged(l, lud_ptr, 0);

    assert_eq!(lua_tolightuserdata(l, -1), lud_ptr);
    assert_eq!(lua_touserdata(l, -1), lud_ptr);
    assert_eq!(lua_topointer(l, -1), lud_ptr.cast_const());

    let ud1 = lua_newuserdatatagged(l, 4, 0) as *mut i32;
    *ud1 = 42;

    assert!(lua_tolightuserdata(l, -1).is_null());
    assert_eq!(lua_touserdata(l, -1), ud1.cast::<c_void>());
    assert_eq!(lua_topointer(l, -1), ud1.cast::<c_void>().cast_const());

    let ud2 = lua_newuserdatatagged(l, 4, 42) as *mut i32;
    *ud2 = -4;

    assert_eq!(lua_touserdatatagged(l, -1, 42), ud2.cast::<c_void>());
    assert!(lua_touserdatatagged(l, -1, 41).is_null());
    assert_eq!(lua_userdatatag(l, -1), 42);

    lua_setuserdatatag(l, -1, 43);
    assert_eq!(lua_userdatatag(l, -1), 43);
    lua_setuserdatatag(l, -1, 42);

    let ud3 = lua_newuserdatadtor(l, 4, Some(userdata_api_inline_int_dtor)) as *mut i32;
    let ud4 = lua_newuserdatadtor(l, 1, Some(userdata_api_inline_char_dtor)) as *mut i8;

    *ud3 = 43;
    *ud4 = 3;

    lua_l_newmetatable(l, c"udata1".as_ptr());
    lua_l_newmetatable(l, c"udata2".as_ptr());

    let ud5 = lua_newuserdatatagged(l, 0, 0);
    lua_l_getmetatable(l, c"udata1".as_ptr());
    lua_setmetatable(l, -2);

    let ud6 = lua_newuserdatatagged(l, 0, 0);
    lua_l_getmetatable(l, c"udata2".as_ptr());
    lua_setmetatable(l, -2);

    assert_eq!(lua_l_checkudata(l, -2, "udata1"), ud5);
    assert_eq!(lua_l_checkudata(l, -1, "udata2"), ud6);

    lua_l_newmetatable(l, c"udata3".as_ptr());
    lua_pushvalue(l, -1);
    lua_setuserdatametatable(l, 50);

    lua_l_newmetatable(l, c"udata4".as_ptr());
    lua_pushvalue(l, -1);
    lua_setuserdatametatable(l, 51);

    let ud7 = lua_newuserdatatagged(l, 16, 50);
    lua_getuserdatametatable(l, 50);
    lua_setmetatable(l, -2);

    let ud8 = lua_newuserdatataggedwithmetatable(l, 16, 51);

    assert_eq!(lua_l_checkudata(l, -2, "udata3"), ud7);
    assert_eq!(lua_l_checkudata(l, -1, "udata4"), ud8);
  }

  drop(global_state);

  assert_eq!(USERDATA_API_DTOR_HITS.load(Ordering::SeqCst), 42);
}

#[cfg(test)]
#[test]
fn conformance_userdata_direct_access() {
  use ulua_common::FFlag;

  use crate::common::{
    functions::{
      conformance_userdata_direct_access_setup::conformance_userdata_direct_access_setup,
      get_or_create_atom::reset_direct_atom_state, run_conformance::run_conformance,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _luau_udata_direct_access = ScopedFastFlag::new(&FFlag::LuauUdataDirectAccess6, true);

  reset_direct_atom_state();

  unsafe {
    run_conformance(
      c"udata_direct.luau".as_ptr(),
      Some(conformance_userdata_direct_access_setup),
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_utf_8() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"utf8.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_var_arg() {
  use crate::common::functions::run_conformance::run_conformance;

  unsafe {
    run_conformance(
      c"vararg.luau".as_ptr(),
      None,
      None,
      null_mut(),
      null_mut(),
      false,
      null_mut(),
    )
  };
}

#[cfg(test)]
#[test]
fn conformance_vector() {
  use ulua_compiler::records::lua_compile_options::LuaCompileOptions;

  use crate::common::{
    functions::{
      conformance_vector_setup::conformance_vector_setup,
      default_codegen_options::default_codegen_options, run_conformance::run_conformance,
    },
    methods::lowering_fixture_initialize_codegen::{
      vector_access_bytecode_type_callback, vector_access_callback,
      vector_namecall_bytecode_type_callback, vector_namecall_callback,
    },
  };

  for use_ir_hooks in [false, true] {
    for optimization_level in 0..=2 {
      let mut copts = LuaCompileOptions {
        optimization_level,
        debug_level: 1,
        type_info_level: 1,
        coverage_level: 0,
        vector_lib: null(),
        vector_ctor: null(),
        vector_type: null(),
        mutable_globals: null(),
        userdata_types: null(),
        libraries_with_known_members: null(),
        library_member_type_cb: None,
        library_member_constant_cb: None,
        disabled_builtins: null(),
      };
      let mut native_options = default_codegen_options();

      if use_ir_hooks {
        native_options.hooks.vector_access_bytecode_type =
          Some(vector_access_bytecode_type_callback);
        native_options.hooks.vector_namecall_bytecode_type =
          Some(vector_namecall_bytecode_type_callback);
        native_options.hooks.vector_access = Some(vector_access_callback);
        native_options.hooks.vector_namecall = Some(vector_namecall_callback);
      }

      unsafe {
        run_conformance(
          c"vector.luau".as_ptr(),
          Some(conformance_vector_setup),
          None,
          null_mut(),
          &mut copts,
          false,
          &mut native_options,
        )
      };
    }
  }
}

#[cfg(test)]
#[test]
fn conformance_vector_library() {
  use ulua_compiler::records::lua_compile_options::LuaCompileOptions;

  use crate::common::functions::{
    run_conformance::run_conformance, setup_native_helpers::setup_native_helpers,
  };

  for optimization_level in 0..=2 {
    let mut copts = LuaCompileOptions {
      optimization_level,
      debug_level: 1,
      type_info_level: 1,
      coverage_level: 0,
      vector_lib: null(),
      vector_ctor: null(),
      vector_type: null(),
      mutable_globals: null(),
      userdata_types: null(),
      libraries_with_known_members: null(),
      library_member_type_cb: None,
      library_member_constant_cb: None,
      disabled_builtins: null(),
    };

    unsafe {
      run_conformance(
        c"vector_library.luau".as_ptr(),
        Some(setup_native_helpers),
        None,
        null_mut(),
        &mut copts,
        false,
        null_mut(),
      )
    };
  }
}

#[cfg(test)]
#[test]
fn direct_field_access_handler_setboolean_result() {
  use ulua_common::FFlag;
  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_l_newstate::lua_l_newstate,
      lua_registeruserdatadirectfieldget::lua_registeruserdatadirectfieldget,
      lua_toboolean::lua_toboolean,
    },
    macros::{
      lua_isboolean::lua_isboolean, lua_pushcfunction::LUA_PUSHCFUNCTION,
      lua_setglobal::lua_setglobal,
    },
  };

  use crate::common::{
    functions::{
      direct_field_access_create_vec_2::direct_field_access_create_vec_2,
      direct_field_access_get_non_zero_boolean::direct_field_access_get_non_zero_boolean,
      direct_field_access_k_tag_vec_2::K_TAG_VEC2, run_code::run_code,
    },
    type_aliases::{scoped_fast_flag::ScopedFastFlag, state_ref::StateRef},
  };

  let _sff = ScopedFastFlag::new(&FFlag::LuauDirectFieldGet, true);
  let state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = state.as_ptr();

  unsafe {
    lua_registeruserdatadirectfieldget(
      l,
      K_TAG_VEC2,
      c"NonZero".as_ptr(),
      Some(direct_field_access_get_non_zero_boolean),
    );

    LUA_PUSHCFUNCTION(
      l,
      Some(direct_field_access_create_vec_2),
      c"createVec2".as_ptr(),
    );
    lua_setglobal(l, c"createVec2".as_ptr());

    {
      let status = run_code(
        l,
        r#"
            local v = createVec2(1, 0)
            return v.NonZero
        "#,
      );
      assert_eq!(status, LuaStatus::Ok as i32);
      assert!(lua_isboolean!(l, -1));
      assert_eq!(lua_toboolean(l, -1), 1);
    }
    {
      let status = run_code(
        l,
        r#"
            local v = createVec2(0, 0)
            return v.NonZero
        "#,
      );
      assert_eq!(status, LuaStatus::Ok as i32);
      assert!(lua_isboolean!(l, -1));
      assert_eq!(lua_toboolean(l, -1), 0);
    }
  }
}

#[cfg(test)]
#[test]
fn direct_field_access_handler_setnumber_result() {
  use ulua_common::FFlag;
  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_isnumber::lua_isnumber, lua_l_newstate::lua_l_newstate,
      lua_registeruserdatadirectfieldget::lua_registeruserdatadirectfieldget,
    },
    macros::{
      lua_pushcfunction::LUA_PUSHCFUNCTION, lua_setglobal::lua_setglobal,
      lua_tonumber::lua_tonumber,
    },
  };

  use crate::common::{
    functions::{
      direct_field_access_create_vec_2::direct_field_access_create_vec_2,
      direct_field_access_get_x_number::direct_field_access_get_x_number,
      direct_field_access_k_tag_vec_2::K_TAG_VEC2, run_code::run_code,
    },
    type_aliases::{scoped_fast_flag::ScopedFastFlag, state_ref::StateRef},
  };

  let _sff = ScopedFastFlag::new(&FFlag::LuauDirectFieldGet, true);
  let state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = state.as_ptr();

  unsafe {
    lua_registeruserdatadirectfieldget(
      l,
      K_TAG_VEC2,
      c"X".as_ptr(),
      Some(direct_field_access_get_x_number),
    );

    LUA_PUSHCFUNCTION(
      l,
      Some(direct_field_access_create_vec_2),
      c"createVec2".as_ptr(),
    );
    lua_setglobal(l, c"createVec2".as_ptr());

    let status = run_code(
      l,
      r#"
        local v = createVec2(3.5, 0)
        return v.X
    "#,
    );
    assert_eq!(status, LuaStatus::Ok as i32);

    assert_ne!(lua_isnumber(l, -1), 0);
    assert_eq!(lua_tonumber!(l, -1), 3.5);
  }
}

#[cfg(test)]
#[test]
fn direct_field_access_multiple_fields_same_type_dispatch_independently() {
  use ulua_common::FFlag;
  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_gettop::lua_gettop, lua_l_newstate::lua_l_newstate,
      lua_registeruserdatadirectfieldget::lua_registeruserdatadirectfieldget,
    },
    macros::{
      lua_pushcfunction::LUA_PUSHCFUNCTION, lua_setglobal::lua_setglobal,
      lua_tonumber::lua_tonumber,
    },
  };

  use crate::common::{
    functions::{
      direct_field_access_create_vec_2::direct_field_access_create_vec_2,
      direct_field_access_get_x_number::direct_field_access_get_x_number,
      direct_field_access_get_y_number::direct_field_access_get_y_number,
      direct_field_access_k_tag_vec_2::K_TAG_VEC2, run_code::run_code,
    },
    type_aliases::{scoped_fast_flag::ScopedFastFlag, state_ref::StateRef},
  };

  let _sff = ScopedFastFlag::new(&FFlag::LuauDirectFieldGet, true);
  let state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = state.as_ptr();

  unsafe {
    lua_registeruserdatadirectfieldget(
      l,
      K_TAG_VEC2,
      c"X".as_ptr(),
      Some(direct_field_access_get_x_number),
    );
    lua_registeruserdatadirectfieldget(
      l,
      K_TAG_VEC2,
      c"Y".as_ptr(),
      Some(direct_field_access_get_y_number),
    );

    LUA_PUSHCFUNCTION(
      l,
      Some(direct_field_access_create_vec_2),
      c"createVec2".as_ptr(),
    );
    lua_setglobal(l, c"createVec2".as_ptr());

    let status = run_code(
      l,
      r#"
        local v = createVec2(1.5, 2.5)
        return v.X, v.Y
    "#,
    );
    assert_eq!(status, LuaStatus::Ok as i32);
    assert_eq!(lua_gettop(l), 2);

    assert_eq!(lua_tonumber!(l, -2), 1.5);
    assert_eq!(lua_tonumber!(l, -1), 2.5);
  }
}

#[cfg(test)]
#[test]
fn direct_field_access_repeated_access_handler_called_every_iteration() {
  use ulua_common::FFlag;
  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_isnumber::lua_isnumber, lua_l_newstate::lua_l_newstate,
      lua_registeruserdatadirectfieldget::lua_registeruserdatadirectfieldget,
    },
    macros::{
      lua_pushcfunction::LUA_PUSHCFUNCTION, lua_setglobal::lua_setglobal,
      lua_tonumber::lua_tonumber,
    },
  };

  use crate::common::{
    functions::{
      direct_field_access_counted_get_x_number::direct_field_access_counted_get_x_number,
      direct_field_access_create_vec_2::direct_field_access_create_vec_2,
      direct_field_access_handler_hit_count::direct_field_access_handler_hit_count,
      direct_field_access_k_tag_vec_2::K_TAG_VEC2,
      direct_field_access_reset_handler_hit_count::direct_field_access_reset_handler_hit_count,
      run_code::run_code,
    },
    type_aliases::{scoped_fast_flag::ScopedFastFlag, state_ref::StateRef},
  };

  let _lock = DIRECT_FIELD_ACCESS_TEST_MUTEX.lock().unwrap();
  let _sff = ScopedFastFlag::new(&FFlag::LuauDirectFieldGet, true);
  let state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = state.as_ptr();

  unsafe {
    direct_field_access_reset_handler_hit_count();
    lua_registeruserdatadirectfieldget(
      l,
      K_TAG_VEC2,
      c"X".as_ptr(),
      Some(direct_field_access_counted_get_x_number),
    );

    LUA_PUSHCFUNCTION(
      l,
      Some(direct_field_access_create_vec_2),
      c"createVec2".as_ptr(),
    );
    lua_setglobal(l, c"createVec2".as_ptr());

    let status = run_code(
      l,
      r#"
        local v = createVec2(7, 0)
        local sum = 0
        for i = 1, 5 do
            sum = sum + v.X
        end
        return sum
    "#,
    );
    assert_eq!(status, LuaStatus::Ok as i32);
    assert_ne!(lua_isnumber(l, -1), 0);
    assert_eq!(lua_tonumber!(l, -1), 35.0);

    assert_eq!(direct_field_access_handler_hit_count(), 5);
  }
}

#[cfg(test)]
#[test]
fn direct_field_access_same_field_name_different_tags_dispatch_independently() {
  use ulua_common::FFlag;
  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_gettop::lua_gettop, lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs,
      lua_registeruserdatadirectfieldget::lua_registeruserdatadirectfieldget,
    },
    macros::{
      lua_pushcfunction::LUA_PUSHCFUNCTION, lua_setglobal::lua_setglobal,
      lua_tonumber::lua_tonumber,
    },
  };

  use crate::common::{
    functions::{
      direct_field_access_counted_get_999_number::direct_field_access_counted_get_999_number,
      direct_field_access_counted_get_x_number::direct_field_access_counted_get_x_number,
      direct_field_access_create_other_without_mt::direct_field_access_create_other_without_mt,
      direct_field_access_create_vec_2::direct_field_access_create_vec_2,
      direct_field_access_handler_hit_count::direct_field_access_handler_hit_count,
      direct_field_access_k_tag_other::K_TAG_OTHER, direct_field_access_k_tag_vec_2::K_TAG_VEC2,
      direct_field_access_reset_handler_hit_count::direct_field_access_reset_handler_hit_count,
      run_code::run_code,
    },
    type_aliases::{scoped_fast_flag::ScopedFastFlag, state_ref::StateRef},
  };

  let _lock = DIRECT_FIELD_ACCESS_TEST_MUTEX.lock().unwrap();
  let _sff = ScopedFastFlag::new(&FFlag::LuauDirectFieldGet, true);
  let state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = state.as_ptr();

  unsafe {
    lua_l_openlibs(l);

    direct_field_access_reset_handler_hit_count();
    lua_registeruserdatadirectfieldget(
      l,
      K_TAG_VEC2,
      c"X".as_ptr(),
      Some(direct_field_access_counted_get_x_number),
    );
    lua_registeruserdatadirectfieldget(
      l,
      K_TAG_OTHER,
      c"X".as_ptr(),
      Some(direct_field_access_counted_get_999_number),
    );

    LUA_PUSHCFUNCTION(
      l,
      Some(direct_field_access_create_vec_2),
      c"createVec2".as_ptr(),
    );
    lua_setglobal(l, c"createVec2".as_ptr());

    LUA_PUSHCFUNCTION(
      l,
      Some(direct_field_access_create_other_without_mt),
      c"createOther".as_ptr(),
    );
    lua_setglobal(l, c"createOther".as_ptr());

    let status = run_code(
      l,
      r#"
        return createVec2(3, 0).X, createOther().X
    "#,
    );
    assert_eq!(status, LuaStatus::Ok as i32);
    assert_eq!(lua_gettop(l), 2);

    assert_eq!(lua_tonumber!(l, -2), 3.0);
    assert_eq!(lua_tonumber!(l, -1), 999.0);
    assert_eq!(direct_field_access_handler_hit_count(), 2);
  }
}

#[cfg(test)]
#[test]
fn direct_field_access_unregistered_tag_falls_through_to_index_metamethod() {
  use ulua_common::FFlag;
  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_gettop::lua_gettop, lua_l_newmetatable::lua_l_newmetatable,
      lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs,
      lua_registeruserdatadirectfieldget::lua_registeruserdatadirectfieldget,
      lua_setfield::lua_setfield, lua_setuserdatametatable::lua_setuserdatametatable,
    },
    macros::{
      lua_pushcfunction::LUA_PUSHCFUNCTION, lua_setglobal::lua_setglobal,
      lua_tonumber::lua_tonumber,
    },
  };

  use crate::common::{
    functions::{
      direct_field_access_counted_get_x_number::direct_field_access_counted_get_x_number,
      direct_field_access_create_other_with_mt::direct_field_access_create_other_with_mt,
      direct_field_access_create_vec_2::direct_field_access_create_vec_2,
      direct_field_access_handler_hit_count::direct_field_access_handler_hit_count,
      direct_field_access_k_tag_other::K_TAG_OTHER, direct_field_access_k_tag_vec_2::K_TAG_VEC2,
      direct_field_access_push_minus_one::direct_field_access_push_minus_one,
      direct_field_access_reset_handler_hit_count::direct_field_access_reset_handler_hit_count,
      run_code::run_code,
    },
    type_aliases::{scoped_fast_flag::ScopedFastFlag, state_ref::StateRef},
  };

  let _lock = DIRECT_FIELD_ACCESS_TEST_MUTEX.lock().unwrap();
  let _sff = ScopedFastFlag::new(&FFlag::LuauDirectFieldGet, true);
  let state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = state.as_ptr();

  unsafe {
    lua_l_openlibs(l);

    direct_field_access_reset_handler_hit_count();
    lua_registeruserdatadirectfieldget(
      l,
      K_TAG_VEC2,
      c"X".as_ptr(),
      Some(direct_field_access_counted_get_x_number),
    );

    lua_l_newmetatable(l, c"metaOther".as_ptr());
    LUA_PUSHCFUNCTION(
      l,
      Some(direct_field_access_push_minus_one),
      c"__index".as_ptr(),
    );
    lua_setfield(l, -2, c"__index".as_ptr());
    lua_setuserdatametatable(l, K_TAG_OTHER);

    LUA_PUSHCFUNCTION(
      l,
      Some(direct_field_access_create_vec_2),
      c"createVec2".as_ptr(),
    );
    lua_setglobal(l, c"createVec2".as_ptr());
    LUA_PUSHCFUNCTION(
      l,
      Some(direct_field_access_create_other_with_mt),
      c"createOther".as_ptr(),
    );
    lua_setglobal(l, c"createOther".as_ptr());

    let status = run_code(
      l,
      r#"
        local uds = {createVec2(1, 0), createOther()}
        local results = {}
        for _, v in uds do
            results[#results + 1] = v.X
        end
        return table.unpack(results)
    "#,
    );
    assert_eq!(status, LuaStatus::Ok as i32);
    assert_eq!(lua_gettop(l), 2);

    assert_eq!(lua_tonumber!(l, -2), 1.0);
    assert_eq!(lua_tonumber!(l, -1), -1.0);

    assert_eq!(direct_field_access_handler_hit_count(), 1);
  }
}

#[cfg(test)]
#[test]
fn feedback_vector_c_call_sealed() {
  use ulua_common::{FFlag, FInt, macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED};
  use ulua_vm::{functions::luaopen_base::luaopen_base, macros::lua_pop::lua_pop};

  use crate::common::{
    functions::id_inliner::id_inliner,
    records::feedback_vector_fixture::FeedbackVectorFixture,
    type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
  };

  let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);
  let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
  let _inline_threshold = ScopedFastInt::new(&FInt::LuauInlineHitsThreshold, 2);

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(String::from(
    r#"
        local function f(h) return h(1) + 1 end
        f(tostring)
    "#,
  ));

  let top = fixture.load();

  unsafe {
    let f = *(*top).p.add(0);
    let fbslot = (*f).feedbackvec.add(0);

    assert_eq!(
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
      0
    );

    fixture.on_inline = Some(id_inliner);

    let l = fixture.lua_state();
    lua_pop(l, luaopen_base(l));

    fixture.run();

    assert_eq!(
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
      LUAU_INSN_FBSLOT_SEALED
    );
  }
}

#[cfg(test)]
#[test]
fn feedback_vector_high_order_call() {
  use ulua_common::{FFlag, FInt, enums::luau_proto_flag::LuauProtoFlag};
  use ulua_vm::enums::feedback_vector_slot_kind::FeedbackVectorSlotKind;

  use crate::common::{
    functions::id_inliner_with_assert::id_inliner_with_assert,
    records::{
      assert_inliner_data::AssertInlinerData, feedback_vector_fixture::FeedbackVectorFixture,
    },
    type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
  };

  let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);
  let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
  let _inline_threshold = ScopedFastInt::new(&FInt::LuauInlineHitsThreshold, 2);

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(String::from(
    r#"
        local function g() return 1 end
        local function f(h) return h() + 1 end
        f(g)
        f(g)
    "#,
  ));

  assert_eq!(
    format!("\n{}", fixture.bcb.dump_function(1)),
    r#"
MOVE R2 R0
CALLFB R2 0 1 [0]
LOADK R3 K0 [1]
ADD R1 R2 R3
RETURN R1 1
"#
  );

  let top = fixture.load();

  unsafe {
    let g = *(*top).p.add(0);
    assert_ne!((*g).flags & LuauProtoFlag::LPF_INLINABLE as u8, 0);

    let f = *(*top).p.add(1);
    assert_eq!((*f).feedbackvecsize, 1);

    let fbslot = (*f).feedbackvec.add(0);
    assert_eq!((*fbslot).kind, FeedbackVectorSlotKind::CallTarget);
    assert_eq!((*fbslot).data.call_target.pc, 1);
    assert_eq!((*fbslot).data.call_target.proto, 0);
    assert_eq!((*fbslot).data.call_target.hits, 0);
    assert_eq!(
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
      0
    );

    let data = (*(*fixture.lua_state()).global).ecbdata.as_mut_ptr() as *mut AssertInlinerData;
    data.write(AssertInlinerData {
      proto: f,
      target: g,
      pc: (*fbslot).data.call_target.pc,
      called: false,
    });
    fixture.on_inline = Some(id_inliner_with_assert);

    fixture.run();

    assert_eq!((*fbslot).data.call_target.pc, 1);
    assert_eq!((*fbslot).data.call_target.proto, (*g).funid);
    assert_eq!((*fbslot).data.call_target.hits, 2);
    assert!((*data).called);
  }
}

#[cfg(test)]
#[test]
fn feedback_vector_metamethod_call_sealed() {
  use ulua_common::{FFlag, FInt, macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED};
  use ulua_vm::{functions::luaopen_base::luaopen_base, macros::lua_pop::lua_pop};

  use crate::common::{
    functions::id_inliner::id_inliner,
    records::feedback_vector_fixture::FeedbackVectorFixture,
    type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
  };

  let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);
  let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
  let _inline_threshold = ScopedFastInt::new(&FInt::LuauInlineHitsThreshold, 2);

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(String::from(
    r#"
        local function f(h) return h(1) + 1 end

        local callableTable = {}

        setmetatable(callableTable, { __call = function(self, arg) return arg + 42 end })

        f(callableTable)
    "#,
  ));

  let top = fixture.load();

  unsafe {
    let f = *(*top).p.add(0);
    let fbslot = (*f).feedbackvec.add(0);

    assert_eq!(
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
      0
    );

    fixture.on_inline = Some(id_inliner);

    let l = fixture.lua_state();
    lua_pop(l, luaopen_base(l));

    fixture.run();

    assert_eq!(
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
      LUAU_INSN_FBSLOT_SEALED
    );
  }
}

#[cfg(test)]
#[test]
fn feedback_vector_namecall() {
  use ulua_common::{FFlag, FInt, enums::luau_proto_flag::LuauProtoFlag};
  use ulua_vm::enums::feedback_vector_slot_kind::FeedbackVectorSlotKind;

  use crate::common::{
    functions::id_inliner_with_assert::id_inliner_with_assert,
    records::{
      assert_inliner_data::AssertInlinerData, feedback_vector_fixture::FeedbackVectorFixture,
    },
    type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
  };

  let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);
  let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
  let _inline_threshold = ScopedFastInt::new(&FInt::LuauInlineHitsThreshold, 2);

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(String::from(
    r#"
        local t = { x = 1 }
        function t.g(self) return self.x end
        local function f(t) return t:g() + 1 end
        f(t)
        f(t)
    "#,
  ));

  assert_eq!(
    format!("\n{}", fixture.bcb.dump_function(1)),
    r#"
NAMECALL R2 R0 K0 ['g']
CALLFB R2 1 1 [0]
LOADK R3 K1 [1]
ADD R1 R2 R3
RETURN R1 1
"#
  );

  let top = fixture.load();

  unsafe {
    let g = *(*top).p.add(0);
    assert_ne!((*g).flags & LuauProtoFlag::LPF_INLINABLE as u8, 0);

    let f = *(*top).p.add(1);
    assert_eq!((*f).feedbackvecsize, 1);

    let fbslot = (*f).feedbackvec.add(0);
    assert_eq!((*fbslot).kind, FeedbackVectorSlotKind::CallTarget);
    assert_eq!((*fbslot).data.call_target.pc, 2);
    assert_eq!((*fbslot).data.call_target.proto, 0);
    assert_eq!((*fbslot).data.call_target.hits, 0);
    assert_eq!(
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
      0
    );

    let data = (*(*fixture.lua_state()).global).ecbdata.as_mut_ptr() as *mut AssertInlinerData;
    data.write(AssertInlinerData {
      proto: f,
      target: g,
      pc: (*fbslot).data.call_target.pc,
      called: false,
    });
    fixture.on_inline = Some(id_inliner_with_assert);

    fixture.run();

    assert_eq!((*fbslot).data.call_target.proto, (*g).funid);
    assert_eq!((*fbslot).data.call_target.hits, 2);
    assert!((*data).called);
  }
}

#[cfg(test)]
#[test]
fn feedback_vector_polymorphic_call_sealed() {
  use ulua_common::{FFlag, FInt, macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED};

  use crate::common::{
    functions::id_inliner::id_inliner,
    records::feedback_vector_fixture::FeedbackVectorFixture,
    type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
  };

  let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);
  let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
  let _inline_threshold = ScopedFastInt::new(&FInt::LuauInlineHitsThreshold, 2);

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(String::from(
    r#"
        local function g() return 1 end
        local function y() return 2 end
        local function f(h) return h() + 1 end
        f(g)
        f(y)
    "#,
  ));

  let top = fixture.load();

  unsafe {
    let f = *(*top).p.add(2);
    let fbslot = (*f).feedbackvec.add(0);

    assert_eq!(
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
      0
    );

    fixture.on_inline = Some(id_inliner);

    fixture.run();

    assert_eq!(
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
      LUAU_INSN_FBSLOT_SEALED
    );
  }
}

#[cfg(test)]
#[test]
fn feedback_vector_simple_call() {
  use ulua_common::{FFlag, FInt, enums::luau_proto_flag::LuauProtoFlag};
  use ulua_vm::enums::feedback_vector_slot_kind::FeedbackVectorSlotKind;

  use crate::common::{
    functions::id_inliner_with_assert::id_inliner_with_assert,
    records::{
      assert_inliner_data::AssertInlinerData, feedback_vector_fixture::FeedbackVectorFixture,
    },
    type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
  };

  let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);
  let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
  let _inline_threshold = ScopedFastInt::new(&FInt::LuauInlineHitsThreshold, 2);

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(String::from(
    r#"
        local function g() return 1 end
        local function f() return g() + 1 end
        f()
        f()
    "#,
  ));

  assert_eq!(
    format!("\n{}", fixture.bcb.dump_function(1)),
    r#"
GETUPVAL R1 0
CALLFB R1 0 1 [0]
LOADK R2 K0 [1]
ADD R0 R1 R2
RETURN R0 1
"#
  );

  let top = fixture.load();

  unsafe {
    let g = *(*top).p.add(0);
    assert_ne!((*g).flags & LuauProtoFlag::LPF_INLINABLE as u8, 0);

    let f = *(*top).p.add(1);
    assert_eq!((*f).feedbackvecsize, 1);

    let fbslot = (*f).feedbackvec.add(0);
    assert_eq!((*fbslot).kind, FeedbackVectorSlotKind::CallTarget);
    assert_eq!((*fbslot).data.call_target.pc, 1);
    assert_eq!((*fbslot).data.call_target.proto, 0);
    assert_eq!((*fbslot).data.call_target.hits, 0);
    assert_eq!(
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
      0
    );

    let data = (*(*fixture.lua_state()).global).ecbdata.as_mut_ptr() as *mut AssertInlinerData;
    data.write(AssertInlinerData {
      proto: f,
      target: g,
      pc: (*fbslot).data.call_target.pc,
      called: false,
    });
    fixture.on_inline = Some(id_inliner_with_assert);

    fixture.run();

    assert_eq!((*fbslot).data.call_target.pc, 1);
    assert_eq!((*fbslot).data.call_target.proto, (*g).funid);
    assert_eq!((*fbslot).data.call_target.hits, 2);
    assert!((*data).called);
  }
}

#[cfg(test)]
#[test]
fn feedback_vector_simple_call_sealed() {
  use ulua_common::{FFlag, FInt, macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED};

  use crate::common::{
    functions::id_inliner::id_inliner,
    records::feedback_vector_fixture::FeedbackVectorFixture,
    type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
  };

  let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);
  let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
  let _inline_threshold = ScopedFastInt::new(&FInt::LuauInlineHitsThreshold, 2);

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(String::from(
    r#"
        local function g() return 1 end
        local function f() return g() + 1 end
        f()
        f()
    "#,
  ));

  let top = fixture.load();

  unsafe {
    let f = *(*top).p.add(1);
    let fbslot = (*f).feedbackvec.add(0);

    assert_eq!(
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
      0
    );
    *(*f).code.add((*fbslot).data.call_target.pc as usize + 1) = LUAU_INSN_FBSLOT_SEALED;

    fixture.on_inline = Some(id_inliner);

    fixture.run();

    assert_eq!((*fbslot).data.call_target.proto, 0);
    assert_eq!((*fbslot).data.call_target.hits, 0);
  }
}

#[cfg(test)]
#[test]
fn feedback_vector_simple_call_sealed_on_inline() {
  use ulua_common::{FFlag, FInt, macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED};

  use crate::common::{
    functions::sealing_inliner::sealing_inliner,
    records::feedback_vector_fixture::FeedbackVectorFixture,
    type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
  };

  let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);
  let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
  let _inline_threshold = ScopedFastInt::new(&FInt::LuauInlineHitsThreshold, 2);

  let mut fixture = FeedbackVectorFixture::new();
  fixture.compile(String::from(
    r#"
        local function g() return 1 end
        local function f() return g() + 1 end
        f()
        f()
    "#,
  ));

  let top = fixture.load();

  unsafe {
    let f = *(*top).p.add(1);
    let fbslot = (*f).feedbackvec.add(0);

    assert_eq!(
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
      0
    );

    fixture.on_inline = Some(sealing_inliner);

    fixture.run();

    assert_eq!(
      *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
      LUAU_INSN_FBSLOT_SEALED
    );
  }
}

#[cfg(test)]
#[test]
fn shared_code_allocator_anonymous_module_lifetime() {
  use alloc::vec::Vec;

  use ulua_code_gen::{
    functions::{
      create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32,
      get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{code_allocator::CodeAllocator, shared_code_allocator::SharedCodeAllocator},
  };

  if luau_codegen_supported() == 0 {
    return;
  }

  const K_BLOCK_SIZE: usize = 1024 * 1024;
  const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;

  let mut code_allocator = CodeAllocator::default();
  code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

  let mut allocator = SharedCodeAllocator::default();
  allocator.shared_code_allocator_code_allocator(&mut code_allocator as *mut _);

  let data = [0u8; 8];
  let code = [0u8; 8];

  let mut native_protos = Vec::with_capacity(1);

  {
    let native_proto = create_native_proto_exec_data_u32_u32(2, 0);
    unsafe {
      let header = get_native_proto_exec_data_header_mut(native_proto.as_ptr());
      (*header).bytecode_id = 1;
      (*header).entry_offset_or_address = null::<u8>();
      *native_proto.as_ptr().add(0) = 0;
      *native_proto.as_ptr().add(1) = 4;
    }
    native_protos.push(native_proto);
  }

  let mut mod_ref = unsafe {
    allocator.insert_anonymous_native_module(
      native_protos,
      data.as_ptr(),
      data.len(),
      code.as_ptr(),
      code.len(),
    )
  };
  assert!(!mod_ref.native_module_ref_empty());

  let module = mod_ref.native_module_ref_get();
  unsafe {
    assert!(!(*module).native_module_get_module_base_address().is_null());
    assert!(!(*module).native_module_try_get_native_proto(1).is_null());
    assert_eq!(1, (*module).native_module_get_refcount());
  }

  unsafe {
    (*module).native_module_add_ref();
    assert_eq!(2, (*module).native_module_get_refcount());
  }

  mod_ref.native_module_ref_reset();
  unsafe {
    assert_eq!(1, (*module).native_module_get_refcount());
    (*module).release();
  }
}

#[cfg(test)]
#[test]
fn shared_code_allocator_native_module_ref_refcounting() {
  use alloc::vec::Vec;

  use ulua_code_gen::{
    functions::luau_codegen_supported::luau_codegen_supported,
    records::{
      code_allocator::CodeAllocator, native_module_ref::NativeModuleRef,
      shared_code_allocator::SharedCodeAllocator,
    },
  };

  use crate::common::functions::shared_code_allocator_module_id::shared_code_allocator_module_id as module_id;

  if luau_codegen_supported() == 0 {
    return;
  }

  const K_BLOCK_SIZE: usize = 1024 * 1024;
  const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;
  const FAKE_CODE: [u8; 1] = [0x00];

  let mut code_allocator = CodeAllocator::default();
  code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

  let mut allocator = SharedCodeAllocator::default();
  allocator.shared_code_allocator_code_allocator(&mut code_allocator as *mut _);

  let refcount = |module_ref: &NativeModuleRef| unsafe {
    (*module_ref.native_module_ref_get()).native_module_get_refcount()
  };

  assert!(
    allocator
      .try_get_native_module(&module_id(0x0a))
      .native_module_ref_empty()
  );

  let mut mod_ref_a = unsafe {
    allocator.get_or_insert_native_module(
      &module_id(0x0a),
      Vec::new(),
      null(),
      0,
      FAKE_CODE.as_ptr(),
      FAKE_CODE.len(),
    )
  }
  .0;
  assert!(!mod_ref_a.native_module_ref_empty());

  assert_eq!(
    allocator
      .try_get_native_module(&module_id(0x0a))
      .native_module_ref_get(),
    mod_ref_a.native_module_ref_get()
  );

  assert_eq!(
    unsafe {
      allocator.get_or_insert_native_module(
        &module_id(0x0a),
        Vec::new(),
        null(),
        0,
        FAKE_CODE.as_ptr(),
        FAKE_CODE.len(),
      )
    }
    .0
    .native_module_ref_get(),
    mod_ref_a.native_module_ref_get()
  );

  assert!(
    allocator
      .try_get_native_module(&module_id(0x0b))
      .native_module_ref_empty()
  );

  let mod_ref_b = unsafe {
    allocator.get_or_insert_native_module(
      &module_id(0x0b),
      Vec::new(),
      null(),
      0,
      FAKE_CODE.as_ptr(),
      FAKE_CODE.len(),
    )
  }
  .0;
  assert!(!mod_ref_b.native_module_ref_empty());
  assert_ne!(
    mod_ref_b.native_module_ref_get(),
    mod_ref_a.native_module_ref_get()
  );

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mod_ref1 = mod_ref_a.clone();
    assert_eq!(
      mod_ref1.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mod_ref1 = NativeModuleRef::default();
    let mod_ref2 = mod_ref1.clone();
    assert!(mod_ref1.native_module_ref_empty());
    assert!(mod_ref2.native_module_ref_empty());
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = mod_ref_a.clone();
    let mod_ref2 = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
    assert!(mod_ref1.native_module_ref_empty());
    assert_eq!(
      mod_ref2.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = NativeModuleRef::default();
    let mod_ref2 = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
    assert!(mod_ref1.native_module_ref_empty());
    assert!(mod_ref2.native_module_ref_empty());
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = NativeModuleRef::default();
    mod_ref1.native_module_ref_operator_assign(mod_ref_a.clone());
    assert_eq!(
      mod_ref1.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mod_ref1 = NativeModuleRef::default();
    let mut mod_ref2 = NativeModuleRef::default();
    mod_ref2.native_module_ref_operator_assign(mod_ref1.clone());
    assert!(mod_ref1.native_module_ref_empty());
    assert!(mod_ref2.native_module_ref_empty());
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = mod_ref_a.clone();
    mod_ref1.native_module_ref_operator_assign(mod_ref1.clone());
    assert_eq!(
      mod_ref1.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mod_ref1 = mod_ref_a.clone();
    let mut mod_ref2 = mod_ref_b.clone();
    mod_ref2.native_module_ref_operator_assign(mod_ref1.clone());
    assert_eq!(
      mod_ref1.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(
      mod_ref2.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(3, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = mod_ref_a.clone();
    let mut mod_ref2 = NativeModuleRef::default();
    let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
    mod_ref2.native_module_ref_operator_assign(moved);
    assert!(mod_ref1.native_module_ref_empty());
    assert_eq!(
      mod_ref2.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = NativeModuleRef::default();
    let mut mod_ref2 = NativeModuleRef::default();
    let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
    mod_ref2.native_module_ref_operator_assign(moved);
    assert!(mod_ref1.native_module_ref_empty());
    assert!(mod_ref2.native_module_ref_empty());
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  #[cfg(not(target_os = "linux"))]
  {
    let mut mod_ref1 = mod_ref_a.clone();
    let mod_ref1_ptr: *mut NativeModuleRef = &mut mod_ref1;
    unsafe {
      let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut *mod_ref1_ptr);
      (*mod_ref1_ptr).native_module_ref_operator_assign(moved);
    }
    assert_eq!(
      mod_ref1.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = mod_ref_a.clone();
    let mut mod_ref2 = mod_ref_b.clone();
    let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
    mod_ref2.native_module_ref_operator_assign(moved);
    assert!(mod_ref1.native_module_ref_empty());
    assert_eq!(
      mod_ref2.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = NativeModuleRef::default();
    mod_ref1.native_module_ref_reset();
    assert!(mod_ref1.native_module_ref_empty());
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = mod_ref_a.clone();
    mod_ref1.native_module_ref_reset();
    assert!(mod_ref1.native_module_ref_empty());
    assert_eq!(1, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = mod_ref_a.clone();
    let mut mod_ref2 = mod_ref_b.clone();
    mod_ref1.native_module_ref_swap(&mut mod_ref2);
    assert_eq!(
      mod_ref1.native_module_ref_get(),
      mod_ref_b.native_module_ref_get()
    );
    assert_eq!(
      mod_ref2.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
    assert_eq!(2, refcount(&mod_ref_b));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  mod_ref_a.native_module_ref_reset();
  assert!(
    allocator
      .try_get_native_module(&module_id(0x0a))
      .native_module_ref_empty()
  );
}

#[cfg(test)]
#[test]
fn shared_code_allocator_native_proto_refcounting() {
  use alloc::vec::Vec;

  use ulua_code_gen::{
    functions::{
      create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32,
      get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{code_allocator::CodeAllocator, shared_code_allocator::SharedCodeAllocator},
  };

  use crate::common::functions::shared_code_allocator_module_id::shared_code_allocator_module_id as module_id;

  if luau_codegen_supported() == 0 {
    return;
  }

  const K_BLOCK_SIZE: usize = 1024 * 1024;
  const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;
  const FAKE_CODE: [u8; 1] = [0x00];

  let mut code_allocator = CodeAllocator::default();
  code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

  let mut allocator = SharedCodeAllocator::default();
  allocator.shared_code_allocator_code_allocator(&mut code_allocator as *mut _);

  let mut native_protos = Vec::with_capacity(1);
  let native_proto = create_native_proto_exec_data_u32_u32(0, 0);
  unsafe {
    (*get_native_proto_exec_data_header_mut(native_proto.as_ptr())).bytecode_id = 0x01;
  }
  native_protos.push(native_proto);

  let mut mod_ref_a = unsafe {
    allocator.get_or_insert_native_module(
      &module_id(0x0a),
      native_protos,
      null(),
      0,
      FAKE_CODE.as_ptr(),
      FAKE_CODE.len(),
    )
  }
  .0;
  assert!(!mod_ref_a.native_module_ref_empty());
  assert_eq!(1, unsafe {
    (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
  });

  unsafe {
    (*mod_ref_a.native_module_ref_get()).native_module_add_ref();
  }
  assert_eq!(2, unsafe {
    (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
  });

  unsafe {
    (*mod_ref_a.native_module_ref_get()).native_module_add_refs(2);
  }
  assert_eq!(4, unsafe {
    (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
  });

  unsafe {
    (*mod_ref_a.native_module_ref_get()).release();
  }
  assert_eq!(3, unsafe {
    (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
  });

  unsafe {
    (*mod_ref_a.native_module_ref_get()).release();
  }
  assert_eq!(2, unsafe {
    (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
  });

  mod_ref_a.native_module_ref_reset();

  mod_ref_a = allocator.try_get_native_module(&module_id(0x0a));
  assert!(!mod_ref_a.native_module_ref_empty());
  assert_eq!(2, unsafe {
    (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
  });

  let raw_mod_a = mod_ref_a.native_module_ref_get();

  mod_ref_a.native_module_ref_reset();
  unsafe {
    (*raw_mod_a).release();
  }
  assert!(
    allocator
      .try_get_native_module(&module_id(0x0a))
      .native_module_ref_empty()
  );
}

#[cfg(test)]
#[test]
fn shared_code_allocator_native_proto_state() {
  use alloc::vec::Vec;

  use ulua_code_gen::{
    functions::{
      create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32,
      get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut,
      get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{code_allocator::CodeAllocator, shared_code_allocator::SharedCodeAllocator},
  };

  use crate::common::functions::shared_code_allocator_module_id::shared_code_allocator_module_id as module_id;

  if luau_codegen_supported() == 0 {
    return;
  }

  const K_BLOCK_SIZE: usize = 1024 * 1024;
  const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;

  let mut code_allocator = CodeAllocator::default();
  code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

  let mut allocator = SharedCodeAllocator::default();
  allocator.shared_code_allocator_code_allocator(&mut code_allocator as *mut _);

  let data = [0u8; 16];
  let code = [0u8; 16];

  let mut native_protos = Vec::with_capacity(2);

  {
    let native_proto = create_native_proto_exec_data_u32_u32(2, 0);
    unsafe {
      let header = get_native_proto_exec_data_header_mut(native_proto.as_ptr());
      (*header).bytecode_id = 1;
      (*header).entry_offset_or_address = null::<u8>();
      *native_proto.as_ptr().add(0) = 0;
      *native_proto.as_ptr().add(1) = 4;
    }
    native_protos.push(native_proto);
  }

  {
    let native_proto = create_native_proto_exec_data_u32_u32(2, 0);
    unsafe {
      let header = get_native_proto_exec_data_header_mut(native_proto.as_ptr());
      (*header).bytecode_id = 3;
      (*header).entry_offset_or_address = 0x08usize as *const u8;
      *native_proto.as_ptr().add(0) = 8;
      *native_proto.as_ptr().add(1) = 12;
    }
    native_protos.push(native_proto);
  }

  let mod_ref_a = unsafe {
    allocator.get_or_insert_native_module(
      &module_id(0x0a),
      native_protos,
      data.as_ptr(),
      data.len(),
      code.as_ptr(),
      code.len(),
    )
  }
  .0;
  assert!(!mod_ref_a.native_module_ref_empty());

  let module = unsafe { &*mod_ref_a.native_module_ref_get() };
  let module_base_address = module.native_module_get_module_base_address();
  assert!(!module_base_address.is_null());

  let proto1 = module.native_module_try_get_native_proto(1);
  assert!(!proto1.is_null());
  unsafe {
    let header = get_native_proto_exec_data_header(proto1);
    assert_eq!(1, (*header).bytecode_id);
    assert_eq!(
      module_base_address.add(0x00),
      (*header).entry_offset_or_address
    );
    assert_eq!(0, *proto1.add(0));
    assert_eq!(4, *proto1.add(1));
  }

  let proto3 = module.native_module_try_get_native_proto(3);
  assert!(!proto3.is_null());
  unsafe {
    let header = get_native_proto_exec_data_header(proto3);
    assert_eq!(3, (*header).bytecode_id);
    assert_eq!(
      module_base_address.add(0x08),
      (*header).entry_offset_or_address
    );
    assert_eq!(8, *proto3.add(0));
    assert_eq!(12, *proto3.add(1));
  }

  assert!(module.native_module_try_get_native_proto(0).is_null());
  assert!(module.native_module_try_get_native_proto(2).is_null());
  assert!(module.native_module_try_get_native_proto(4).is_null());
}

#[cfg(test)]
#[test]
fn shared_code_allocator_shared_allocation() {
  use core::ffi::{c_char, c_void};

  use ulua_code_gen::{
    enums::{code_gen_compilation_result::CodeGenCompilationResult, code_gen_flags::CodeGenFlags},
    functions::{
      compile_internal::compile_internal, create_code_gen_context_alt_d::create,
      create_shared_code_gen_context_code_gen_context::create_shared_code_gen_context,
      destroy_shared_code_gen_context::destroy_shared_code_gen_context,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{
      compilation_options::CompilationOptions, compilation_stats::CompilationStats,
      shared_code_gen_context::SharedCodeGenContext,
    },
    type_aliases::unique_shared_code_gen_context::UniqueSharedCodeGenContext,
  };
  use ulua_compiler::functions::luau_compile::luau_compile;
  use ulua_vm::functions::{lua_l_newstate::lua_l_newstate, luau_load::luau_load};

  use crate::common::{
    functions::shared_code_allocator_module_id::shared_code_allocator_module_id as module_id,
    type_aliases::state_ref::StateRef,
  };

  unsafe extern "C" {
    fn free(ptr: *mut c_void);
  }

  struct SharedContextRef(UniqueSharedCodeGenContext);

  impl SharedContextRef {
    fn as_ptr(&self) -> *mut SharedCodeGenContext {
      self.0.as_ptr()
    }
  }

  impl Drop for SharedContextRef {
    fn drop(&mut self) {
      unsafe {
        destroy_shared_code_gen_context(self.as_ptr());
      }
    }
  }

  if luau_codegen_supported() == 0 {
    return;
  }

  let shared_code_gen_context = SharedContextRef(create_shared_code_gen_context());

  let state1 = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let state2 = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l1 = state1.as_ptr();
  let l2 = state2.as_ptr();

  unsafe { create(l1, shared_code_gen_context.as_ptr()) };
  unsafe { create(l2, shared_code_gen_context.as_ptr()) };

  let source = r#"
        function add(x, y) return x + y end
        function sub(x, y) return x - y end
    "#;

  let mut bytecode_size = 0usize;
  let bytecode = unsafe {
    luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      null_mut(),
      &mut bytecode_size,
    )
  };
  assert!(!bytecode.is_null());

  unsafe {
    let load_result1 = luau_load(l1, c"=Functions".as_ptr(), bytecode, bytecode_size, 0);
    let load_result2 = luau_load(l2, c"=Functions".as_ptr(), bytecode, bytecode_size, 0);
    free(bytecode as *mut c_void);

    assert_eq!(0, load_result1);
    assert_eq!(0, load_result2);
  }

  let module_id = module_id(0x01);

  let options = CompilationOptions {
    flags: CodeGenFlags::CodeGenColdFunctions as u32,
    ..Default::default()
  };
  let mut native_stats1 = CompilationStats::default();
  let mut native_stats2 = CompilationStats::default();
  let code_gen_result1 = unsafe {
    compile_internal(
      &Some(module_id),
      l1,
      -1,
      &options,
      &mut native_stats1 as *mut CompilationStats,
    )
  };
  let code_gen_result2 = unsafe {
    compile_internal(
      &Some(module_id),
      l2,
      -1,
      &options,
      &mut native_stats2 as *mut CompilationStats,
    )
  };

  assert_eq!(CodeGenCompilationResult::Success, code_gen_result1.result);
  assert_eq!(CodeGenCompilationResult::Success, code_gen_result2.result);

  assert_eq!(3, native_stats1.functions_total);
  assert_eq!(3, native_stats2.functions_total);

  assert_eq!(3, native_stats1.functions_compiled);
  assert_eq!(0, native_stats2.functions_compiled);

  assert_eq!(3, native_stats1.functions_bound);
  assert_eq!(3, native_stats2.functions_bound);
}

#[cfg(test)]
#[test]
fn conformance_large_module_a64() {
  use core::ffi::{c_char, c_void};
  use std::string::String;

  use ulua_code_gen::{
    enums::{
      code_gen_compilation_result::CodeGenCompilationResult, code_gen_flags::CodeGenFlags,
      function_stats_flags::FunctionStatsFlags, target::Target,
    },
    functions::{
      compile_internal::compile_internal, get_assembly::get_assembly,
      luau_codegen_create::luau_codegen_create, luau_codegen_supported::luau_codegen_supported,
    },
    records::{
      assembly_options::AssemblyOptions, compilation_options::CompilationOptions,
      compilation_stats::CompilationStats, lowering_stats::LoweringStats,
    },
  };
  use ulua_compiler::{
    functions::luau_compile::luau_compile, records::lua_compile_options::LuaCompileOptions,
  };
  use ulua_vm::{
    functions::{
      lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_resume::lua_resume,
      luau_load::luau_load,
    },
    lua_tonumber,
  };

  use crate::common::{functions::run_conformance::CODEGEN, type_aliases::state_ref::StateRef};

  unsafe extern "C" {
    fn free(ptr: *mut c_void);
  }

  // C++ 还设 LuauCodegenA64FarRefs/LuauCodegenProtectData，但 Rust 移植无读取点。
  const FILLER_COUNT: usize = 60;
  const LINES_PER_FILLER: usize = 2000;
  const EXPECTED_RESULT: f64 = 140_000.0;

  let mut source = String::new();
  for i in 0..FILLER_COUNT {
    source.push_str(&format!("function filler{i}(x: number)\n"));
    for k in 1..=LINES_PER_FILLER {
      source.push_str(&format!("    x = x + {k}\n"));
    }
    source.push_str("    return x\nend\n");
  }

  // 常量刻意选为无法 fmov 下沉，必须分配到数据段。
  source.push_str("function trigger(x: number)\n");
  source.push_str("    x = x + 0.1\n");
  source.push_str("    x = x + 0.3\n");
  source.push_str("    return x\n");
  source.push_str("end\n");
  source.push_str("return math.floor(trigger(1) * 100000)\n");

  let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    if luau_codegen_supported() != 0 {
      luau_codegen_create(l);
    }

    lua_l_openlibs(l);

    let mut opts = LuaCompileOptions {
      optimization_level: 2,
      debug_level: 1,
      type_info_level: 1,
      coverage_level: 0,
      vector_lib: null(),
      vector_ctor: null(),
      vector_type: null(),
      mutable_globals: null(),
      userdata_types: null(),
      libraries_with_known_members: null(),
      library_member_type_cb: None,
      library_member_constant_cb: None,
      disabled_builtins: null(),
    };

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      &mut opts as *mut _,
      &mut bytecode_size,
    );
    let result = luau_load(l, c"=LargeModuleA64".as_ptr(), bytecode, bytecode_size, 0);
    free(bytecode as *mut c_void);
    assert_eq!(0, result);

    if luau_codegen_supported() != 0 {
      let native_opts = CompilationOptions {
        flags: CodeGenFlags::CodeGenColdFunctions as u32,
        ..Default::default()
      };
      let mut stats = LoweringStats {
        function_stats_flags: FunctionStatsFlags::FunctionStatsEnable as u32,
        ..Default::default()
      };
      let mut assembly_options = AssemblyOptions {
        target: Target::A64,
        compilation_options: native_opts.clone(),
        output_binary: false,
        include_assembly: true,
        include_ir: true,
        include_outlined_code: true,
        include_ir_types: true,
        include_ir_prefix: Default::default(),
        include_use_info: Default::default(),
        include_cfg_info: Default::default(),
        include_reg_flow_info: Default::default(),
        annotator: None,
        annotator_context: null_mut(),
      };
      let a64 = get_assembly(l, -1, assembly_options.clone(), &mut stats);
      assert!(!a64.is_empty());
      assert_eq!(0, stats.reg_alloc_errors);
      assert_eq!(0, stats.lowering_errors);

      assembly_options.target = Target::X64SystemV;
      let x64 = get_assembly(l, -1, assembly_options, &mut stats);
      assert!(!x64.is_empty());
      assert_eq!(0, stats.reg_alloc_errors);
      assert_eq!(0, stats.lowering_errors);
    }

    if CODEGEN && luau_codegen_supported() != 0 {
      let native_options = CompilationOptions {
        flags: CodeGenFlags::CodeGenColdFunctions as u32,
        ..Default::default()
      };
      let mut native_stats = CompilationStats::default();
      let native_result = compile_internal(&None, l, -1, &native_options, &mut native_stats);
      assert_eq!(CodeGenCompilationResult::Success, native_result.result);
      assert!(!native_result.has_errors());
    }

    let status = lua_resume(l, null_mut(), 0);
    assert_eq!(0, status);
    assert_eq!(EXPECTED_RESULT, lua_tonumber!(l, -1));
  }
}
