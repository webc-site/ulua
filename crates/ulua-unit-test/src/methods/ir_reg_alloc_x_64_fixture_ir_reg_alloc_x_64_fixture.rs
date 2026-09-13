//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Method:Luau.UnitTest:tests/IrRegAllocX64.test.cpp:12:ir_reg_alloc_x_64_fixture_ir_reg_alloc_x_64_fixture`
//! Source: `tests/IrRegAllocX64.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/IrRegAllocX64.test.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
//! - incoming:
//!   - declares <- source_file tests/IrRegAllocX64.test.cpp
//!   - type_ref <- record IrRegAllocX64Fixture (tests/IrRegAllocX64.test.cpp)
//! - outgoing:
//!   - type_ref -> record IrRegAllocX64Fixture (tests/IrRegAllocX64.test.cpp)
//!   - translates_to -> rust_item IrRegAllocX64Fixture::IrRegAllocX64Fixture

use alloc::boxed::Box;
use core::ptr::null_mut;

use ulua_code_gen::{
  enums::abix_64::ABIX64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_function::IrFunction,
    ir_reg_alloc_x_64::IrRegAllocX64,
  },
};

use crate::records::ir_reg_alloc_x_64_fixture::IrRegAllocX64Fixture;
impl IrRegAllocX64Fixture {
  pub fn new() -> Self {
    let mut build = Box::new(AssemblyBuilderX64::assembly_builder_x_64_bool_abix_64_i32(
      true,
      ABIX64::Windows,
      0,
    ));
    let mut function = Box::new(IrFunction::default());
    let regs = IrRegAllocX64::ir_reg_alloc_x_64_ir_reg_alloc_x_64(
      build.as_mut(),
      function.as_mut(),
      null_mut(),
    );

    Self {
      build,
      function,
      regs,
    }
  }
}

pub fn ir_reg_alloc_x_64_fixture_ir_reg_alloc_x_64_fixture() -> IrRegAllocX64Fixture {
  IrRegAllocX64Fixture::new()
}

impl Default for IrRegAllocX64Fixture {
  fn default() -> Self {
    Self::new()
  }
}
