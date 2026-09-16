use crate::common::{
  functions::parse_f_value_helper::parse_f_value_helper, type_aliases::f_value_result::FValueResult,
};

pub fn parse_f_flag(view: &str) -> FValueResult<bool> {
  let (name, value) = parse_f_value_helper(view);
  let state = match &value {
    Some(val) => val == "true",
    None => true,
  };

  if let Some(ref val) = value
    && val != "true"
    && val != "false"
  {
    eprintln!(
      "Ignored '{}' because '{}' is not a valid flag state",
      name, val
    );
  }

  (name, state)
}
