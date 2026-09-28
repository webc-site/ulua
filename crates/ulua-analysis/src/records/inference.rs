//!
//! B 型（可空句柄字段的定义处收口）：`ty`/`refinement` 分别是 type_aliases 声明
//! 的可空 arena 句柄（`TypeId` 可为空、`RefinementId` 见 cpp Refinement.h:22
//! "(can be null)"）。本文件是这两个空值的构造/收口处：`dense_default` 仅作
//! `DenseHashMap` 空槽填充值、消费方永不可见；`new`/`no_refinement` 分别对应
//! cpp `Inference{}` 与 `Inference{ty}` 的聚合初始化缺省，语义为「无类型/
//! 无 refinement」，读取方（check_* 系列）按 is_null 分支处理。
use core::ptr::{null, null_mut};

use ulua_common::records::dense_hash_table::DenseDefault;

use crate::type_aliases::{refinement_id_refinement::RefinementId, type_id::TypeId};
#[derive(Debug, Clone)]
pub struct Inference {
  pub ty: TypeId,
  pub refinement: RefinementId,
}

impl DenseDefault for Inference {
  fn dense_default() -> Self {
    Self {
      ty: null(),
      refinement: null_mut(),
    }
  }
}

impl Inference {
  // C++: `Inference()` — `ty` default-initialized to nullptr, no refinement.
  // (Analysis/include/Luau/ConstraintGenerator.h)
  pub fn new() -> Self {
    Self {
      ty: null(),
      refinement: null_mut(),
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
      refinement: null_mut(),
    }
  }
}

impl Default for Inference {
  fn default() -> Self {
    Self::new()
  }
}
