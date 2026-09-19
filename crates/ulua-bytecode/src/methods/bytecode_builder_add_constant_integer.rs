use crate::{
  enums::r#type::Type,
  records::{
    bytecode_builder::BytecodeBuilder,
    constant::{Constant, ConstantValue},
    constant_key::ConstantKey,
  },
};

impl BytecodeBuilder {
  pub fn add_constant_integer(&mut self, value: i64) -> i32 {
    let c = Constant {
      r#type: Type::Integer,
      value: ConstantValue {
        value_integer64: value,
      },
    };

    let k = ConstantKey {
      r#type: Type::Integer,
      value: value as u64,
      extra: 0,
    };

    self.add_constant(k, c)
  }
}
