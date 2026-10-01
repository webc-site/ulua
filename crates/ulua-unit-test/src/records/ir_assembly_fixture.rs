//! Source: `tests/IrAssembly.test.cpp`

use alloc::boxed::Box;

use ulua_code_gen::records::{
  assembly_options::AssemblyOptions, host_ir_hooks::HostIrHooks, ir_builder::IrBuilder,
};

#[derive(Debug)]
pub struct IrAssemblyFixture {
  pub hooks: Box<HostIrHooks>,
  pub build: IrBuilder,
  pub options: AssemblyOptions,
}

impl IrAssemblyFixture {
  pub const TNIL: u8 = 0;
  pub const TBOOLEAN: u8 = 1;
  pub const TNUMBER: u8 = 3;
  pub const TINTEGER: u8 = 4;
  pub const TVECTOR: u8 = 5;
  pub const TSTRING: u8 = 6;
  pub const TTABLE: u8 = 7;
  pub const TFUNCTION: u8 = 8;
  pub const TUSERDATA: u8 = 9;
}
