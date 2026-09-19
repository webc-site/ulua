use core::{ffi::c_void, slice::from_raw_parts};

use crate::{
  enums::size_x_64::SizeX64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn bytes_data(&mut self, data: &[u8], align: usize) -> OperandX64 {
    let size = data.len();
    let pos = self.allocate_data(size, align);

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
