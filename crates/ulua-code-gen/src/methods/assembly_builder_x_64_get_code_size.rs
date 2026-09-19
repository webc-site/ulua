use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn get_code_size(&self) -> u32 {
    let code_pos = self.code_pos as usize;
    let code_data = self.code.as_ptr() as usize;
    let code_size = code_pos.wrapping_sub(code_data);
    u32::try_from(code_size).unwrap_or(u32::MAX)
  }
}
