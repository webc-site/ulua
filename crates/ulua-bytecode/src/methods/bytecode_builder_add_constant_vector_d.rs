use crate::{
  enums::r#type::Type,
  records::{
    bytecode_builder::BytecodeBuilder,
    constant::{Constant, ConstantValue},
    constant_key::ConstantKey,
  },
};

impl BytecodeBuilder {
  /// cpp `addConstantVectord`（`BytecodeBuilder.cpp:406-424`）：四个 `f64` 分量
  /// 按位依次落在 `value`/`extra1`/`extra2`/`extra3` 上。
  pub fn add_constant_vector_d(&mut self, x: f64, y: f64, z: f64, w: f64) -> i32 {
    let c = Constant {
      r#type: Type::Vectord,
      value: ConstantValue {
        value_vector_d: [x, y, z, w],
      },
    };

    let k = ConstantKey {
      r#type: Type::Vectord,
      value: x.to_bits(),
      extra: y.to_bits(),
      extra2: z.to_bits(),
      extra3: w.to_bits(),
    };

    self.add_constant(k, c)
  }
}
