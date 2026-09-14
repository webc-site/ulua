use alloc::string::String;

use crate::{
  functions::write_var_int::write_var_int,
  records::{bytecode_builder::BytecodeBuilder, class_shape::ClassShape},
};

impl BytecodeBuilder {
  pub fn write_class_shape(&self, ss: &mut String, cs: &ClassShape) {
    write_var_int(ss, cs.class_name as u64);
    write_var_int(ss, cs.property_names.len() as u64);
    write_var_int(ss, cs.method_names.len() as u64);

    for &prop_name in &cs.property_names {
      write_var_int(ss, prop_name as u64);
    }

    for &method_name in &cs.method_names {
      write_var_int(ss, method_name as u64);
    }
  }
}
