use alloc::fmt::format;
use core::fmt::Arguments;
use std::panic::panic_any;

use crate::records::{location::Location, parse_error::ParseError};

impl ParseError {
  pub fn raise(location: Location, args: Arguments<'_>) -> ! {
    let message = format(args);

    // C++ `throw ParseError(...)`. The panic payload MUST be the ParseError
    // object itself: `Parser::parse` catches the unwind and recovers the error
    // via `downcast_ref::<ParseError>()`. Panicking with a formatted String
    // (panic!("{}", ...)) made that downcast fail, so the panic escaped the
    // parse boundary uncaught (every recursion-/error-limit test crashed).
    panic_any(ParseError::new(location, message));
  }
}
