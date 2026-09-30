use crate::{
  functions::{follow_type_pack, get_type_pack::type_pack_variant_of},
  records::{txn_log::TxnLog, type_pack::TypePack},
  type_aliases::{type_pack_id::TypePackId, type_pack_variant::TypePackVariantMember},
};

/// 对应 C++ `size_t size(TypePackId tp, TxnLog* log = nullptr)`
/// （`cpp/Analysis/src/TypePack.cpp:316`）。
///
/// Rust 形态（§2）：C++ 可空 `TxnLog*` 默认形参 → `Option<&TxnLog>`，null 哨兵
/// 消解为 `None`；`tp` 句柄的 arena 存活前提与 `type_pack_variant_of` 同一契约，
/// 全函数只读遍历，无裸指针解引用。
pub fn size(tp: TypePackId, log: Option<&TxnLog>) -> usize {
  let tp = match log {
    Some(log) => log.follow_type_pack_id(tp),
    None => follow_type_pack::follow(tp),
  };
  if let Some(pack) = TypePack::get_if(type_pack_variant_of(tp)) {
    size_type_pack(pack, log)
  } else {
    0
  }
}

/// 对应 C++ `size_t size(const TypePack& tp, TxnLog* log = nullptr)`
/// （`cpp/Analysis/src/TypePack.cpp:338`）。形参契约同 [`size`]。
pub fn size_type_pack(tp: &TypePack, log: Option<&TxnLog>) -> usize {
  let mut result = tp.head.len();
  if let Some(tp_tail) = tp.tail {
    let followed = match log {
      Some(log) => log.follow_type_pack_id(tp_tail),
      None => follow_type_pack::follow(tp_tail),
    };
    if let Some(tail) = TypePack::get_if(type_pack_variant_of(followed)) {
      result += size_type_pack(tail, log);
    }
  }
  result
}
