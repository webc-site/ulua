use alloc::string::String;

use crate::records::ir_reg_alloc_x_64_fixture::IrRegAllocX64Fixture;

impl IrRegAllocX64Fixture {
  pub fn check_match(&mut self, expected: String) {
    self.build.finalize();
    assert_eq!(alloc::format!("\n{}", self.build.text), expected);
  }
}

pub fn ir_reg_alloc_x_64_fixture_check_match(fixture: &mut IrRegAllocX64Fixture, expected: String) {
  fixture.check_match(expected)
}
