use crate::functions::{
  default_luau_print_line::default_luau_print_line, set_print_line::set_print_line,
};

pub fn reset_print_line() {
  set_print_line(Some(default_luau_print_line));
}
