//! @interface-stub
use alloc::vec::Vec;

use ulua_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;

use crate::records::assembly_builder_a_64_fixture::AssemblyBuilderA64Fixture;
impl AssemblyBuilderA64Fixture {
  pub fn check(
    &mut self,
    f: fn(&mut AssemblyBuilderA64),
    code: Vec<u32>,
    data: Vec<u8>,
    features: u32,
  ) -> bool {
    // C++ AssemblyBuilderA64Fixture::check: fresh builder, run f, finalize,
    // compare code and data.
    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, features);
    f(&mut build);
    build.finalize();
    build.code == code && build.data == data
  }
}
