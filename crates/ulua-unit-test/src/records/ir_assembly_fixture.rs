//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/IrAssembly.test.cpp:74:ir_assembly_fixture`
//! Source: `tests/IrAssembly.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/IrAssembly.test.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
//!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
//!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/IrAssembly.test.cpp
//!   - type_ref <- method IrAssemblyFixture::IrAssemblyFixture (tests/IrAssembly.test.cpp)
//!   - type_ref <- method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//! - outgoing:
//!   - type_ref -> method IrAssemblyFixture::IrAssemblyFixture (tests/IrAssembly.test.cpp)
//!   - type_ref -> record HostIrHooks (CodeGen/include/Luau/CodeGenOptions.h)
//!   - type_ref -> record IrBuilder (CodeGen/include/Luau/IrBuilder.h)
//!   - type_ref -> record AssemblyOptions (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item IrAssemblyFixture

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
  pub const TBUFFER: u8 = 11;
}
