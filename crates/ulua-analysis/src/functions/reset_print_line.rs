use crate::functions::{
  default_luau_print_line::default_luau_print_line, set_print_line::LUAU_PRINT_LINE,
};
extern "C-unwind" fn default_luau_print_line_wrapper(s: &String) {
  default_luau_print_line(s)
}

pub fn reset_print_line() {
  unsafe {
    LUAU_PRINT_LINE = Some(default_luau_print_line_wrapper);
  }
}
