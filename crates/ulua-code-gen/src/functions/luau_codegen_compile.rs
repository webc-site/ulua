use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::compile_internal::compile_internal, records::compilation_options::CompilationOptions,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luau_codegen_compile(l: *mut LuaState, idx: i32) {
  // Safety: compile_internal 依 `# Safety` 契约消费 l（存活 LuaState*）与 idx（界内栈位），
  // None 为“无 meta 输出”的默认形参（对齐 C++ nullptr），返回值显式丢弃与 C++ 一致。
  unsafe {
    let _ = compile_internal(&None, l, idx, &CompilationOptions::default(), None);
  }
}

/// J1 Phase 2b：暖重编译入口——对已编译闭包强制重编译并重绑定
/// （`force_recompile` 路径；旧 native module 目前泄漏，见 CompilationOptions 注）。
///
/// # Safety
/// 同 [`luau_codegen_compile`]：`l` 存活、`idx` 为界内栈位，且须处于安全点
/// （目标 proto 无在途 native 帧——调用方在解释器环/VM 出口投递）。
pub unsafe fn luau_codegen_warm_recompile(l: *mut LuaState, idx: i32) {
  unsafe {
    let options = CompilationOptions {
      force_recompile: true,
      ..CompilationOptions::default()
    };
    let _ = compile_internal(&None, l, idx, &options, None);
  }
}
