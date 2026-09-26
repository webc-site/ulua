use alloc::vec::Vec;
use core::ptr::null_mut;

use crate::{
  enums::unify_result::UnifyResult,
  type_aliases::{constraint_v::ConstraintV, upper_bounds::UpperBounds},
};
#[derive(Debug, Clone)]
pub struct Result {
  pub unified: UnifyResult,
  pub outstanding_constraints: Vec<ConstraintV>,
  pub upper_bound_contributors: UpperBounds,
}

impl Default for Result {
  fn default() -> Self {
    Self {
      unified: UnifyResult::Ok,
      outstanding_constraints: Vec::new(),
      upper_bound_contributors: UpperBounds::new(null_mut()),
    }
  }
}

/// # Safety
///
/// `outstanding_constraints: Vec<ConstraintV>` 与 `upper_bound_contributors`
/// （`DenseHashMap<TypeId, Vec<(Location, TypeId)>>`）内含借用自约束/类型 arena
/// 的 `TypeId`/约束裸指针，令自动 Send 失效。这些指针仅身份/查找、从不解引用或
/// 释放；只要所属 arena 在 `Result` 存活期内有效，转移即可靠。
unsafe impl Send for Result {}
/// # Safety
///
/// 同上：内嵌 arena id 与约束借用只被只读访问，共享 `&Result` 在底层 arena 有效
/// 期间不产生数据竞争。
unsafe impl Sync for Result {}
