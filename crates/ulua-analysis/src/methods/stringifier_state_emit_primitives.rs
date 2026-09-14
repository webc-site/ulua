//! Source: `Analysis/src/ToString.cpp:254-305` (hand-ported)
//! The C++ overloaded `emit(...)` family as a trait + generic method
//! (the AstJsonEncoder::write precedent).

use alloc::string::{String, ToString};

use crate::records::{stringifier_state::StringifierState, type_level::TypeLevel};

pub trait EmitText {
  fn emit_text(&self, state: &mut StringifierState);
}

impl StringifierState {
  /// Generic entry mirroring C++ overload resolution: `state.emit(x)`.
  pub fn emit<T: EmitText + ?Sized>(&mut self, value: &T) {
    value.emit_text(self);
  }

  fn emit_str_raw(&mut self, s: &str) {
    unsafe {
      // if (opts.maxTypeLength > 0 && result.name.length() > opts.maxTypeLength) return;
      let opts = &*self.opts;
      let result = &mut *self.result;
      if opts.max_type_length > 0 && result.name.len() > opts.max_type_length {
        return;
      }
      result.name.push_str(s);
    }
  }
}

impl EmitText for str {
  fn emit_text(&self, state: &mut StringifierState) {
    state.emit_str_raw(self);
  }
}
impl EmitText for String {
  fn emit_text(&self, state: &mut StringifierState) {
    state.emit_str_raw(self);
  }
}
impl EmitText for TypeLevel {
  fn emit_text(&self, state: &mut StringifierState) {
    state.emit_str_raw(&self.level.to_string());
    state.emit_str_raw("-");
    state.emit_str_raw(&self.sub_level.to_string());
  }
}
macro_rules! emit_int {
    ($($t:ty),*) => {$(
        impl EmitText for $t {
            fn emit_text(&self, state: &mut StringifierState) {
                state.emit_str_raw(&self.to_string());
            }
        }
    )*};
}
emit_int!(i32, i64, u32, u64, usize, isize);
