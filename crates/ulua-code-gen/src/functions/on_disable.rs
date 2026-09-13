//! Node: `cxx:Function:Luau.CodeGen:CodeGen/src/CodeGen.cpp:100:on_disable`
//!
//! Disable native code for a proto: point its entry back at bytecode, clear the
//! exec target, and walk every thread's Lua call stack clearing the
//! `LUA_CALLINFO_NATIVE` flag on any frame still pointing at this proto.

use core::ffi::c_void;
use std::ptr::eq;

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::lua_m_visitgco::lua_m_visitgco,
  macros::{
    clvalue::clvalue, gco_2_th::gco2th, is_lua::isLua, lua_callinfo_native::LUA_CALLINFO_NATIVE,
  },
  records::{gc_object::GCObject, lua_page::lua_Page, lua_state::lua_State, proto::Proto},
};

unsafe fn on_disable_visitor(
  context: *mut c_void,
  _page: *mut lua_Page,
  gco: *mut GCObject,
) -> bool {
  unsafe {
    let proto = context as *mut Proto;

    if (*gco).gch.tt as i32 != LuaType::Thread as i32 {
      return false;
    }

    let th = gco2th!(gco);

    let mut ci = (*th).ci;
    while ci > (*th).base_ci {
      if isLua!(ci) {
        let f = &*clvalue!((*ci).func);
        let p = f.inner.l.p;

        if p == proto {
          (*ci).flags &= !(LUA_CALLINFO_NATIVE as u32);
        }
      }
      ci = ci.sub(1);
    }

    false
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn on_disable(l: *mut lua_State, proto: *mut Proto) {
  unsafe {
    // do nothing if proto already uses bytecode
    if eq((*proto).codeentry, (*proto).code) {
      return;
    }

    // ensure that VM does not call native code for this proto
    (*proto).codeentry = (*proto).code as *const _;

    // prevent native code from entering proto with breakpoints
    (*proto).exectarget = 0;

    lua_m_visitgco(l, proto as *mut c_void, on_disable_visitor as *mut c_void);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_on_disable")]
pub unsafe extern "C-unwind" fn on_disable_export(l: *mut lua_State, proto: *mut Proto) {
  unsafe {
    on_disable(l, proto);
  }
}
