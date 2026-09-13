use alloc::{string::String, vec::Vec};
use core::ptr::null_mut;

use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
  pub fn assembly_builder_a_64_assembly_builder_a_64(&mut self, log_text: bool, features: u32) {
    self.log_text = log_text;
    self.features = features;

    self.data.resize(4096, 0);
    self.data_pos = self.data.len(); // data is filled backwards

    self.code.resize(1024, 0);
    self.code_pos = self.code.as_mut_ptr();
    self.code_end = unsafe { self.code_pos.add(self.code.len()) };
  }

  /// C++ `AssemblyBuilderA64(bool logText, unsigned int features = 0)` — the
  /// real constructor. The bare `assembly_builder_a_64_assembly_builder_a_64`
  /// above is the `&mut self` init body (the ctor was translated as a method
  /// that returns `()`); this builds, initializes, and RETURNS the value, so
  /// callers/tests can write `let build = AssemblyBuilderA64::...(false, 0);`.
  pub fn assembly_builder_a_64_bool_i32(log_text: bool, features: u32) -> Self {
    let mut build = AssemblyBuilderA64 {
      data: Vec::new(),
      code: Vec::new(),
      text: String::new(),
      log_text: false,
      features: 0,
      next_label: 1,
      pending_labels: Vec::new(),
      label_locations: Vec::new(),
      finalized: false,
      overflowed: false,
      data_pos: 0,
      code_pos: null_mut(),
      code_end: null_mut(),
    };
    build.assembly_builder_a_64_assembly_builder_a_64(log_text, features);
    build
  }
}
