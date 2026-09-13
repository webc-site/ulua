use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
impl AssemblyBuilderA64 {
  pub fn get_code_size(&self) -> u32 {
    // C++: uint32_t(codePos - code.data()) where code is std::vector<uint32_t>, so the
    // pointer subtraction yields an ELEMENT (word) count, not a byte count.
    let code_pos = self.code_pos as *const u32;
    let code_data = self.code.as_ptr();
    let count = unsafe { code_pos.offset_from(code_data) };
    u32::try_from(count).unwrap_or(u32::MAX)
  }
}
