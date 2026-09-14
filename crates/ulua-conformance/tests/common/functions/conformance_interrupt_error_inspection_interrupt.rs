use core::{ffi::c_int, sync::atomic::Ordering};

use ulua_vm::{functions::lua_l_error_l::lua_l_error_l, records::lua_state::lua_State};

use crate::common::records::conformance_interrupt_error_inspection_state::CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_error_inspection_interrupt(
  l: *mut lua_State,
  gc: c_int,
) {
  unsafe {
    if gc >= 0 {
      return;
    }

    let step = CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE
      .step
      .load(Ordering::SeqCst);
    let target = CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE
      .target
      .load(Ordering::SeqCst);

    if step == target {
      lua_l_error_l(l, c"%s".as_ptr(), format_args!("test"));
    }

    CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE
      .step
      .store(step + 1, Ordering::SeqCst);
  }
}
