use crate::records::{
  address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
};

impl AssemblyBuilderA64 {
  pub fn log_c_char_register_a_64_register_a_64_address_a_64(
    &mut self,
    opcode: &str,
    dst1: RegisterA64,
    dst2: RegisterA64,
    src: AddressA64,
  ) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_register_a_64(dst1);
    self.text.push(',');
    self.log_register_a_64(dst2);
    self.text.push(',');
    self.log_address_a_64(src);
    self.text.push('\n');
  }
}
