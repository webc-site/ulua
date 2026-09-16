use alloc::ffi::CString;
use core::cell::RefCell;

/// cpp 默认优化级别 (`globalOptions.optimizationLevel = 1`)
pub const DEFAULT_OPTIMIZATION_LEVEL: i32 = 1;
/// cpp 默认调试级别 (`globalOptions.debugLevel = 1`)
pub const DEFAULT_DEBUG_LEVEL: i32 = 1;
/// cpp 默认类型信息级别 (`globalOptions.typeInfoLevel = 0`)
pub const DEFAULT_TYPE_INFO_LEVEL: i32 = 0;

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

thread_local! {
    /// 镜像 cpp 全局 gOptions: 单线程 CLI 专用全局。
    /// `thread_local` + `RefCell` 取代 `static mut`，全程零 unsafe；
    /// 闭包与外界不得重入（否则 RefCell 借用冲突 panic，非 UB），CLI 单遍解析/编译不构成重入。
    static GLOBAL_OPTIONS: RefCell<GlobalOptions> = const {
      RefCell::new(GlobalOptions {
        optimization_level: DEFAULT_OPTIMIZATION_LEVEL,
        debug_level: DEFAULT_DEBUG_LEVEL,
        type_info_level: DEFAULT_TYPE_INFO_LEVEL,
        vector_lib: None,
        vector_ctor: None,
        vector_type: None,
        only_parse: false,
        parse_cst: false,
        dump_reg_spills: false,
      })
    };
}

/// 读取全局选项
///
/// 不得与 `mutate` 重入 (RefCell 借用冲突会 panic，非 UB)。
pub fn with<R>(f: impl FnOnce(&GlobalOptions) -> R) -> R {
  GLOBAL_OPTIONS.with(|opts| f(&opts.borrow()))
}

/// 修改全局选项
///
/// 不得与 `with`/`mutate` 重入 (RefCell 借用冲突会 panic，非 UB)。
pub fn mutate(f: impl FnOnce(&mut GlobalOptions)) {
  GLOBAL_OPTIONS.with(|opts| f(&mut opts.borrow_mut()))
}
