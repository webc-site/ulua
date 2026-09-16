use alloc::string::String;
use std::iter::repeat_n;

use crate::records::stringifier_state::StringifierState;
impl StringifierState {
  pub fn emit_indentation(&mut self) {
    let use_line_breaks = unsafe { (*self.opts).use_line_breaks };
    if !use_line_breaks {
      return;
    }

    let indent_str = String::from_iter(repeat_n(' ', self.indentation));
    self.emit_string(&indent_str);
  }
}
