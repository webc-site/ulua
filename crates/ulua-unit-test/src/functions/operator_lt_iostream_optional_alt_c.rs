use alloc::{string::String, vec::Vec};
use core::fmt::Display;
pub fn operator_lt_ostream_vector_t<T: Display>(t: &Vec<T>) -> String {
  let mut result = String::from("{ ");
  let mut first = true;

  for element in t {
    if first {
      first = false;
    } else {
      result.push_str(", ");
    }

    result.push_str(&format!("{}", element));
  }

  result.push_str(" }");
  result
}
