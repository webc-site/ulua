use crate::{
  enums::address_kind_a_64::AddressKindA64,
  records::{address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64},
};
impl AssemblyBuilderA64 {
  pub fn log_address_a_64(&mut self, addr: AddressA64) {
    self.text.push(b'[' as char);
    match addr.kind {
      AddressKindA64::Reg => {
        self.log_register_a_64(addr.base);
        self.text.push(b',' as char);
        self.log_register_a_64(addr.offset);
      }
      AddressKindA64::Imm => {
        self.log_register_a_64(addr.base);
        if addr.data != 0 {
          self.log_append(format_args!(",#{}", addr.data));
        }
      }
      AddressKindA64::Pre => {
        self.log_register_a_64(addr.base);
        if addr.data != 0 {
          self.log_append(format_args!(",#{}", addr.data));
        }
        self.text.push(b']' as char);
        self.text.push(b'!' as char);
        return;
      }
      AddressKindA64::Post => {
        self.log_register_a_64(addr.base);
        self.text.push(b']' as char);
        self.text.push(b'!' as char);
        if addr.data != 0 {
          self.log_append(format_args!(",#{}", addr.data));
        }
        return;
      }
    }
    self.text.push(b']' as char);
  }
}
