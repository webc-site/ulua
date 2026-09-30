//! Source: `Analysis/include/Luau/Refinement.h`

// Refinement.h:22 — using RefinementId = Refinement*; (can be null)
use core::ptr::null_mut;

use crate::type_aliases::refinement_refinement::Refinement;
pub type RefinementId = *mut Refinement;

/// 可空句柄的空哨兵在类型定义处收口（与 `Inference::no_refinement`、
/// `SymDefId::NULL` 同一 B 型先例）：组合子工厂（`records/refinement_arena_refinement.rs`）
/// 已不再手写 `null_mut()`，退化输入统一返回 `Option::None`；仅当消费方须把
/// `None` 落回直存可空句柄的数据槽（`Inference.refinement`、
/// `InferencePack.refinements`、`Conjunction/Disjunction` 结点字段等）时，
/// 才在此取具名哨兵，业务调用点不再散落空指针字面量。
pub const NULL_REFINEMENT_ID: RefinementId = null_mut();
