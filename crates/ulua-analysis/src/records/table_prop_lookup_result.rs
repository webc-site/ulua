use alloc::vec::Vec;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TablePropLookupResult {
  /// What types are we blocked on for determining this type?
  pub blocked_types: Vec<TypeId>,
  /// The type of the property (if we were able to determine it).
  pub prop_type: Option<TypeId>,
  /// Whether or not this is _definitely_ derived as the result of an indexer.
  /// We use this to determine whether or not code like:
  ///
  ///   t.lol = nil;
  ///
  /// ... is legal. If `t: { [string]: ~nil }` then this is legal as
  /// there's no guarantee on whether "lol" specifically exists.
  /// However, if `t: { lol: ~nil }`, then we cannot allow assignment as
  /// that would remove "lol" from the table entirely.
  pub is_index: bool,
}

impl TablePropLookupResult {
  #[inline]
  pub const fn empty() -> Self {
    Self {
      blocked_types: Vec::new(),
      prop_type: None,
      is_index: false,
    }
  }

  #[inline]
  pub const fn found(ty: TypeId) -> Self {
    Self {
      blocked_types: Vec::new(),
      prop_type: Some(ty),
      is_index: false,
    }
  }

  #[inline]
  pub const fn found_opt(ty: Option<TypeId>) -> Self {
    Self {
      blocked_types: Vec::new(),
      prop_type: ty,
      is_index: false,
    }
  }

  #[inline]
  pub const fn index(ty: TypeId) -> Self {
    Self {
      blocked_types: Vec::new(),
      prop_type: Some(ty),
      is_index: true,
    }
  }

  #[inline]
  pub fn blocked_on(ty: TypeId) -> Self {
    Self {
      blocked_types: alloc::vec![ty],
      prop_type: None,
      is_index: false,
    }
  }

  #[inline]
  pub fn blocked_index(ty: TypeId) -> Self {
    Self {
      blocked_types: alloc::vec![ty],
      prop_type: None,
      is_index: true,
    }
  }

  #[inline]
  pub fn blocked(blocked_types: Vec<TypeId>) -> Self {
    Self {
      blocked_types,
      prop_type: None,
      is_index: false,
    }
  }
}

/// # Safety
///
/// `blocked_types: Vec<TypeId>` 与 `prop_type: Option<TypeId>` 内含
/// `*const Type` 借用，令自动 Send 失效。这些 id 仅作类型 arena 的身份、从不
/// 解引用；只要 arena 在 `TablePropLookupResult` 存活期内有效，转移即可靠。
// Safety: 所有字段仅承载 arena 类型身份（`TypeId = *const Type` 与 bool），本结构从
// 不解引用它们；将该值转移到其它线程时，只要其所属 type arena 在 TablePropLookupResult
// 存活期内保持有效且无并发访问者改写 arena 不变量，转移所有权即可靠，故满足 Send。
unsafe impl Send for TablePropLookupResult {}
/// # Safety
///
/// 同上：裸 `TypeId` 仅只读身份，`is_index` 为 `bool`，共享 `&` 引用不产生数据
/// 竞争。
// Safety: 多线程共享 `&TablePropLookupResult` 只读取按值的类型身份 id 与 bool，从不对
// 裸指针解引用、无内部可变状态，故并发只读共享不产生数据竞争，满足 Sync。
unsafe impl Sync for TablePropLookupResult {}
