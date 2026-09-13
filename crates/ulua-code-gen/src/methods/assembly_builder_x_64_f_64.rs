use core::{
  mem::size_of,
  ptr::{copy_nonoverlapping, write_bytes},
};

use crate::{
  enums::size_x_64::SizeX64,
  functions::writef_64::writef_64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn f64(&mut self, value: f64) -> OperandX64 {
    let mut as64_bit_key: u64 = 0;
    unsafe {
      copy_nonoverlapping(
        &value as *const f64 as *const u8,
        &mut as64_bit_key as *mut u64 as *mut u8,
        size_of::<f64>(),
      );
    }

    if as64_bit_key != !0u64
      && let Some(prev) = self.const_cache_64.find(&as64_bit_key)
    {
      return OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
        SizeX64::Qword,
        RegisterX64::NOREG,
        1,
        RegisterX64::RIP,
        *prev,
      );
    }

    let pos = {
      if self.data_pos < 8 {
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

      self.data_pos = (self.data_pos - 8) & !(8 - 1);
      self.data_pos
    };

    unsafe {
      writef_64(self.data.as_mut_ptr().add(pos), value);
    }
    let offset = (pos as i32) - (self.data.len() as i32);

    if as64_bit_key != !0u64 {
      *self.const_cache_64.get_or_insert(as64_bit_key) = offset;
    }

    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RIP,
      offset,
    )
  }
}
