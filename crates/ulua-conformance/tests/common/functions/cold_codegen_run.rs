use core::ptr::null_mut;

use ulua_code_gen::{
  enums::code_gen_flags::CodeGenFlags,
  functions::{
    compile_internal::compile_internal, luau_codegen_create::luau_codegen_create,
    luau_codegen_supported::luau_codegen_supported,
  },
  records::compilation_options::CompilationOptions,
};
use ulua_vm::{functions::lua_resume::lua_resume, records::lua_state::LuaState};

use crate::common::functions::{
  compile_and_load::compile_and_load, openlibs_and_sandbox::openlibs_and_sandbox,
  run_conformance::codegen,
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
/// # Safety
/// 调用方须保证 `l` 是存活、尚未 openlibs 的 `LuaState`（与 `openlibs_and_sandbox`
/// 同一前提），且 `source` 编译产物可被该状态加载。
pub unsafe fn cold_codegen_run(l: *mut LuaState, source: &str, chunkname: &str) {
  // Safety: `l` 的存活与未初始化前提由调用方保证；本段仅在支持原生编译时创建 codegen
  // 上下文（次序上先于 openlibs，与 cpp 一致）。
  unsafe {
    if codegen() && luau_codegen_supported() != 0 {
      luau_codegen_create(l);
    }
  }

  // Safety: 两个门面都只接受存活 `l`；`compile_and_load` 只接受其断言成功的产物。
  unsafe {
    openlibs_and_sandbox(l);
    compile_and_load(l, source, chunkname, None);
  }

  // Safety: `l` 存活且栈顶为刚加载的闭包，`compile_internal` 以 `-1` 索引它；
  // `null_mut()` 表示不收集编译统计（与 cpp `Luau::CodeGen::compile(L, -1, options)` 一致）。
  unsafe {
    if codegen() && luau_codegen_supported() != 0 {
      let native_options = CompilationOptions {
        flags: CodeGenFlags::CodeGenColdFunctions as u32,
        ..Default::default()
      };
      // FFI: c-API 要求 NULL
      let _ = compile_internal(&None, l, -1, &native_options, null_mut());
    }
  }

  // Safety: `l` 存活；`lua_resume` 的 `null_mut()` from 参数为「无父线程」哨兵，
  // 与 cpp 原样一致。断言成功状态后栈顶即 chunk 返回值。
  // FFI: c-API 要求 NULL
  let status = unsafe { lua_resume(l, null_mut(), 0) };
  assert_eq!(0, status);
}
