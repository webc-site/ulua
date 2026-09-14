use ulua_common::FFlag::DebugLuauTimeTracing;

/// cpp 各 CLI 的
/// `#if !defined(LUAU_ENABLE_TIME_TRACE)` 守卫 (Repl.cpp / Analyze.cpp / Compile.cpp):
/// 本构建未启用时间追踪, `--timetrace` 时打印消息并让调用方返回 1。
/// 返回 `true` 表示应当报错退出。
pub fn time_trace_unsupported() -> bool {
  if DebugLuauTimeTracing.get() {
    eprintln!("To run with --timetrace, Luau has to be built with LUAU_ENABLE_TIME_TRACE enabled");
    true
  } else {
    false
  }
}
