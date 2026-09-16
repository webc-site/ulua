use core::{ffi::c_int, mem::zeroed, sync::atomic::Ordering};

use ulua_vm::{
  functions::{lua_getinfo::lua_getinfo, lua_yield::lua_yield},
  luaL_error,
  records::{lua_debug::LuaDebug, lua_state::lua_State},
};

use crate::common::records::conformance_interrupt_state::{
  CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS, CONFORMANCE_INTERRUPT_MODE_INFLOOP,
  CONFORMANCE_INTERRUPT_MODE_TIMEOUT, CONFORMANCE_INTERRUPT_STATE,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_interrupt(l: *mut lua_State, gc: c_int) {
  unsafe {
    if gc >= 0 {
      return;
    }

    match CONFORMANCE_INTERRUPT_STATE.mode.load(Ordering::SeqCst) {
      CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS => {
        static EXPECTED_HITS: [i32; 22] = [
          11, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 20, 15, 15, 15, 15, 18, 25, 23, 26,
        ];

        let index = CONFORMANCE_INTERRUPT_STATE
          .index
          .fetch_add(1, Ordering::SeqCst);
        assert!(
          (index as usize) < EXPECTED_HITS.len(),
          "interrupt index {index} exceeded expected hit count"
        );

        let mut ar: LuaDebug = zeroed();
        lua_getinfo(l, 0, c"l".as_ptr(), &mut ar);
        assert_eq!(ar.currentline, EXPECTED_HITS[index as usize]);

        if index + 1 == 4 {
          lua_yield(l, 0);
        }
      }
      CONFORMANCE_INTERRUPT_MODE_INFLOOP => {
        let index = CONFORMANCE_INTERRUPT_STATE
          .index
          .fetch_add(1, Ordering::SeqCst)
          + 1;
        assert!(index <= 11, "interrupt index {index} exceeded infloop cap");

        if index == 11 {
          lua_yield(l, 0);
        }
      }
      CONFORMANCE_INTERRUPT_MODE_TIMEOUT => {
        let index = CONFORMANCE_INTERRUPT_STATE
          .index
          .fetch_add(1, Ordering::SeqCst)
          + 1;

        if index == 1_000 {
          CONFORMANCE_INTERRUPT_STATE.index.store(0, Ordering::SeqCst);
          luaL_error!(l, "timeout");
        }
      }
      mode => panic!("unknown conformance interrupt mode {mode}"),
    }
  }
}
