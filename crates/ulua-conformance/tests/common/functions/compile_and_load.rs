//! `compile` → `luau_load` 的一次性样板：对应 cpp 各 codegen 用例里逐字重复的
//! `char* bytecode = luau_compile(...); int result = luau_load(...); free(bytecode);
//! REQUIRE(result == 0);`（如 `tests/Conformance.test.cpp:5205-5208`、`5033-5036`）。
//! cpp 侧的 malloc/free 所有权契约在 Rust 端由 owned [`Vec<u8>`] 产物消解。
//!
//! 本门面自身不开口 `unsafe`：编解码与加载全部走 `safe_api::load_source`，
//! 裸指针收口只发生在门面文件一处。

use ulua_compiler::records::compile_options::CompileOptions;

use crate::common::functions::safe_api::{L, load_source};

/// 编译 `source` 并把它作为 `chunkname` 加载到 `l`，失败即中止用例。
///
/// `options` 为 `None` 时取缺省编译选项（cpp 的
/// `luau_compile(source, size, nullptr, &size)` 中 null options 同形）。
pub fn compile_and_load(l: L, source: &str, chunkname: &str, options: Option<&mut CompileOptions>) {
  let mut fallback = CompileOptions::default();
  let options = options.unwrap_or(&mut fallback);

  let result = load_source(l, chunkname, source.as_bytes(), options);
  assert_eq!(0, result, "luau_load failed for {chunkname:?}");
}
