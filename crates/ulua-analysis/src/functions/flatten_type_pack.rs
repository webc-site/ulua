use alloc::vec::Vec;

use crate::{
  functions::{
    begin_type_pack::begin,
    end_type_pack::{end, end_type_pack_id},
  },
  records::{txn_log::TxnLog, type_pack_iterator::TypePackIterator},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn flatten_type_pack_id(tp: TypePackId) -> (Vec<TypeId>, Option<TypePackId>) {
  let mut res = Vec::new();

  let mut iter = begin(tp);
  let end_iter = end(tp);

  while iter != end_iter {
    res.push(*iter.current());
    iter.advance();
  }

  (res, iter.tail())
}

pub(crate) fn flatten(tp: TypePackId, log: &TxnLog) -> (Vec<TypeId>, Option<TypePackId>) {
  let tp = log.follow_type_pack_id(tp);
  let mut flattened = Vec::new();
  let mut it = TypePackIterator::new();
  // Safety: `log` 是存活的 `&TxnLog`，`log as *const TxnLog` 保持非空、对齐并指向该引用；
  // `tp` 经上一行 `log.follow_type_pack_id(tp)` 收敛后指向 type arena 中存活的
  // TypePackVar（bump 分配、地址不移动）。被调 unsafe fn 的前置条件（有效 log 指针
  // + 存活 tp）均满足，且本语句内无并发可变借用。
  unsafe { it.type_pack_iterator_type_pack_id_txn_log(tp, log as *const TxnLog) };

  while it != end_type_pack_id(tp) {
    flattened.push(*it.current());
    it.advance();
  }

  let tail = it.tail();
  (flattened, tail)
}
