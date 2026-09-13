use crate::records::stringifier_state::StringifierState;

impl StringifierState {
  pub fn emit_string(&mut self, s: &str) {
    if self.opts.is_null() {
      return;
    }

    let max_type_length = unsafe { (*self.opts).max_type_length };
    if max_type_length > 0 {
      let result_name = unsafe { &(*self.result).name };
      if result_name.len() > max_type_length {
        return;
      }
    }

    unsafe { (*self.result).name.push_str(s) };
  }
}
