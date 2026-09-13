use crate::{
  enums::r#type::Type,
  records::{
    bytecode_builder::BytecodeBuilder,
    constant::{Constant, ConstantValue},
    constant_key::ConstantKey,
  },
};

impl BytecodeBuilder {
  pub fn add_constant_nil(&mut self) -> i32 {
    let c = Constant {
      r#type: Type::Nil,
      value: ConstantValue {
        value_boolean: false,
      },
    };

    let k = ConstantKey {
      r#type: Type::Nil,
      value: 0,
      extra: 0,
    };

    self.add_constant(k, c)
  }
}
