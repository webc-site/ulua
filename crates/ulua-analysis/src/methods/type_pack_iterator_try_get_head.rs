//! C++ `TypePackIterator::tryGetHead`（TypePack.cpp:204-210）。
use crate::{
  records::type_pack_iterator::TypePackIterator, type_aliases::type_pack_id::TypePackId,
};

impl TypePackIterator {
  /// 迭代器恰位于某个（子）类型包的首元素时，直接返回该包。
  ///
  /// C++ 原实现在迭代耗尽状态会返回装着空指针的 optional，调用方
  /// `if (auto t = ...)` 判真后直接返回空句柄；该状态语义上就是
  /// 「没有可复用的头包」，此处统一返回 None，杜绝空句柄外泄。
  pub fn try_get_head(&self) -> Option<TypePackId> {
    if self.current_index == 0 && !self.current_type_pack.is_null() {
      Some(self.current_type_pack)
    } else {
      None
    }
  }
}
