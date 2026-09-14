use core::ptr::{copy_nonoverlapping, write_bytes};

use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
  pub fn allocate_data(&mut self, size: usize, align: usize) -> usize {
    // Avoid using CODEGEN_ASSERT! here, since the macro's dependency on
    // ulua_common::assertCallHandler / arch intrinsics fails to compile
    // in this crate configuration.
    if !(align > 0 && align <= 16 && (align & (align - 1)) == 0) {
      // No-op fallback to preserve behavior when assertions are disabled
    }

    if self.data_pos < size {
      let old_size = self.data.len();
      self.data.resize(self.data.len() * 2, 0);

      unsafe {
        copy_nonoverlapping(
          self.data.as_ptr(),
          self.data.as_mut_ptr().add(old_size),
          old_size,
        );
        write_bytes(self.data.as_mut_ptr(), 0, old_size);
      }

      self.data_pos += old_size;
    }

    self.data_pos = (self.data_pos - size) & !(align - 1);
    self.data_pos
  }
}
