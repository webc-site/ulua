use alloc::{string::String, vec::Vec};

use ulua_bytecode::{
  functions::{
    from_function_bytecode::from_function_bytecode,
    to_function_bytecode_bytecode_graph::to_function_bytecode_bytecode_builder_comp_time_bc_function,
  },
  records::bytecode_builder::BytecodeBuilder,
};

use crate::{
  functions::parse_and_compile::parse_and_compile,
  records::bytecode_compiler_fixture::BytecodeCompilerFixture,
};

impl BytecodeCompilerFixture {
  /// cpp `BytecodeCompilerFixture::getRoundtripFunctionBytecode`
  /// （tests/BytecodeCompiler.test.cpp:160-183）：编译源码 → 逐函数
  /// 反序列化/重序列化进带 dump 的 reserializer（cpp 侧函数按子先父后
  /// 编号，NEWCLOSURE 依赖先前序列化过的函数）→ 返回第 `function_id`
  /// 个函数按 `dump_flags` 反汇编的文本。
  pub fn get_roundtrip_function_bytecode(
    &mut self,
    src: &str,
    dump_flags: u32,
    optimization_level: i32,
    function_id: u32,
  ) -> String {
    let bcb = parse_and_compile(src, optimization_level).expect("expected compiled bytecode");

    let mut reserializer = BytecodeBuilder::new(None);
    reserializer.set_dump_flags(dump_flags);

    let strings = self.extract_string_table(&bcb);
    let table: Vec<&[u8]> = strings.iter().map(Vec::as_slice).collect();

    for fi in 0..=function_id {
      let data = bcb.get_function_data(fi);
      let mut fn_ =
        from_function_bytecode(&data, &table).expect("expected function bytecode to deserialize");
      to_function_bytecode_bytecode_builder_comp_time_bc_function(&mut reserializer, &mut fn_);
      reserializer.clear_strings();
    }

    reserializer.dump_function(function_id)
  }
}
