use core::{ffi::CStr, mem::zeroed, ptr::null_mut, sync::atomic::Ordering};

use ulua_vm::{
  functions::{
    lua_checkstack::lua_checkstack, lua_getargument::lua_getargument, lua_getinfo::lua_getinfo,
    lua_getlocal::lua_getlocal, lua_getupvalue::lua_getupvalue, lua_resume::lua_resume,
  },
  macros::{
    lua_isnil::lua_isnil, lua_minstack::LUA_MINSTACK, lua_pop::lua_pop,
    lua_tointeger::lua_tointeger,
  },
  records::{lua_debug::LuaDebug, lua_state::lua_State},
};

use crate::common::records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_yield(l: *mut lua_State) -> bool {
  unsafe {
    let breakhits = CONFORMANCE_DEBUGGER_STATE.breakhits.load(Ordering::SeqCst);

    assert_eq!(breakhits % 2, 1);
    lua_checkstack(l, LUA_MINSTACK);

    macro_rules! assert_stack_integer {
      ($value:expr) => {{
        assert_eq!(lua_tointeger!(l, -1), $value);
        lua_pop(l, 1);
      }};
    }

    macro_rules! assert_local {
      ($level:expr, $slot:expr, $name:expr, $value:expr) => {{
        let local = lua_getlocal(l, $level, $slot);
        assert!(!local.is_null());
        assert_eq!(CStr::from_ptr(local).to_bytes(), $name);
        assert_stack_integer!($value);
      }};
    }

    if breakhits == 1 {
      assert_ne!(0, lua_getargument(l, 0, 1));
      assert_stack_integer!(50);

      assert_ne!(0, lua_getargument(l, 0, 2));
      assert_stack_integer!(42);

      assert_local!(0, 1, b"b", 50);

      let mut ar: LuaDebug = zeroed();
      lua_getinfo(l, 0, c"f".as_ptr(), &mut ar);

      let upvalue = lua_getupvalue(l, -1, 1);
      assert!(!upvalue.is_null());
      assert_eq!(CStr::from_ptr(upvalue).to_bytes(), b"a");
      assert_eq!(lua_tointeger!(l, -1), 5);
      lua_pop(l, 2);
    } else if breakhits == 3 {
      assert_local!(0, 1, b"a", 6);
    } else if breakhits == 5 {
      assert_local!(1, 1, b"a", 7);
    } else if breakhits == 7 {
      assert_local!(1, 1, b"a", 8);
    } else if breakhits == 9 {
      assert_local!(1, 1, b"a", 9);
    } else if breakhits == 13 {
      let local = lua_getlocal(l, 0, 1);
      assert!(!local.is_null());
      assert_eq!(CStr::from_ptr(local).to_bytes(), b"a");
      assert!(lua_isnil!(l, -1));
      lua_pop(l, 1);
    } else if breakhits == 15 {
      let x = lua_getlocal(l, 2, 1);
      assert!(!x.is_null());
      assert_eq!(CStr::from_ptr(x).to_bytes(), b"x");
      lua_pop(l, 1);

      let a1 = lua_getlocal(l, 2, 2);
      assert!(a1.is_null());
    }

    let interruptedthread = CONFORMANCE_DEBUGGER_STATE
      .interruptedthread
      .swap(null_mut(), Ordering::SeqCst);
    if !interruptedthread.is_null() {
      lua_resume(interruptedthread, null_mut(), 0);
    }

    false
  }
}
