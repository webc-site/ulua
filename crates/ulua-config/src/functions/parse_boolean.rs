use alloc::format;

use crate::type_aliases::error::Error;

pub(crate) fn parse_boolean(result: &mut bool, value: &str) -> Error {
  if value == "true" {
    *result = true;
  } else if value == "false" {
    *result = false;
  } else {
    return Some(format!(
      "Bad setting '{}'.  Valid options are true and false",
      value
    ));
  }

  None
}
