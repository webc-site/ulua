use crate::{
  enums::size_x_64::SizeX64,
  functions::writeu_32::writeu_32,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn i32(&mut self, value: i32) -> OperandX64 {
    let as_32_bit_key = value as u32;

    if as_32_bit_key != !0u32
      && let Some(prev) = self.const_cache_32.find(&as_32_bit_key)
    {
      return OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
        SizeX64::Dword,
        RegisterX64::NOREG,
        1,
        RegisterX64::RIP,
        *prev,
      );
    }

    let pos = self.allocate_data(4, 4);

    unsafe {
      writeu_32(self.data.as_mut_ptr().add(pos), value as u32);
    }
    let offset = (pos as isize - self.data.len() as isize) as i32;

    if as_32_bit_key != !0u32 {
      self.const_cache_32.try_insert(as_32_bit_key, offset);
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
