//! Source: `Common/src/TimeTrace.cpp:109-113` (hand-ported)
//! C++ `getGlobalContext()` returns a process-wide `shared_ptr` singleton:
//! ```cpp
//! static std::shared_ptr<GlobalContext> context = std::shared_ptr<GlobalContext>{new GlobalContext};
//! return context;
//! ```
use std::sync::{Arc, OnceLock};

use crate::records::global_context::GlobalContext;

/// cpp `TimeTrace` 镜像工件（`Common/src/TimeTrace.cpp:109-113`），sync-cpp 维护；
/// 仅本 crate 打点机制消费（`create_scope_data`/`ThreadContext::new`），降 `pub(crate)`。
///
/// review.md §5 的 `ctor` 在此**不引入**（有意例外，故写明理由）：cpp 的函数内
/// `static` 单例（首次调用惰性构造、由编译器 guard 变量保证恰好一次）在 Rust 侧的
/// 直译就是 `OnceLock::get_or_init`——同样是首次调用初始化、同样线程安全、后续读取
/// 只是一次原子加载；`ctor` 走的是链接期 `.init_array` 构造函数，会把「未使用
/// TimeTrace 的进程也付出启动期分配」变成无条件成本，并丢掉这里的惰性语义。
pub(crate) fn get_global_context() -> Arc<GlobalContext> {
  static CONTEXT: OnceLock<Arc<GlobalContext>> = OnceLock::new();
  CONTEXT
    .get_or_init(|| Arc::new(GlobalContext::new()))
    .clone()
}
