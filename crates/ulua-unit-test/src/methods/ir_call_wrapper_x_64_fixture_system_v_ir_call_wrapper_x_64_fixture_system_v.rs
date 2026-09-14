use ulua_code_gen::enums::abix_64::ABIX64;

use crate::records::{
  ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture,
  ir_call_wrapper_x_64_fixture_system_v::IrCallWrapperX64FixtureSystemV,
};
impl IrCallWrapperX64FixtureSystemV {
  pub fn new() -> Self {
    Self {
      base: IrCallWrapperX64Fixture::new(ABIX64::SystemV),
    }
  }
}

impl Default for IrCallWrapperX64FixtureSystemV {
  fn default() -> Self {
    Self::new()
  }
}
