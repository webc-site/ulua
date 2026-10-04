//! 实现层模块清单。`pub` 项一一对应 cpp `Repl.h` / `ReplRequirer.h` 的导出，
//! `pub(crate)` 项对应同名 cpp 文件里的 `static`。

// —— cpp `Repl.h` / `ReplRequirer.h` 导出的入口（供 ulua-cli-test 夹具复用）——
pub mod create_cli_require_context;
pub mod get_completions;
pub mod repl_main;
pub mod run_code;
pub mod setup_state;

// —— 以下为 Repl.cpp / ReplRequirer.cpp 的 crate 内部实现（cpp `static`）——
pub(crate) mod compile_source;
pub(crate) mod complete_indexer;
pub(crate) mod complete_partial_matches;
pub(crate) mod complete_repl;
pub(crate) mod copts;
pub(crate) mod counters_active;
pub(crate) mod counters_dump;
pub(crate) mod counters_function_callback;
pub(crate) mod counters_init;
pub(crate) mod counters_track;
pub(crate) mod counters_value_callback;
pub(crate) mod coverage_active;
pub(crate) mod coverage_callback;
pub(crate) mod coverage_dump;
pub(crate) mod coverage_init;
pub(crate) mod coverage_track;
pub(crate) mod create_dump_writer;
pub(crate) mod get_file_path;
pub(crate) mod ic_get_completions;
pub(crate) mod load;
pub(crate) mod load_history;
pub(crate) mod lua_loadstring;
pub(crate) mod main;
pub(crate) mod profiler_dump;
pub(crate) mod profiler_loop;
pub(crate) mod profiler_start;
pub(crate) mod profiler_stop;
pub(crate) mod profiler_trigger;
pub(crate) mod run_file;
pub(crate) mod run_repl;
pub(crate) mod run_repl_impl;
pub(crate) mod sigint_callback;
pub(crate) mod sigint_handler_repl;
// 收口门面：Ctrl-C 处理的注册/撤回（libc FFI 与 null 协议值只留这里）
pub(crate) mod sigint_setup;
pub(crate) mod stack_function_name;
// `*mut LuaState` 入口解引用门面（各 VM 边界入口共用，契约见模块头）
pub(crate) mod state_ref;
