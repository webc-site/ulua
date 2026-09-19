use crate::{
  enums::size_x_64::SizeX64,
  functions::writef_64::writef_64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn f64x2(&mut self, x: f64, y: f64) -> OperandX64 {
    let pos = self.allocate_data(16, 16);

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
