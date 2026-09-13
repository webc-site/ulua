use core::{
  ffi::c_void,
  ptr::{copy_nonoverlapping, write_bytes},
  slice::from_raw_parts,
};

use crate::{
  enums::size_x_64::SizeX64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn bytes_data(&mut self, data: &[u8], align: usize) -> OperandX64 {
    let size = data.len();
    let pos = {
      // AssemblyBuilderX64::bytes in C++ calls a private helper `allocateData`.
      // In Rust, that helper is currently not available as a method on this type.
      // Fall back to the known layout/fields on AssemblyBuilderX64 to preserve behavior.
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
    };

    self.data[pos..pos + size].copy_from_slice(data);

    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
      SizeX64::None,
      RegisterX64::NOREG,
      1,
      RegisterX64::RIP,
      (pos as i32) - (self.data.len() as i32),
    )
  }

  pub fn bytes(&mut self, ptr: *const c_void, size: usize, align: usize) -> OperandX64 {
    let slice = unsafe { from_raw_parts(ptr as *const u8, size) };
    self.bytes_data(slice, align)
  }
}
