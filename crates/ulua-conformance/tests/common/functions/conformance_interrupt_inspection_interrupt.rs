use core::{
  ffi::c_int,
  sync::atomic::{AtomicBool, Ordering},
};

use ulua_vm::{
  functions::{lua_break::lua_break, lua_isyieldable::lua_isyieldable},
  records::lua_state::lua_State,
};

/// cpp 的 `static bool skipBreak`；每次中断在 break/skip 之间交替，并行测试下
/// `static mut` 即数据竞争，故用 `AtomicBool`。
static SKIP_BREAK: AtomicBool = AtomicBool::new(false);

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_inspection_interrupt(
  l: *mut lua_State,
  gc: c_int,
) {
  unsafe {
    if gc >= 0 {
      return;
    }

    if lua_isyieldable(l) == 0 {
      return;
    }

    let skip = SKIP_BREAK.load(Ordering::Relaxed);
    if !skip {
      lua_break(l);
    }

    SKIP_BREAK.store(!skip, Ordering::Relaxed);
  }
}
