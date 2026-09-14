use crate::records::{
  bytecode_builder::{BytecodeBuilder, K_MAX_CONSTANT_COUNT},
  constant::Constant,
  constant_key::ConstantKey,
};

impl BytecodeBuilder {
  pub fn add_constant(&mut self, key: ConstantKey, value: Constant) -> i32 {
    if let Some(cache) = self.constant_map.find(&key) {
      return *cache;
    }

    let id = self.constants.len() as u32;

    if id >= K_MAX_CONSTANT_COUNT {
      return -1;
    }

    self.constant_map.try_insert(key, id as i32);
    self.constants.push(value);

    id as i32
  }
}
