use alloc::vec::Vec;

use ulua_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;

use crate::{
  functions::bytecode_as_array_assembly_builder_x_64_test::bytecode_as_array_vector_u8,
  records::assembly_builder_x_64_fixture::AssemblyBuilderX64Fixture,
};

impl AssemblyBuilderX64Fixture {
  pub fn check(&self, f: fn(&mut AssemblyBuilderX64), code: Vec<u8>, data: Vec<u8>) -> bool {
    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

    f(&mut build);

    build.finalize();

    if build.code != code {
      println!(
        "Expected code: {}\nReceived code: {}",
        bytecode_as_array_vector_u8(&code),
        bytecode_as_array_vector_u8(&build.code)
      );
      return false;
    }

    if build.data != data {
      println!(
        "Expected data: {}\nReceived data: {}",
        bytecode_as_array_vector_u8(&data),
        bytecode_as_array_vector_u8(&build.data)
      );
      return false;
    }

    true
  }
}
