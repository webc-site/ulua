use crate::{
  functions::parse_f_value_helper::parse_f_value_helper, type_aliases::f_value_result::FValueResult,
};

pub fn parse_f_int(view: &str) -> FValueResult<i32> {
  let (name, value) = parse_f_value_helper(view);
  if let Some(v) = value {
    let parsed: i32 = v
      .parse()
      .expect("Expected a valid integer value associated with the FInt");
    (name, parsed)
  } else {
    panic!("Expected a value associated with {}", name);
  }
}
