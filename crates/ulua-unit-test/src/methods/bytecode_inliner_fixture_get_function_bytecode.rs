use alloc::vec::Vec;

use ulua_bytecode::functions::from_function_bytecode::from_function_bytecode;

use crate::{
  functions::parse_and_compile::parse_and_compile,
  records::{bytecode_inliner_fixture::BytecodeInlinerFixture, bytecode_res::BytecodeRes},
};

impl BytecodeInlinerFixture {
  /// cpp `getFunctionBytecode` 取 `funcCount-3/-2` 两个固定槽位；当 target 含嵌套
  /// 闭包时编译顺序变成 inlinee→内嵌闭包→caller→main，固定下标会拿错函数。
  /// 这里按 debugname 扫描函数表定位 `inlinee` / `caller`，对既有用例等价，
  /// 并让「target 含 protos 的内联」用例也能复用本 fixture。
  pub fn get_function_bytecode(
    &mut self,
    src: &str,
    optimization_level: i32,
  ) -> Option<BytecodeRes> {
    let bcb = parse_and_compile(src, optimization_level)?;
    let strings = self.extract_string_table(&bcb);
    let table: Vec<&[u8]> = strings.iter().map(Vec::as_slice).collect();

    let mut inlinee_bytecode: Option<Vec<u8>> = None;
    let mut caller_bytecode: Option<Vec<u8>> = None;
    for fid in 0..bcb.get_function_count() {
      let data = bcb.get_function_data(fid);
      // 先取出名字结束 `parsed` 对 `data` 的借用，命中后才能把字节搬进结果
      let debugname = match from_function_bytecode(&data, &table) {
        Some(parsed) => parsed.debugname.clone(),
        None => continue,
      };
      match debugname.as_str() {
        "inlinee" if inlinee_bytecode.is_none() => inlinee_bytecode = Some(data),
        "caller" if caller_bytecode.is_none() => caller_bytecode = Some(data),
        _ => {}
      }
    }

    Some(BytecodeRes {
      inlinee_bytecode: inlinee_bytecode?,
      caller_bytecode: caller_bytecode?,
      string_table: strings,
    })
  }
}
