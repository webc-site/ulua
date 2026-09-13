use core::ptr::{copy_nonoverlapping, write_bytes};

use crate::{
  enums::size_x_64::SizeX64,
  functions::writef_64::writef_64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn f64x2(&mut self, x: f64, y: f64) -> OperandX64 {
    let pos = {
      if self.data_pos < 16 {
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

      self.data_pos = (self.data_pos - 16) & !(16 - 1);
      self.data_pos
    };

    unsafe {
      writef_64(self.data.as_mut_ptr().add(pos), x);
      writef_64(self.data.as_mut_ptr().add(pos + 8), y);
    }

    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
      SizeX64::Xmmword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RIP,
      (pos as i32) - (self.data.len() as i32),
    )
  }
}
