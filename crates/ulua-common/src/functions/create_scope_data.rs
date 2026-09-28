//! Source: `Common/src/TimeTrace.cpp:263-266` (hand-ported)
//! C++ `LUAU_NOINLINE uint16_t createScopeData(const char* name, const char* category)`:
//! `return createToken(*Luau::TimeTrace::getGlobalContext(), name, category);`

use crate::functions::{create_token::create_token, get_global_context::get_global_context};

/// cpp `TimeTrace` 镜像工件，sync-cpp 维护；保留 `pub`：`LUAU_TIMETRACE_*` 宏
/// 启用形态在下游 crate 展开时直接调用本函数（宏 ABI 面）。
pub fn create_scope_data(name: &'static str, category: &'static str) -> u16 {
  let context = get_global_context();
  create_token(&context, name, category)
}
