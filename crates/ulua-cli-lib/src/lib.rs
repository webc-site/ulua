//! `ulua-cli-lib` — 各 CLI 可执行入口共用的宿主层。
//!
//! FFI 边界登记（review.md §9.3）：本 crate 的 `unsafe` / 裸指针仅出现在两类真边界，
//! 其余均为安全 Rust——
//!
//! - ulua-vm 的 Lua/C API：以 `*mut LuaState` 收发的 VM 宿主操作（`lua_collectgarbage`
//!   等 `extern "C-unwind"` 入口），入口内一次 `unsafe { &mut *l }` 物化为带调用期
//!   生命周期的借用并就地写 `// Safety:` 论证状态存活与栈配平，下游一律走 ulua-vm 的
//!   引用形安全面（`safe_get_table`、`try_replace_top_with_index`、`setup_arguments`
//!   均为 `&mut LuaState` 借用形的安全 fn）；
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
