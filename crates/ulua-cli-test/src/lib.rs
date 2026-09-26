//! cpp `tests/Repl.test.cpp` 的 `class ReplFixture` 与
//! `cpp/tests/RequireByString.test.cpp` 的 `class ReplWithPathFixture` 的 Rust
//! 夹具层（对应 review.md「清理冗余重复」）：
//!
//! cpp 侧这两个测试都**链接** CLI 的实现（`#include <Luau/Repl.h>`，用到的导出
//! 仅 `setupState` / `runCode` / `getCompletions`），自身只定义 fixture 类。
//! 因此本 crate 同样只保留夹具（fixture）层，CLI 实现全部取自
//! [`ulua_repl_cli`]，不再复制一份。

extern crate alloc;

pub mod enums;
pub mod functions;
pub mod methods;
pub mod records;
pub(crate) mod type_aliases;
