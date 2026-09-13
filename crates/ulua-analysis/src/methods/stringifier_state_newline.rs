//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:347:stringifier_state_newline`
//! Source: `Analysis/src/ToString.cpp:347-354` (hand-ported)

use crate::records::stringifier_state::StringifierState;

impl StringifierState {
  /// C++ `void newline()` — `if (!opts.useLineBreaks) return emit(" ");`
  /// (an earlier translation dropped the space, gluing separators).
  pub fn newline(&mut self) {
    let use_line_breaks = unsafe { (*self.opts).use_line_breaks };
    if !use_line_breaks {
      return self.emit_string(" ");
    }

    self.emit_string("\n");
    self.emit_indentation();
  }
}
