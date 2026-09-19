use crate::{
  enums::size_x_64::SizeX64,
  functions::writef_32::writef_32,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};
impl AssemblyBuilderX64 {
  pub fn f32x4(&mut self, x: f32, y: f32, z: f32, w: f32) -> OperandX64 {
    let pos = self.allocate_data(16, 16);

    unsafe {
      let data_pos = self.data.as_mut_ptr().add(pos);
      let _ = writef_32(data_pos, x);
      let _ = writef_32(data_pos.add(4), y);
      let _ = writef_32(data_pos.add(8), z);
      let _ = writef_32(data_pos.add(12), w);
    }

    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
      SizeX64::Xmmword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RIP,
      pos as i32 - self.data.len() as i32,
    )
  }
}
