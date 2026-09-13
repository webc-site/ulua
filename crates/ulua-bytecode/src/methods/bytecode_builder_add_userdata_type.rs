use crate::records::{bytecode_builder::BytecodeBuilder, userdata_type::UserdataType};

impl BytecodeBuilder {
  pub fn add_userdata_type(&mut self, name: &str) -> u32 {
    let ty = UserdataType {
      name: name.to_string(),
      ..Default::default()
    };

    self.userdata_types.push(ty);
    (self.userdata_types.len() - 1) as u32
  }
}
