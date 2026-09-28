//! ControlFlowGraph 之外的旧细化系统（`Def.h` 侧）的 `DefId`：与
//! `def_id_def` 同一 u32 句柄类型（cpp `NotNull<const Def>` 的两种书写）。
pub use crate::records::def_registry::DefId;
