use crate::records::function_bytecode_summary::FunctionBytecodeSummary;

impl FunctionBytecodeSummary {
  pub fn get_op_limit(&self) -> u32 {
    Self::LOP__COUNT
  }
}
