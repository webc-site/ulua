use alloc::vec::Vec;

use ulua_bytecode::{
  functions::from_function_bytecode::from_function_bytecode, records::bc_function::BcFunction,
};

use crate::{
  functions::parse_and_compile::parse_and_compile,
  records::bytecode_inliner_fixture::BytecodeInlinerFixture,
};

impl BytecodeInlinerFixture {
  /// cpp `buildGraphs`：把整段源码编译成字节码，再把每个函数反序列化为 SSA 图。
  /// 与 `buildBytecode` 不同，这里不指定 inlinee/caller 身份，用于直接检查图结构。
  pub fn build_graphs(&mut self, src: &str, optimization_level: i32) -> Vec<BcFunction> {
    let bcb = parse_and_compile(src, optimization_level).expect("expected compiled source");
    let strings = self.extract_string_table(&bcb);
    self.strings = strings;
    let table: Vec<&[u8]> = self.strings.iter().map(Vec::as_slice).collect();

    (0..bcb.get_function_count())
      .map(|fi| {
        let bytecode = bcb.get_function_data(fi);
        from_function_bytecode(&bytecode, &table)
          .expect("expected function bytecode to deserialize")
      })
      .collect()
  }
}
