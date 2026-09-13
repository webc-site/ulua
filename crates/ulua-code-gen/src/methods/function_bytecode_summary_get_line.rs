use crate::records::function_bytecode_summary::FunctionBytecodeSummary;
impl FunctionBytecodeSummary {
  pub fn get_line(&self) -> i32 {
    self.line
  }
}
