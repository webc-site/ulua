//! Source: `Analysis/include/Luau/Def.h:16` (hand-ported)
// C++ `using DefId = NotNull<const Def>` — 身份令牌，Rust 侧为 u32 句柄 +
// `records::def_registry` 单点注册表（裸指针只在分配点出现）。
// (Previously mis-aliased to ControlFlowGraph.h's SymDef/Definition, which is
// the NEW dataflow system's unrelated DefId.)
pub use crate::records::def_registry::DefId;
