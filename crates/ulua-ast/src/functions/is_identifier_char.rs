use crate::functions::{
  is_digit_pretty_printer::is_digit, is_identifier_start_char::is_identifier_start_char,
};

pub fn is_identifier_char(c: char) -> bool {
  is_identifier_start_char(c) || is_digit(c)
}
