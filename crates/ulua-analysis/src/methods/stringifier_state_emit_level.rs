//! Source: `Analysis/src/ToString.cpp:262-278` (hand-ported)

use alloc::format;

use ulua_common::fint;

use crate::records::{
  scope::Scope, scope_registry::resolve_scope, stringifier_state::StringifierState,
};
impl StringifierState {
  /// C++ `void emitLevel(Scope* scope)`.
  ///
  /// # Safety
  /// `scope` 须为空或指向 `ToString` 调用期内存活的 `Scope`（对应 cpp 裸
  /// `Scope*` 形参契约）；函数体内仅此一处 `as_ref` 只读解引用。
  pub unsafe fn emit_level(&mut self, scope: *mut Scope) {
    let mut count: usize = 0;
    // 句柄化上溯：仅计数深度，走只读 resolve_scope 出口。
    let mut s = unsafe { scope.as_ref() };
    while let Some(scope_ref) = s {
      count += 1;
      s = scope_ref.parent.and_then(resolve_scope);
    }

    self.emit(&count);

    if fint::DebugLuauVerboseTypeNames.get() >= 3 {
      self.emit("-");
      // snprintf(Buffer, 16, "0x%x", uint32_t(intptr_t(scope) & 0xFFFFFF))
      let v = (scope as usize as u32) & 0xFFFFFF;
      let buffer = format!("0x{:x}", v);
      self.emit(buffer.as_str());
    }
  }
}
