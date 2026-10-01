#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Arch {
  #[default]
  X64,
  A64,
}

/// codegen 上下文的封闭集合判别：替代 cpp `BaseCodeGenContext` 的虚表分派。
///
/// cpp 用 `StandaloneCodeGenContext` / `SharedCodeGenContext` 两个子类的虚覆写
/// （`bindModule` / `tryBindExistingModule` / `onCloseState`）区分行为；Rust 侧类型集合
/// 编译期封闭，收口为同一结构体 + 本判别枚举，分派点全部 `match` 单态完成
/// （见 `records::base_code_gen_context::BaseCodeGenContext`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodeGenContextKind {
  /// 宿主 VM 独占持有：状态关闭时上下文自行回收（对齐 cpp `StandaloneCodeGenContext` 的 `delete this`）。
  Standalone,
  /// 创建者（`create_shared_code_gen_context` / `destroy_shared_code_gen_context`）持有，
  /// 可被多个 VM 注册共享；状态关闭时不回收（对齐 cpp `SharedCodeGenContext` 的 no-op `onCloseState`）。
  Shared,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum IncludeIrPrefix {
  #[default]
  No,
  Yes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum IncludeRegFlowInfo {
  #[default]
  No,
  Yes,
}
