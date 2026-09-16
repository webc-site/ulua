use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn get_instruction_count(&self) -> u32 {
    self.instruction_count
  }
}
