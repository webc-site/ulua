// C++ (ControlFlowGraph): `using DefId = NotNull<Definition>;` where
// `Definition = SymDef`。裸指针身份令牌已在任务 #17 收敛为 u32 句柄 +
// `records::sym_def_registry` 单点注册表（与 `def_registry` 的 `DefId` 同
// 形状）：`register_sym_def` 只在 `CfgAllocator::new_definition` 分配点出
// 现，业务侧解引用一律经 `resolve_sym_def` 取 `Option<&SymDef>`。
pub use crate::records::sym_def_registry::SymDefId as DefId;
