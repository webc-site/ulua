//! `type_pack_iterator` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::ptr::{null, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{txn_log::TxnLog, type_pack::TypePack, type_pack_iterator::TypePackIterator},
  type_aliases::type_pack_id::TypePackId,
};

impl TypePackIterator {
  pub fn tail(&self) -> Option<TypePackId> {
    LUAU_ASSERT!(self.tp.is_null());
    if !self.current_type_pack.is_null() {
      Some(self.current_type_pack)
    } else {
      None
    }
  }
}

// C++ `TypePackIterator::tryGetHead`（TypePack.cpp:204-210）。

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

impl TypePackIterator {
  pub fn new() -> Self {
    // TypePackId is currently a stub type; null sentinel deferred until TypePackId = *const TypePackVar
    let null_tp: TypePackId = Default::default();
    Self {
      current_type_pack: null_tp,
      tail_cycle_check: null_tp,
      tp: null(),
      current_index: 0,
      log: null(),
    }
  }
}
impl Default for TypePackIterator {
  fn default() -> Self {
    Self::new()
  }
}
impl TypePackIterator {
  pub(crate) fn type_pack_iterator_type_pack_id(&mut self, _type_pack: TypePackId) {
    // Safety: 唯一传入的裸指针是 `TxnLog::empty()`——进程级非空单例（永不释放、
    // 构造后只读），故被调方 `(*log)` 解引用与 follow/get 读取的前置条件成立。
    unsafe { self.type_pack_iterator_type_pack_id_txn_log(_type_pack, TxnLog::empty()) };
  }

  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn type_pack_iterator_type_pack_id_txn_log(
    &mut self,
    type_pack: TypePackId,
    log: *const TxnLog,
  ) {
    // Safety: log 由本方法唯一调用方传入 TxnLog::empty() 进程级非空单例，指向
    // 存活且构造后只读的 TxnLog；follow_type_pack_id 仅经 &self 读取。
    self.current_type_pack = unsafe { (*log).follow_type_pack_id(type_pack) };
    // Safety: 同上 log 非空存活；txn_log_get_mutable 经 &self 只读查询，返回指向
    // 持久 arena/log 存储的 *mut TypePack，不在此处解引用。
    self.tp = unsafe { (*log).txn_log_get_mutable::<TypePack, TypePackId>(self.current_type_pack) };
    self.current_index = 0;
    self.log = log;

    // Safety: 短路求值保证进入右侧时 self.tp 非空；tp 由 txn_log_get_mutable 得到，
    // 指向 arena 中存活的 TypePack（块地址不移动、遍历期内有效），只读 head 字段。
    while !self.tp.is_null() && unsafe { (*self.tp).head.is_empty() } {
      // Safety: while 条件 `!self.tp.is_null()` 在循环体内成立，tp 指向存活
      // TypePack；按 repr(C) 只读 tail 字段。
      self.current_type_pack = if let Some(tail) = unsafe { (*self.tp).tail } {
        // Safety: log 非空存活（TxnLog 单例），follow 只读。
        unsafe { (*log).follow_type_pack_id(tail) }
      } else {
        null()
      };

      self.tp = if !self.current_type_pack.is_null() {
        // Safety: log 非空存活；txn_log_get_mutable 只读查询返回存活 arena 节点指针。
        unsafe { (*log).txn_log_get_mutable::<TypePack, TypePackId>(self.current_type_pack) }
      } else {
        null_mut()
      };
    }
  }
}
