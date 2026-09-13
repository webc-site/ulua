use crate::{
  enums::size_x_64::SizeX64,
  macros::{rex_b::rex_b, rex_force::rex_force, rex_w_bit::REX_W_BIT},
  records::{assembly_builder_x_64::AssemblyBuilderX64, register_x_64::RegisterX64},
};

impl AssemblyBuilderX64 {
  pub fn place_rex_register_x_64(&mut self, op: RegisterX64) {
    let code: u8 = REX_W_BIT!(op.size() == SizeX64::Qword) | rex_b(op) | rex_force(op);

    if code != 0 {
      self.place(code | 0x40);
    }
  }
}
