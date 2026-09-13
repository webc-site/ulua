use core::fmt::{Arguments, write};

use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn log_append(&mut self, args: Arguments<'_>) {
    let _ = write(&mut self.text, args);
  }
}
