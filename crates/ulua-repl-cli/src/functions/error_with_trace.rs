//! 「yield 消息或栈顶错误字符串 + trace 头 + debugtrace」组装已上移进 VM
//! （[`ulua_vm::functions::error_with_trace`]，供 run_loaded_chunk / runFile
//! 共用）；此处按原路径重导出，runFile 等调用点不变。

pub(crate) use ulua_vm::functions::error_with_trace::error_with_trace;
