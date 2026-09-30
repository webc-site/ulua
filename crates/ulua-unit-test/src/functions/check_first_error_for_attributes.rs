use ulua_ast::records::{location::Location, parse_error::ParseError};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

pub fn check_first_error_for_attributes(
  errors: &[ParseError],
  min_size: usize,
  location: Location,
  message: &str,
) {
  LUAU_ASSERT!(min_size >= 1);

  assert!(errors.len() >= min_size);
  assert_eq!(errors[0].get_location(), &location);
  assert_eq!(errors[0].get_message(), message);
}
