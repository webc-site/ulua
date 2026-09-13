use crate::{
  enums::r#type::Type,
  records::{
    bytecode_builder::BytecodeBuilder,
    constant::{Constant, ConstantValue},
    constant_key::ConstantKey,
  },
};

impl BytecodeBuilder {
  pub fn add_constant_vector(&mut self, x: f32, y: f32, z: f32, w: f32) -> i32 {
    let c = Constant {
      r#type: Type::Vector,
      value: ConstantValue {
        value_vector: [x, y, z, w],
      },
    };

    let mut k = ConstantKey {
      r#type: Type::Vector,
      value: 0,
      extra: 0,
    };

    k.value = x.to_bits() as u64;
    k.value |= (y.to_bits() as u64) << 32;

    k.extra = z.to_bits() as u64;
    k.extra |= (w.to_bits() as u64) << 32;

    self.add_constant(k, c)
  }
}
