use crate::{
  enums::r#type::Type,
  records::{
    bytecode_builder::BytecodeBuilder,
    constant::{Constant, ConstantValue},
    constant_key::ConstantKey,
  },
};

impl BytecodeBuilder {
  pub fn add_constant_closure(&mut self, fid: u32) -> i32 {
    let c = Constant {
      r#type: Type::Closure,
      value: ConstantValue { value_closure: fid },
    };

    let k = ConstantKey {
      r#type: Type::Closure,
      value: fid as u64,
      extra: 0,
    };

    self.add_constant(k, c)
  }
}
