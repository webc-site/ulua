//! Source: `Analysis/include/Luau/ControlFlowGraph.h`

// C++ `using BlockId = NotNull<Block>;` 的裸指针镜像已在 #17 续任务收敛为
// u32 句柄 + `records::block_registry` 单点注册表（与 `sym_def_registry` 的
// `SymDefId` 同形状）：`register_block` 只在 `CfgAllocator::new_block` 分配点
// 出现，业务侧解引用一律经 `resolve_block`/`resolve_block_mut` 取
// `Option<&Block>`/`Option<&mut Block>`。
pub use crate::records::block_registry::BlockId;
