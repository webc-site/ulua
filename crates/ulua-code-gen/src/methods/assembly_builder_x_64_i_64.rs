use crate::{
  enums::size_x_64::SizeX64,
  functions::writeu_64::writeu_64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn i64(&mut self, value: i64) -> OperandX64 {
    let as_64_bit_key = value as u64;

    if as_64_bit_key != !0u64
      && let Some(prev) = self.const_cache_64.find(&as_64_bit_key)
    {
      return OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
        SizeX64::Qword,
        RegisterX64::NOREG,
        1,
        RegisterX64::RIP,
        *prev,
      );
    }

    let pos = self.allocate_data(8, 8);

    unsafe {
      writeu_64(self.data.as_mut_ptr().add(pos), as_64_bit_key);
    }
    let offset = (pos as isize - self.data.len() as isize) as i32;

    if as_64_bit_key != !0u64 {
      self.const_cache_64.try_insert(as_64_bit_key, offset);
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
