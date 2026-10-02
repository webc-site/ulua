use core::ptr::{NonNull, null};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    arena_handle::{alias_nn_ref, alias_ref},
    internal_compiler_error::InternalCompilerError,
    txn_log::TxnLog,
    type_pack::TypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct TypePackIterator {
  pub(crate) current_type_pack: TypePackId,
  pub(crate) tail_cycle_check: TypePackId,
  pub(crate) tp: Option<NonNull<TypePack>>,
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
    LUAU_ASSERT!(self.tp.is_some());
    &alias_nn_ref(self.tp.expect("LUAU_ASSERT 判 pack 迭代器未耗尽")).head[self.current_index]
  }

  /// 向前推进迭代器游标。
  pub fn advance(&mut self) {
    LUAU_ASSERT!(self.tp.is_some());

    self.current_index += 1;
    while self.tp.is_some() && self.current_index >= alias_nn_ref(self.tp.unwrap()).head.len() {
      self.current_type_pack = if let Some(tail) = alias_nn_ref(self.tp.unwrap()).tail {
        alias_ref(self.log).follow_type_pack_id(tail)
      } else {
        null()
      };

      self.tp = if !self.current_type_pack.is_null() {
        alias_ref(self.log).txn_log_get_mutable::<TypePack, _>(self.current_type_pack)
      } else {
        None
      };

      if let Some(tp_nn) = self.tp {
        // Step twice on each iteration to detect cycles
        self.tail_cycle_check = if let Some(tail) = alias_nn_ref(tp_nn).tail {
          alias_ref(self.log).follow_type_pack_id(tail)
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
/// 注意 `tail()` 语义不变：只能在迭代耗尽（`tp` 为 `None`）后调用，
/// 因此"循环后取 tail"的调用点不能迁到 for-in（迭代器被消费）。
impl Iterator for TypePackIterator {
  type Item = TypeId;

  fn next(&mut self) -> Option<TypeId> {
    // C++ 以 `tp == end 迭代器的 tp（nullptr）` 作为终止条件
    self.tp?;
    let ty = *self.current();
    self.advance();
    Some(ty)
  }
}
