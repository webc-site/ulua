//!
//! B 型（可空句柄字段的定义处收口）：`ty`/`refinement` 分别是 type_aliases 声明
//! 的可空 arena 句柄（`TypeId` 可为空、`RefinementId` 见 cpp Refinement.h:22
//! "(can be null)"）。本文件是这两个空值的构造/收口处：`dense_default` 仅作
//! `DenseHashMap` 空槽填充值、消费方永不可见；`new`/`no_refinement` 分别对应
//! cpp `Inference{}` 与 `Inference{ty}` 的聚合初始化缺省，语义为「无类型/
//! 无 refinement」，读取方（check_* 系列）按 is_null 分支处理。
use core::ptr::null;

use ulua_common::records::dense_hash_table::DenseDefault;

use crate::type_aliases::{
  refinement_id_refinement::{NULL_REFINEMENT_ID, RefinementId},
  type_id::TypeId,
};
#[derive(Debug, Clone)]
pub struct Inference {
  pub ty: TypeId,
  /// §2(b)：cpp `Inference { RefinementId refinement; }`（Refinement.h:22
  /// `using RefinementId = Refinement*`，可为 null）。本字段是整条
  /// **裸指针形态** Refinement 子系统的又一个数据槽：`RefinementId` 别名同时被
  /// arena 组合子工厂（`variadic(&[RefinementId])`、`negation/conjunction/
  /// disjunction/equivalence/proposition(RefinementId,..) -> Option<RefinementId>`）
  /// 与 Negation/Conjunction/Disjunction 结点字段、`InferencePack.refinements` 复用；
  /// 消费点（`constraint_generator_check*`、`check_expr_call` 把 `inference.refinement`
  /// push 进 `argument_refinements` 再喂 `variadic`、`compute_refinement(..) ->
  /// RefinementId`）均以裸 `RefinementId` 为形参/键。单把本字段改 `Option<NonNull>`
  /// 会在每次传入组合子时逼出 `.as_ptr()` 倒灌，或被迫重写整个 `RefinementId` 别名
  /// 及全部结点字段/`InferencePack`（均超出本轮 12 字段范围），收益仅字段形态，故按
  /// §2(b) 连同该子系统整体保留空指针（空值构造已收口于本定义处的 `NULL_REFINEMENT_ID`
  /// /具名 ctor），避免后续误改。
  pub refinement: RefinementId,
}

impl DenseDefault for Inference {
  fn dense_default() -> Self {
    Self {
      ty: null(),
      refinement: NULL_REFINEMENT_ID,
    }
  }
}

impl Inference {
  // C++: `Inference()` — `ty` default-initialized to nullptr, no refinement.
  // (Analysis/include/Luau/ConstraintGenerator.h)
  pub fn new() -> Self {
    Self {
      ty: null(),
      refinement: NULL_REFINEMENT_ID,
    }
  }
  // C++: `Inference(TypeId ty, RefinementId refinement = nullptr)`.
  pub fn inference_type_id_refinement_id(ty: TypeId, refinement: RefinementId) -> Self {
    Self { ty, refinement }
  }
  /// C++ `Inference{ty}`（refinement 默认 nullptr）的具名对应：
  /// 「本次推断不产出 refinement」这一可选返回的空哨兵收口在本类型
  /// 定义处（`RefinementId` 即裸指针句柄），业务调用点不再手写空指针。
  pub fn no_refinement(ty: TypeId) -> Self {
    Self {
      ty,
      refinement: NULL_REFINEMENT_ID,
    }
  }
}

impl Default for Inference {
  fn default() -> Self {
    Self::new()
  }
}
