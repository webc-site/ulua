//! Source: `Analysis/include/Luau/ControlFlowGraph.h:49` (hand-ported)
// C++ `using RefinementId = NotNull<Refinement>` over ControlFlowGraph.h's
// OWN Refinement variant (previously mis-aliased to Refinement.h's unrelated
// RefinementId).
//
// 表示判定（b14 类型代数化）：本句柄的唯一生产者是其属主
// `RefinementArena { allocator: TypedAllocator<Refinement> }`——32 KiB 分块
// bump arena。块只追加、既分配象永不搬运（`allocate` 经 `append_block` 对
// null 块按 bad_alloc 语义 panic，槽位偏移恒在块界内），故发出的指针地址
// 稳定且恒非空；arena 由 `CfgAllocator::refinement_arena` 独占持有，随整个
// CFG 构建与 dump 期存活（`freeze` 只封闭后续分配，不使既有地址失效）。
// 据此用类型编码非空：`Handle<Refinement>`（`NonNull` 的 crate 统一最小
// 边界封装，unsafe 解引用收拢在 `arena_handle.rs` 一处），Copy/Eq/Hash/Debug
// 与原裸指针逐位同构，物化借用与原 `&*r` 同构，业务侧不再有 `unsafe`。
// 真正可空处（cpp `std::optional<RefinementId>`）一律 `Option<RefinementId>`
// 表达（NonNull 使 `Option<Handle<T>>` 无额外开销）。
// 注意：这与 `Refinement.h:22` 的可空同名别名（`refinement_id_refinement`）
// 是两个不同变体的不同表示，勿混用。
use crate::{
  records::arena_handle::Handle, type_aliases::refinement_control_flow_graph::Refinement,
};

pub type RefinementId = Handle<Refinement>;
