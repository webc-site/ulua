//! Source: `Analysis/include/Luau/ControlFlowGraph.h:38` (hand-ported)
// C++ `using InstrId = NotNull<Instruction>` over ControlFlowGraph.h's OWN
// Instruction variant（此前曾误别名到 Instruction.h 的无关 InstrId）。裸指针
// 镜像已在 #17 续任务收敛为 u32 句柄 + `records::instr_registry` 单点注册表
// （与 `sym_def_registry`/`block_registry` 同形状）：`register_instruction`
// 只在 `CfgAllocator::new_instruction` 分配点出现，业务侧解引用一律经
// `resolve_instruction`/`resolve_instruction_mut`。
pub use crate::records::instr_registry::InstrId;
