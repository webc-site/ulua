use crate::{
  functions::bad_setting::{OPT_BOOL, SETTING_FALSE, SETTING_TRUE, bad_setting},
  type_aliases::error::Error,
};

pub(crate) fn parse_boolean(result: &mut bool, value: &str) -> Error {
  match value {
    SETTING_TRUE => *result = true,
    SETTING_FALSE => *result = false,
    _ => return Some(bad_setting(value, OPT_BOOL)),
  }

  None
}
