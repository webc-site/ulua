use crate::records::label::Label;

/// cpp `IrLoweringA64.h:83` / `IrLoweringX64.h:87` 各自声明的同名嵌套结构；
/// 两处字段完全一致，故合并为单一定义供两个后端共用。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[derive(Default)]
pub struct ExitHandler {
  pub self_: Label,
  pub pcpos: u32,
}
