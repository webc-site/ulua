use core::mem::take;

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT, records::assembly_builder_a_64::AssemblyBuilderA64,
};

impl AssemblyBuilderA64 {
  pub fn finalize(&mut self) -> bool {
    let code_pos_ptr = self.code_pos as *const u32;
    let code_data_ptr = self.code.as_ptr();
    let code_len = unsafe { code_pos_ptr.offset_from(code_data_ptr) as usize };
    self.code.resize(code_len, 0);

    let pending_labels = take(&mut self.pending_labels);
    for fixup in pending_labels {
      let label = fixup.label();
      CODEGEN_ASSERT!(self.label_locations[label as usize - 1] != !0u32);
      let value = (self.label_locations[label as usize - 1] as i32) - (fixup.location as i32);

      self.patch_offset(fixup.location, value, fixup.kind());
    }

    let data_size = self.data.len() - self.data_pos;

    if data_size > 0 {
      self.data.copy_within(self.data_pos.., 0);
    }

    self.data.resize(data_size, 0);

    self.finalized = true;

    !self.overflowed
  }
}
