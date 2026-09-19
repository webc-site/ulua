//! Source: `Compiler/src/Compiler.cpp`

use crate::records::compiler::Compiler;

// RAII register-stack guard. Must NOT be `Copy`/`Clone`: the C++ `~RegScope`
// restores `regTop = oldTop` on scope exit, reproduced by the `Drop` below. The
// prior `Copy` derive silently elided that restore, leaving `regTop` un-rewound
// between sibling sub-expressions and tripping `assert!(top <= regTop)` in
// `reg_scope_compiler_i32`.
#[derive(Debug)]
pub struct RegScope {
  pub(crate) self_: *mut Compiler,
  pub(crate) old_top: u32,
}

impl Drop for RegScope {
  fn drop(&mut self) {
    // C++ `~RegScope() { self->regTop = oldTop; }`
    unsafe {
      (*self.self_).reg_top = self.old_top;
    }
  }
}
