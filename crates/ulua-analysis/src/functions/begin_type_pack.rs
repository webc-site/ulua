use crate::{
  records::{txn_log::TxnLog, type_pack_iterator::TypePackIterator},
  type_aliases::type_pack_id::TypePackId,
};

pub fn begin(tp: TypePackId) -> TypePackIterator {
  let mut it = TypePackIterator::new();
  it.type_pack_iterator_type_pack_id(tp);
  it
}

pub(crate) fn begin_type_pack_id_txn_log(tp: TypePackId, log: *const TxnLog) -> TypePackIterator {
  let mut it = TypePackIterator::new();
  // Safety: 本 crate 内该构造的全部调用方传入的 log 或由存活 &TxnLog 指针转换
  // （unifier/checker 的 state.log）、或来自 TxnLog::empty() 的进程级 OnceLock 单例，两者
  // 均非空、对齐且在调用期间只读存活；tp 指向 type arena 中存活的 TypePackVar，被调方法
  // 仅经 &self 查询跟随。it 为栈局对象，&mut 重建无并发借用。
  unsafe { it.type_pack_iterator_type_pack_id_txn_log(tp, log) };
  it
}
