use core::fmt::Arguments;

use crate::records::{location::Location, parser::Parser};

impl Parser {
  pub fn report_location_c_char_item(&mut self, location: Location, format: Arguments<'_>) {
    self.report(location, format);
  }
}
