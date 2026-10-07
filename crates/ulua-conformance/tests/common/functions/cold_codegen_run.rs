//! 冷函数原生编译驱动门面：不开口 `unsafe`，codegen 上下文创建、原生重编、
//! resume 全部经由 `safe_api` 对应能力函数。

use ulua_code_gen::{
  enums::code_gen_flags::CodeGenFlags, functions::luau_codegen_supported::luau_codegen_supported,
  records::compilation_options::CompilationOptions,
};

use crate::common::functions::{
  compile_and_load::compile_and_load,
  openlibs_and_sandbox::openlibs_and_sandbox,
  run_conformance::codegen,
  safe_api::{L, codegen_create, compile_native, resume},
};

/// cpp `tests/Conformance.test.cpp` 中 `HugeConstantTable`（:4915）、`HugeFunction`
/// （:4814）、`LargeNestedClosure`（:4968）三例逐字重复的冷函数原生编译驱动：
/// `codegen 就位 → openlibs+sandbox → 编译加载 → CodeGen_ColdFunctions 原生重编 →
/// resume 一次并要求成功`。三步与 chunkname/源码耦合、其余完全同形，收敛前本 crate
/// 内有 3 份副本（均在 `conformance/bytecode.rs`），现全部切到本门面。
///
/// 各副本相对 `Conformance.test.cpp` 的既有次序保持一致：`codegen_create` 先于
/// openlibs，`compile`（`compile_internal` 冷标志）在加载之后、resume 之前。
///
/// 返回前 `resume` 状态已断言为 0，栈顶为 chunk 返回值；调用方用 `lua_tonumber` 等
/// 宏核对期望值。
///
/// 前提（与文档化的上游用法一致）：`l` 是存活、尚未 openlibs 的 `LuaState`，
/// 且 `source` 编译产物可被该状态加载。
pub fn cold_codegen_run(l: L, source: &str, chunkname: &str) {
  if codegen() && luau_codegen_supported() != 0 {
    codegen_create(l);
  }

  openlibs_and_sandbox(l);
  compile_and_load(l, source, chunkname, None);

  if codegen() && luau_codegen_supported() != 0 {
    let native_options = CompilationOptions {
      flags: CodeGenFlags::CodeGenColdFunctions as u32,
      ..Default::default()
    };
    let _ = compile_native(l, &native_options);
  }

  let status = resume(l, None, 0);
  assert_eq!(0, status);
}
