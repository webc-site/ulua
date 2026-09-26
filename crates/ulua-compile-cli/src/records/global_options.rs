use alloc::vec::Vec;
use core::cell::RefCell;

// 与 ulua-cli-lib 共享同一默认值定义（r1-14 已把 repl/bytecode 侧下沉，
// compile 侧此前留了一份可独立漂移的副本）。
use ulua_cli_lib::records::global_options::{DEFAULT_DEBUG_LEVEL, DEFAULT_OPTIMIZATION_LEVEL};
/// cpp 默认类型信息级别 (`globalOptions.typeInfoLevel = 0`)
pub(crate) const DEFAULT_TYPE_INFO_LEVEL: i32 = 0;

/// cpp `GlobalOptions` (CLI/src/Compile.cpp:45-59)。
///
/// vector_* 在 cpp 为 `const char*` 指向 argv 内存; Rust 侧改由本结构持有
/// 「NUL 结尾字节串」(`Vec<u8>` 末尾补 0), `copts()` 时取 `as_ptr().cast()`
/// 透传给编译器 (指针随线程局部静态存活)。
///
/// cpp 的 `dumpRegSpills` 不在此列: 它唯一的读取点是 `compileFile` 里
/// `options.includeRegSpills` 的透传，而 Rust 侧 `AssemblyOptions` 还没有该字段
/// (TODO(cross-crate), 见 docs/CONFORMANCE.md)，所以它是「只写不读」的全局状态，
/// 改为 `run()` 内的局部量并在解析后提示未生效。
#[derive(Clone)]
pub struct GlobalOptions {
  pub optimization_level: i32,
  pub debug_level: i32,
  pub type_info_level: i32,

  pub vector_lib: Option<Vec<u8>>,
  pub vector_ctor: Option<Vec<u8>>,
  pub vector_type: Option<Vec<u8>>,

  pub only_parse: bool,
  pub parse_cst: bool,
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

/// 快照当前线程的全局选项（仅主线程、参数解析全部完成后调用一次）。
///
/// 并行编译时 rayon 工作线程的 `GLOBAL_OPTIONS` 是线程局部默认值，
/// 任务内必须先 [`restore`] 本快照，`with`/`copts` 才能读到已解析选项。
pub fn snapshot() -> GlobalOptions {
  with(Clone::clone)
}

/// 把快照写回当前线程的全局选项（每个并行编译任务入口调用一次）。
///
/// `vector_*` 的堆缓冲随副本迁入本线程 thread_local，`copts()` 在本任务内
/// 取指针、存活至下次 `restore`/线程退出，满足「指针不先于读数失效」契约。
pub fn restore(snapshot: &GlobalOptions) {
  mutate(|opts| *opts = snapshot.clone());
}
