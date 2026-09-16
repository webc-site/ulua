use crate::{
  enums::r#type::Type,
  records::{
    bytecode_builder::BytecodeBuilder,
    constant::{Constant, ConstantValue},
    constant_key::ConstantKey,
  },
};

impl BytecodeBuilder {
  pub fn add_constant_boolean(&mut self, value: bool) -> i32 {
    let c = Constant {
      r#type: Type::Boolean,
      value: ConstantValue {
        value_boolean: value,
      },
    };

    let k = ConstantKey {
      r#type: Type::Boolean,
      value: value as u64,
      extra: 0,
    };

    self.add_constant(k, c)
  }
}
