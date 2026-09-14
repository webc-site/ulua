use core::{
  mem::size_of,
  ptr::{copy_nonoverlapping, write_bytes},
};

use crate::{
  enums::size_x_64::SizeX64,
  functions::writef_32::writef_32,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn f32(&mut self, value: f32) -> OperandX64 {
    let mut as32_bit_key: u32 = 0;
    unsafe {
      copy_nonoverlapping(
        &value as *const f32 as *const u8,
        &mut as32_bit_key as *mut u32 as *mut u8,
        size_of::<f32>(),
      );
    }

    if as32_bit_key != !0u32
      && let Some(prev) = self.const_cache_32.find(&as32_bit_key)
    {
      return OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
        SizeX64::Dword,
        RegisterX64::NOREG,
        1,
        RegisterX64::RIP,
        *prev,
      );
    }

    let pos = {
      if self.data_pos < 4 {
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

      self.data_pos = (self.data_pos - 4) & !(4 - 1);
      self.data_pos
    };

    unsafe {
      writef_32(self.data.as_mut_ptr().add(pos), value);
    }
    let offset = (pos as i32) - (self.data.len() as i32);

    if as32_bit_key != !0u32 {
      *self.const_cache_32.get_or_insert(as32_bit_key) = offset;
    }

    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
      SizeX64::Dword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RIP,
      offset,
    )
  }
}
