use alloc::ffi::CString;

/// cpp `GlobalOptions` (CLI/src/Compile.cpp:45-59)。
///
/// vector_* 在 cpp 为 `const char*` 指向 argv 内存; Rust 侧改由本结构持有
/// `CString`, `copts()` 时取 `as_ptr()` 透传给编译器 (指针随全局静态存活)。
pub struct GlobalOptions {
  pub optimization_level: i32,
  pub debug_level: i32,
  pub type_info_level: i32,

  pub vector_lib: Option<CString>,
  pub vector_ctor: Option<CString>,
  pub vector_type: Option<CString>,

  pub only_parse: bool,
  pub parse_cst: bool,

  pub dump_reg_spills: bool,
}

/// 镜像 cpp 全局 gOptions: 单线程 CLI 专用全局
static mut GLOBAL_OPTIONS: GlobalOptions = GlobalOptions {
  optimization_level: 1,
  debug_level: 1,
  type_info_level: 0,
  vector_lib: None,
  vector_ctor: None,
  vector_type: None,
  only_parse: false,
  parse_cst: false,
  dump_reg_spills: false,
};

/// 读取全局选项, unsafe 访问收敛于此
///
/// # Safety
///
/// 仅允许在无并发写全局选项时调用 (单线程 CLI 满足)
pub unsafe fn with<R>(f: impl FnOnce(&GlobalOptions) -> R) -> R {
  // SAFETY: 单线程 CLI, 调用方保证无并发访问
  unsafe { f(&*core::ptr::addr_of!(GLOBAL_OPTIONS)) }
}

/// 修改全局选项, unsafe 访问收敛于此
///
/// # Safety
///
/// 仅允许在无并发读写全局选项时调用 (单线程 CLI 满足)
pub unsafe fn mutate(f: impl FnOnce(&mut GlobalOptions)) {
  // SAFETY: 单线程 CLI, 调用方保证无并发访问
  unsafe { f(&mut *core::ptr::addr_of_mut!(GLOBAL_OPTIONS)) }
}
