use alloc::string::String;

pub(crate) fn default_luau_print_line(s: &String) {
  println!("{}", s);
}
