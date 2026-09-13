use crate::common::{
  functions::parse_f_value_helper::parse_f_value_helper, type_aliases::f_value_result::FValueResult,
};

pub fn parse_f_int(view: &str) -> FValueResult<i32> {
  let (name, value) = parse_f_value_helper(view);
  let s = value.unwrap_or_else(|| panic!("Expected a value associated with {}", name));
  let n: i32 = s.parse().expect("invalid integer");
  (name, n)
}
