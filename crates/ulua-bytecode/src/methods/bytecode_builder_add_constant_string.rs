use crate::{
  enums::r#type::Type,
  records::{
    bytecode_builder::BytecodeBuilder,
    constant::{Constant, ConstantValue},
    constant_key::ConstantKey,
    string_ref::StringRef,
  },
};

impl BytecodeBuilder {
  pub fn add_constant_string(&mut self, value: impl Into<StringRef>) -> i32 {
    let index = self.add_string_table_entry(value.into());

    let c = Constant {
      r#type: Type::String,
      value: ConstantValue {
        value_string: index,
      },
    };

    let k = ConstantKey {
      r#type: Type::String,
      value: index as u64,
      extra: 0,
    };

    self.add_constant(k, c)
  }
}
