use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn finalize(&mut self) -> bool {
    let code_size = (self.code_pos as usize).wrapping_sub(self.code.as_ptr() as usize);
    self.code.resize(code_size, 0);

    for fixup in self.pending_labels.iter().copied() {
      let location = self.label_locations[(fixup.id - 1) as usize];
      if !(location != !0u32) {
        ulua_common::LUAU_DEBUGBREAK!();
      }
      let value = location.wrapping_sub(fixup.location.wrapping_add(4));
      let offset = fixup.location as usize;
      self.code[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    let data_size = self.data.len() - self.data_pos;

    if data_size > 0 {
      self
        .data
        .copy_within(self.data_pos..self.data_pos + data_size, 0);
    }

    self.data.resize(data_size, 0);

    self.finalized = true;

    true
  }
}
