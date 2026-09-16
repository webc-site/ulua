extern crate alloc;

use alloc::{string::String, vec::Vec};

use ulua_common::enums::luau_opcode::LuauOpcode;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FunctionBytecodeSummary {
  pub source: String,
  pub name: String,
  pub line: i32,
  pub nesting_limit: u32,
  pub counts: Vec<Vec<u32>>,
}

impl FunctionBytecodeSummary {
  pub const LOP__COUNT: u32 = LuauOpcode::LOP__COUNT as u32;
}
