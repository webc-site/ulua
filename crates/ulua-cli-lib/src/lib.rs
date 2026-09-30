//! `ulua-cli-lib` — 各 CLI 可执行入口共用的宿主层。
//!
//! FFI 边界登记（review.md §9.3）：本 crate 的 `unsafe` / 裸指针仅出现在两类真边界，
//! 其余均为安全 Rust——
//!
//! - ulua-vm 的 Lua/C API：以 `*mut LuaState` 收发的 VM 宿主操作（`safe_get_table`、
//!   `try_replace_top_with_index`、`setup_arguments`、`lua_collectgarbage` 等），每个
//!   调用点带 `// SAFETY:` 论证状态存活与栈配平；
//! - 装入 C API 的回调（`extern "C-unwind"`）：`assertion_handler`、
//!   `lua_collectgarbage`——各自带 `DELIBERATE DEVIATION` 说明。
extern crate alloc;

pub mod enums;
pub mod functions;
pub mod macros;
pub mod methods;
pub mod records;

/// CLI 集成测试共享脚手架；默认不编译，测试侧通过 `test-utils` feature 启用。
#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;
