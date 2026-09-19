use core::ptr::null;

use ulua_compiler::records::compile_options::CompileOptions;

use crate::records::global_options::with;

/// cpp `copts()`: 以全局选项构造 `CompileOptions`
pub fn copts() -> CompileOptions {
  // vector_* 指向 GLOBAL_OPTIONS 持有的 CString, 随线程局部静态存活至进程退出;
  // 参数解析全部完成后才调用本函数, 期间不再 mutate, 指针不会失配。
  with(|opts| CompileOptions {
    optimization_level: opts.optimization_level,
    debug_level: opts.debug_level,
    type_info_level: opts.type_info_level,
    vector_lib: opts.vector_lib.as_ref().map_or(null(), |s| s.as_ptr()),
    vector_ctor: opts.vector_ctor.as_ref().map_or(null(), |s| s.as_ptr()),
    vector_type: opts.vector_type.as_ref().map_or(null(), |s| s.as_ptr()),
    ..CompileOptions::default()
  })
}
