//! `compile` → `luau_load` 的一次性样板：对应 cpp 各 codegen 用例里逐字重复的
//! `char* bytecode = luau_compile(...); int result = luau_load(...); free(bytecode);
//! REQUIRE(result == 0);`（如 `tests/Conformance.test.cpp:5205-5208`、`5033-5036`）。
//! cpp 侧的 malloc/free 所有权契约在 Rust 端由 owned [`Vec<u8>`] 产物消解。

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_vm::{functions::luau_load::luau_load, records::lua_state::LuaState};

/// 编译 `source` 并把它作为 `chunkname` 加载到 `l`，失败即中止用例。
///
/// `options` 为 `None` 时取缺省编译选项（cpp 的
/// `luau_compile(source, size, nullptr, &size)` 中 null options 同形）。
///
/// # Safety
/// 调用方须保证 `l` 是存活且可加载字节码的 `LuaState`（上游同一前提）。
pub unsafe fn compile_and_load(
  l: *mut LuaState,
  source: &str,
  chunkname: &str,
  options: Option<&mut CompileOptions>,
) {
  let options = options.copied().unwrap_or_default();

  let bytecode = compile(source, &options, &ParseOptions::default(), NoopEncoder);

  // Safety: 测试并行运行下本资源由本用例独占；`l` 是存活且可加载字节码的状态
  // （函数级契约），`chunkname` 为合法借用，`bytecode` 是本帧拥有的产物切片。
  let result = unsafe { luau_load(l, chunkname, &bytecode, 0) };
  assert_eq!(0, result, "luau_load failed for {chunkname:?}");
}
