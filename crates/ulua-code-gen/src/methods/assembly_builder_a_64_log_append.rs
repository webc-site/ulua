use core::fmt::{Arguments, write};

use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
  pub fn log_append(&mut self, args: Arguments<'_>) {
    let _ = write(&mut self.text, args);
  }
}
