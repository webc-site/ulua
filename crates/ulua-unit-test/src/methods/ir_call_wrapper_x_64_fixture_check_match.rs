use alloc::string::String;

use crate::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;
impl IrCallWrapperX64Fixture {
  pub fn check_match(&mut self, expected: String) {
    self.regs.assert_all_free();
    self.build.finalize();
    assert_eq!(alloc::format!("\n{}", self.build.text), expected);
  }
}

pub fn ir_call_wrapper_x_64_fixture_check_match(
  fixture: &mut IrCallWrapperX64Fixture,
  expected: String,
) {
  fixture.check_match(expected)
}
