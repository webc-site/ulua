use core::ptr::NonNull;

use crate::{functions::fresh_index::fresh_index, records::constraint::Constraint};

#[derive(Debug, Clone)]
pub struct BlockedTypePack {
  pub(crate) index: usize,
  /// §2：cpp `Constraint* owner = nullptr`（TypePack.h:93）——本 blocked pack 归属
  /// 的约束回指，是类型存根被哪个约束「阻塞」的身份句柄。目标在约束 arena 中由
  /// `ConstraintSet` 持有、生命周期不受 pack 借用约束，故按「被 arena 持有的回指」
  /// 建模为 `Option<NonNull<Constraint>>`（`None` 即分配期尚未接线，`Some` 编码非空）。
  /// 全仓该字段只作指针同一性读写、从不解引用（`canMutate` 断言非空后 `==` 比较），
  /// 故 `NonNull` 的地址同一性语义即完整覆盖，无需 chokepoint、裸指针不外渗。
  pub(crate) owner: Option<NonNull<Constraint>>,
}

impl BlockedTypePack {
  pub fn new() -> Self {
    Self {
      index: fresh_index() as usize,
      owner: None,
    }
  }
}

impl Default for BlockedTypePack {
  fn default() -> Self {
    Self::new()
  }
}
