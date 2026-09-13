use crate::records::function_bytecode_summary::FunctionBytecodeSummary;

impl FunctionBytecodeSummary {
  pub fn get_counts(&self, nesting: u32) -> &Vec<u32> {
    debug_assert!(nesting <= self.get_nesting_limit());
    &self.counts[nesting as usize]
  }
}
