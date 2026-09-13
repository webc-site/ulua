use crate::{
  enums::r#type::Type,
  records::{
    bytecode_builder::BytecodeBuilder,
    class_shape::ClassShape,
    constant::{Constant, ConstantValue},
  },
};

impl BytecodeBuilder {
  pub fn add_class_shape(&mut self, shape: ClassShape) -> i32 {
    let id = self.constants.len() as u32;

    const K_MAX_CONSTANT_COUNT: u32 = 0x007f_ffff;
    if id >= K_MAX_CONSTANT_COUNT {
      return -1;
    }

    let c = Constant {
      r#type: Type::ClassShape,
      value: ConstantValue {
        value_class_shape: self.class_shapes.len() as u32,
      },
    };

    self.class_shapes.push(shape);
    self.constants.push(c);

    id as i32
  }
}
