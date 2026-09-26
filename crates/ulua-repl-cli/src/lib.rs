//! `ulua-repl-cli` — REPL / 脚本运行器的实现层，对应 cpp `CLI/src/Repl.cpp`
//! （+ `ReplRequirer.cpp` / `Coverage.cpp` / `Counters.cpp` / `Profiler.cpp`）。
//!
//! 对外可见性刻意对齐 cpp 头文件：只有 `CLI/include/Luau/Repl.h` 与
//! `ReplRequirer.h` 声明的入口（`setupState` / `runCode` / `getCompletions` /
//! `createCliRequireContext` / `requireConfigInit` / `replMain`）是 `pub`，
//! 其余全部 `pub(crate)`，与 cpp 里的 `static` 作用域一一对应。
//!
//! `ulua-cli-test`（cpp `tests/Repl.test.cpp` + `tests/RequireByString.test.cpp`
//! 的夹具层）通过这些入口复用实现，与 cpp 测试直接链接 Repl.cpp 的做法一致，
//! 不再整份复制实现代码。
extern crate alloc;

pub mod functions;
pub(crate) mod records;
pub(crate) mod type_aliases;

pub use functions::main::main;
