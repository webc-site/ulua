use crate::records::function_bytecode_summary::FunctionBytecodeSummary;

impl FunctionBytecodeSummary {
  pub fn get_nesting_limit(&self) -> u32 {
    self.nesting_limit
  }
}
