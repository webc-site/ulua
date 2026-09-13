use crate::{
  enums::r#type::Type,
  records::{
    bytecode_builder::BytecodeBuilder,
    constant::{Constant, ConstantValue},
    constant_key::ConstantKey,
  },
};

impl BytecodeBuilder {
  pub fn add_import(&mut self, iid: u32) -> i32 {
    let c = Constant {
      r#type: Type::Import,
      value: ConstantValue { value_import: iid },
    };

    let k = ConstantKey {
      r#type: Type::Import,
      value: iid as u64,
      extra: 0,
    };

    self.add_constant(k, c)
  }
}
