use crate::records::function_bytecode_summary::FunctionBytecodeSummary;

impl FunctionBytecodeSummary {
  pub fn get_name(&self) -> &str {
    &self.name
  }
}
