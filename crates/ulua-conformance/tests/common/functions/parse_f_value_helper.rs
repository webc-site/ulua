use alloc::string::String;

use crate::common::type_aliases::f_value_result::FValueResult;

pub fn parse_f_value_helper(view: &str) -> FValueResult<Option<String>> {
  if let Some(equal_pos) = view.find('=') {
    let name = String::from(&view[..equal_pos]);
    let value = String::from(&view[equal_pos + 1..]);
    (name, Some(value))
  } else {
    (String::from(view), None)
  }
}
