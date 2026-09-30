/// 覆盖率/计数器/codegen 开关探针：调用方必须在构造 `ReplRequirer` 时提供
/// （对应 cpp `ReplRequirer.h:18` 的 `using BoolCheck = bool (*)()`；
/// Rust 侧函数指针即等价物，且这些回调只在 Rust 内部调用，无需 `extern` ABI）。
pub type BoolCheck = fn() -> bool;
