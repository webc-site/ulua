use ulua_common::{FFlag, FInt};

use crate::records::native_stack_guard::NativeStackGuard;

impl NativeStackGuard {
  #[inline]
  pub fn is_ok(&self) -> bool {
    if !FFlag::LuauUseNativeStackGuard.get() || FInt::LuauStackGuardThreshold.get() <= 0 {
      return true;
    }

    // Linux is included now that the ctor (native_stack_guard, alt_b) populates
    // `low`/`high` there via pthread_getattr_np; previously Linux fell through
    // to the unconditional `true`, so the guard never tripped.
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
      let probe: u8 = 0;
      let sp = &probe as *const u8 as usize;
      let remaining = sp.wrapping_sub(self.low);

      remaining > FInt::LuauStackGuardThreshold.get() as usize
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    true
  }
}
