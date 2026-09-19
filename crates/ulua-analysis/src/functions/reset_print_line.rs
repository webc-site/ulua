use crate::functions::{
  default_luau_print_line::default_luau_print_line, set_print_line::set_print_line,
};
extern "C-unwind" fn default_luau_print_line_wrapper(s: &String) {
  default_luau_print_line(s)
}

pub fn reset_print_line() {
  set_print_line(Some(default_luau_print_line_wrapper));
}
