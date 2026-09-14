use core::ptr::null;

use ulua_compiler::records::compile_options::CompileOptions;

use crate::records::global_options::with;

/// cpp `copts()`: 以全局选项构造 `CompileOptions`
pub fn copts() -> CompileOptions {
  // SAFETY: 单线程 CLI, 无并发写全局选项
  unsafe {
    with(|opts| CompileOptions {
      optimization_level: opts.optimization_level,
      debug_level: opts.debug_level,
      type_info_level: opts.type_info_level,
      // vector_* 指向 GLOBAL_OPTIONS 持有的 CString, 随全局静态存活至进程退出
      vector_lib: opts.vector_lib.as_ref().map_or(null(), |s| s.as_ptr()),
      vector_ctor: opts.vector_ctor.as_ref().map_or(null(), |s| s.as_ptr()),
      vector_type: opts.vector_type.as_ref().map_or(null(), |s| s.as_ptr()),
      ..CompileOptions::default()
    })
  }
}
