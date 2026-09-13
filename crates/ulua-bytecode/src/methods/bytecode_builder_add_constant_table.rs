use crate::{
  enums::r#type::Type,
  records::{
    bytecode_builder::BytecodeBuilder,
    constant::{Constant, ConstantValue},
    table_shape::TableShape,
  },
};

impl BytecodeBuilder {
  pub fn add_constant_table(&mut self, shape: &TableShape) -> i32 {
    if let Some(cache) = self.table_shape_map.find(shape) {
      return *cache;
    }

    let id = self.constants.len() as u32;

    const K_MAX_CONSTANT_COUNT: u32 = 0x007f_ffff;
    if id >= K_MAX_CONSTANT_COUNT {
      return -1;
    }

    let value = Constant {
      r#type: Type::Table,
      value: ConstantValue {
        // C++ `value.valueTable = uint32_t(tableShapes.size())`: value_table
        // indexes table_shapes, NOT constants. The previous `id` (= constants
        // length) over-indexed table_shapes and panicked in write_function.
        value_table: self.table_shapes.len() as u32,
      },
    };

    self.table_shape_map.try_insert(*shape, id as i32);
    self.table_shapes.push(*shape);
    self.constants.push(value);

    id as i32
  }
}
