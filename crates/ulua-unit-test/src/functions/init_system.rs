use ulua_code_gen::macros::codegen_target_x_64::CODEGEN_TARGET_X64;

/// Initialize system settings for unit tests.
/// Disables flush-to-zero and denormals-are-zero modes on x86_64
/// so that denormalized numbers behave correctly in tests.
pub fn init_system() {
  if CODEGEN_TARGET_X64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
      let mut mxcsr = core::arch::x86_64::_mm_getcsr();
      // Clear flush-to-zero bit (bit 15)
      mxcsr &= !(1 << 15);
      // Clear denormals-are-zero bit (bit 6)
      mxcsr &= !(1 << 6);
      core::arch::x86_64::_mm_setcsr(mxcsr);
    }
  }
}
