use crate::{
  functions::parse_and_compile::parse_and_compile,
  records::{bytecode_inliner_fixture::BytecodeInlinerFixture, bytecode_res::BytecodeRes},
};

impl BytecodeInlinerFixture {
  pub fn get_function_bytecode(
    &mut self,
    src: &str,
    optimization_level: i32,
  ) -> Option<BytecodeRes> {
    let bcb = parse_and_compile(src, optimization_level)?;
    Some(BytecodeRes {
      inlinee_bytecode: bcb.get_function_data(0),
      caller_bytecode: bcb.get_function_data(1),
      string_table: self.extract_string_table(&bcb),
    })
  }
}
