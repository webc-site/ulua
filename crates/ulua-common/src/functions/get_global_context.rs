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
pub(crate) fn get_global_context() -> Arc<GlobalContext> {
  static CONTEXT: OnceLock<Arc<GlobalContext>> = OnceLock::new();
  CONTEXT
    .get_or_init(|| Arc::new(GlobalContext::new()))
    .clone()
}
