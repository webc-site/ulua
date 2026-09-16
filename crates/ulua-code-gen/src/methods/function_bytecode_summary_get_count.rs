macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    assert!($expr);
  };
}

use crate::records::function_bytecode_summary::FunctionBytecodeSummary;

impl FunctionBytecodeSummary {
  pub fn get_count(&self, nesting: u32, op: u8) -> u32 {
    CODEGEN_ASSERT!(nesting <= self.get_nesting_limit());
    CODEGEN_ASSERT!((op as u32) < self.get_op_limit());
    self.counts[nesting as usize][op as usize]
  }
}
