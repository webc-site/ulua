//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:262:stringifier_state_emit_level`
//! Source: `Analysis/src/ToString.cpp:262-278` (hand-ported)

use alloc::{format, sync::Arc};
use core::ptr::null;

use ulua_common::FInt;

use crate::records::{scope::Scope, stringifier_state::StringifierState};
impl StringifierState {
  /// C++ `void emitLevel(Scope* scope)`.
  pub fn emit_level(&mut self, scope: *mut Scope) {
    unsafe {
      let mut count: usize = 0;
      let mut s = scope as *const Scope;
      while !s.is_null() {
        count += 1;
        s = match &(*s).parent {
          Some(p) => Arc::as_ptr(p),
          None => null(),
        };
      }

      self.emit(&count);

      if FInt::DebugLuauVerboseTypeNames.get() >= 3 {
        self.emit("-");
        // snprintf(Buffer, 16, "0x%x", uint32_t(intptr_t(scope) & 0xFFFFFF))
        let v = (scope as usize as u32) & 0xFFFFFF;
        let buffer = format!("0x{:x}", v);
        self.emit(buffer.as_str());
      }
    }
  }
}
