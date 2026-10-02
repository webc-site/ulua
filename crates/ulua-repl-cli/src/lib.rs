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
//!
//! FFI 边界登记（review.md §9.3）：本 crate 保留的 `unsafe` / 裸指针全部落在三类
//! 边界，其余业务逻辑均为安全 Rust——
//!
//! - ulua-vm 的 Lua/C API：以 `*mut LuaState` 收发的栈操作（`lua_*`/`luaL_*`/
//!   `luau_load`/`(*l).resume` 等），单步调用点带 `// Safety:` 论证其存活与栈配平；
//!   多步不可拆序列收敛为带 `# Safety` 契约的最小私有封装（`load.rs` 的
//!   `spawn_module_thread`/`prepare`/`check_run`、`run_file.rs` 的 `sandboxed_thread`；
//!   发散抛出经 `load.rs` 的 `throw` 单点，`pub(crate)` 供 `sigint_callback` 复用）。
//!   纯编排入口 `run_repl`/`run_repl_impl`/`run_file` 已收编为 `pub(crate)` 安全 fn
//!   （解引用关在 `state` 门面与各消费点 `unsafe` 块内，句柄契约降级为文档约定）；仅
//!   `run_code`/`setup_state` 因是跨 crate `pub` c-API 句柄边界（夹具以 `unsafe` 块断言
//!   句柄有效，safe 化会撤掉该契约强制并命中 `not_unsafe_ptr_arg_deref`）而保留
//!   `pub unsafe fn` 签名，逐处 `DELIBERATE DEVIATION` 定性；
//! - 装入 VM 的 C-ABI 回调（`extern "C-unwind"`：`lua_loadstring`、`sigint_callback`、
//!   `profiler_interrupt`、`c_abi_cb!` 生成的 coverage/counters 外壳）——各自带
//!   `DELIBERATE DEVIATION` 说明；
//! - 平台/进程原语：`sigint_setup` 门面的 OS 信号注册与 async-signal 共享的
//!   `REPL_STATE` null 协议值。采样器的线程独占字段已改 `thread_local` 持有、
//!   跨线程面只剩原子发布量（`profiler_trigger`），无 `UnsafeCell` 分区契约。
//!
//! c-API 语义要求 NULL 的调用点以 `FFI: c-API 要求 NULL` 就地标注。
extern crate alloc;

pub mod functions;
pub(crate) mod records;
pub(crate) mod type_aliases;

pub use functions::main::main;
