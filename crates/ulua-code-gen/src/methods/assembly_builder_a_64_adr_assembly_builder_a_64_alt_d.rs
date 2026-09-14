use crate::{
  enums::kind::Kind,
  functions::writef_64::writef_64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn adr_register_a_64_f64(&mut self, dst: RegisterA64, value: f64) {
    let pos = self.allocate_data(8, 8);
    let location = self.get_code_size();

    unsafe {
      let p = self.data.as_mut_ptr().add(pos);
      writef_64(p, value);
    }

    self.place_adr_c_char_register_a_64_u8("adr", dst, 0b10000);

    let patch_value = -(location as i32) - (((self.data.len() - pos) / 4) as i32);
    self.patch_offset(location, patch_value, Kind::IMM19);
  }
}
