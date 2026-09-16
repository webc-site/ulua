//! Node: `cxx:Function:Luau.VM:VM/src/ldo.cpp:124:lua_d_rawrunprotected`
//! Source: `VM/src/ldo.cpp:124-159` (hand-ported; C++-exceptions build
//! flavor — `luaD_throw` is `panic_any(lua_exception)`, this is the matching
//! `catch_unwind` boundary; see translation/design-cards/lvmexecute.md)

use alloc::{ffi::CString, string::String};
use core::{
  ffi::{c_int, c_void},
  ptr::eq,
};
use std::panic::{AssertUnwindSafe, catch_unwind};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    install_lua_exception_panic_hook::install_lua_exception_panic_hook,
    lua_g_pusherror::lua_g_pusherror,
  },
  records::lua_exception::lua_exception,
  type_aliases::{lua_state::lua_State, pfunc::Pfunc},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_d_rawrunprotected(l: *mut lua_State, f: Pfunc, ud: *mut c_void) -> c_int {
  unsafe {
    let mut status: i32 = 0;

    // Silence the default panic-hook noise for the VM's longjmp-emulation
    // unwinds (a caught `lua_exception` is a normal Lua error, not a crash).
    install_lua_exception_panic_hook();

    let result = catch_unwind(AssertUnwindSafe(|| {
      if let Some(f) = f {
        f(l, ud);
      }
    }));

    if let Err(payload) = result {
      if let Some(e) = payload.downcast_ref::<lua_exception>() {
        // It is assumed/required that the exception caught here was
        // thrown from the same Luau state (see C++ comment).
        LUAU_ASSERT!(eq(e.get_thread(), l));
        status = e.get_status();
      } else {
        // Luau will never throw this, but this can catch panics that
        // escape from Rust implementations of external functions —
        // the C++ `catch (std::exception&)` arm. Push the message so
        // error handling below can proceed.
        let msg: &str = if let Some(s) = payload.downcast_ref::<&str>() {
          s
        } else if let Some(s) = payload.downcast_ref::<String>() {
          s.as_str()
        } else {
          "unknown error"
        };
        let cmsg =
          CString::new(msg).unwrap_or_else(|_| CString::new("invalid error message").unwrap());
        // C++ nests a second try/catch for OOM while pushing; a Rust
        // allocation failure aborts, so the LUA_ERRMEM arm has no analog.
        lua_g_pusherror(l, cmsg.as_ptr());
        status = LuaStatus::ErrRun as c_int;
      }
    }

    status
  }
}

pub use lua_d_rawrunprotected as luaD_rawrunprotected;
