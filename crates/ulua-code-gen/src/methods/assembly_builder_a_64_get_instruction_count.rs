use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
  pub fn get_instruction_count(&self) -> u32 {
    self.get_code_size()
  }
}
