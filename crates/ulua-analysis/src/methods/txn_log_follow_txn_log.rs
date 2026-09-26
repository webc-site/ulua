use crate::{
  enums::follow_option::FollowOption,
  functions::{
    follow_type::{FollowMapper, follow_full},
    follow_type_pack::{FollowPackMapper, follow_pack_full},
  },
  records::txn_log::TxnLog,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TxnLog {
  /// C++ `TxnLog::follow(TypeId)`：follow 链上叠加本 log 的 pending 重定向。
  /// 映射器经 [`FollowMapper::Log`] 传入，context 裸指针往返的 unsafe 已消除。
  pub fn follow_type_id(&self, ty: TypeId) -> TypeId {
    follow_full(ty, FollowOption::Normal, FollowMapper::Log(self))
  }

  /// C++ `TxnLog::follow(TypePackId)`：follow 链上叠加本 log 的 pending 重定向。
  pub fn follow_type_pack_id(&self, tp: TypePackId) -> TypePackId {
    follow_pack_full(tp, FollowPackMapper::Log(self))
  }
}
