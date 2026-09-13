use alloc::string::String;
use core::ptr::null_mut;

use ulua_code_gen::functions::get_assembly_from_ir::get_assembly_from_ir;

use crate::{
  functions::{
    normalize_state_offsets::normalize_state_offsets,
    strip_lines_containing::strip_lines_containing,
  },
  records::ir_assembly_fixture::IrAssemblyFixture,
};
impl IrAssemblyFixture {
  pub fn lower(&mut self) -> String {
    let mut text =
      unsafe { get_assembly_from_ir(&mut self.build, self.options.clone(), null_mut()) };
    strip_lines_containing(&mut text, "; skipping ");
    normalize_state_offsets(&mut text);
    text
  }
}
