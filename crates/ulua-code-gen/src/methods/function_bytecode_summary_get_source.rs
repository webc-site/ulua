use crate::records::function_bytecode_summary::FunctionBytecodeSummary;

impl FunctionBytecodeSummary {
  pub fn get_source(&self) -> &str {
    &self.source
  }
}
