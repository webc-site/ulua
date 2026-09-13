use alloc::string::String;
use core::fmt::Display;
pub fn operator_lt_ostream_optional_t<T: Display>(t: Option<T>) -> String {
  match t {
    Some(v) => format!("{}", v),
    None => "none".to_owned(),
  }
}
