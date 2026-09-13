use crate::{
  enums::r#type::Type,
  records::{
    bytecode_builder::BytecodeBuilder,
    constant::{Constant, ConstantValue},
    constant_key::ConstantKey,
  },
};

impl BytecodeBuilder {
  pub fn add_constant_number(&mut self, value: f64) -> i32 {
    let c = Constant {
      r#type: Type::Number,
      value: ConstantValue {
        value_number: value,
      },
    };

    let k = ConstantKey {
      r#type: Type::Number,
      value: value.to_bits(),
      extra: 0,
    };

    self.add_constant(k, c)
  }
}
