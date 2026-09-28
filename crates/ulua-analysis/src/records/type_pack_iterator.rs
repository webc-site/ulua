use core::ptr::null;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{internal_compiler_error::InternalCompilerError, txn_log::TxnLog, type_pack::TypePack},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct TypePackIterator {
  pub(crate) current_type_pack: TypePackId,
  pub(crate) tail_cycle_check: TypePackId,
  pub(crate) tp: *const TypePack,
  pub(crate) current_index: usize,
  pub(crate) log: *const TxnLog,
}

impl PartialEq for TypePackIterator {
  fn eq(&self, rhs: &Self) -> bool {
    self.tp == rhs.tp && self.current_index == rhs.current_index
  }
}

impl Eq for TypePackIterator {}

impl TypePackIterator {
  /// 获取当前指向的类型 ID。
  pub fn current(&self) -> &TypeId {
    LUAU_ASSERT!(!self.tp.is_null());
    unsafe {
      let tp = &*self.tp;
      &tp.head[self.current_index]
    }
  }

  /// 向前推进迭代器游标。
  pub fn advance(&mut self) {
    LUAU_ASSERT!(!self.tp.is_null());

    self.current_index += 1;
    // Safety: `&&` 左侧 `!self.tp.is_null()` 短路保证 self.tp 非空；它指向初始化时由
    // `txn_log_get_mutable` 从 log 借出的存活 TypePack（bump arena 已分配块地址不移动），只读 head。
    while !self.tp.is_null() && self.current_index >= unsafe { (*self.tp).head.len() } {
      // Safety: self.tp 仍为上方 while 守卫确认的非空 TypePack 指针（本行尚未重赋值），存活；只读 tail。
      self.current_type_pack = if let Some(tail) = unsafe { (*self.tp).tail } {
        // Safety: self.log 与 self.tp 在 `type_pack_iterator_type_pack_id_txn_log` 中配对赋值，
        // 故 tp 非空即 log 非空；log 为 TxnLog::empty() 进程只读单例或会话活动 log，follow 取 &self。
        unsafe { (*self.log).follow_type_pack_id(tail) }
      } else {
        null()
      };

      self.tp = if !self.current_type_pack.is_null() {
        // Safety: self.log 非空存活（与 self.tp 配对赋值，同上条证成）；txn_log_get_mutable 取
        // &self 只读 log 结构，返回 log/arena 中 TypePack 的可变视图裸指针（本分支 current_type_pack
        // 已判非空）。
        unsafe { (*self.log).txn_log_get_mutable::<TypePack, _>(self.current_type_pack) }
      } else {
        null()
      };

      if !self.tp.is_null() {
        // Step twice on each iteration to detect cycles
        // Safety: 上一行 `!self.tp.is_null()` 已判空，self.tp 为存活 TypePack（同上）；只读 tail。
        self.tail_cycle_check = if let Some(tail) = unsafe { (*self.tp).tail } {
          // Safety: self.log 非空存活（与 tp 配对赋值）；follow_type_pack_id 取 &self 只读。
          unsafe { (*self.log).follow_type_pack_id(tail) }
        } else {
          null()
        };

        if self.current_type_pack == self.tail_cycle_check {
          panic!(
            "{}",
            InternalCompilerError::internal_compiler_error_string_string(
              "TypePackIterator detected a type pack cycle".to_string(),
              "".to_string(),
            )
            .message
          );
        }
      }

      self.current_index = 0;
    }
  }

  /// 后置自增（相当于 C++ `operator++(int)`）。
  pub fn advance_and_get_prev(&mut self) -> Self {
    let copy = self.clone();
    self.advance();
    copy
  }
}

/// 标准迭代器适配：等价于 C++ 的 `it != end; *it; ++it` 三段式循环，
/// 使调用点可以写 `for ty in begin_type_pack::begin(tp)`。
///
/// 注意 `tail()` 语义不变：只能在迭代耗尽（`tp` 为空）后调用，
/// 因此"循环后取 tail"的调用点不能迁到 for-in（迭代器被消费）。
impl Iterator for TypePackIterator {
  type Item = TypeId;

  fn next(&mut self) -> Option<TypeId> {
    // C++ 以 `tp == end 迭代器的 tp（nullptr）` 作为终止条件
    if self.tp.is_null() {
      return None;
    }
    let ty = *self.current();
    self.advance();
    Some(ty)
  }
}
