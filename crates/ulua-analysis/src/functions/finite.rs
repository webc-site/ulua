//! Source: `Analysis/src/TypePack.cpp:317-328` (hand-ported)

use crate::{
  functions::{follow_type_pack, get_type_pack::type_pack_variant_of},
  records::{txn_log::TxnLog, type_pack::TypePack, variadic_type_pack::VariadicTypePack},
  type_aliases::{type_pack_id::TypePackId, type_pack_variant::TypePackVariantMember},
};

/// 对应 C++ `bool finite(TypePackId tp, TxnLog* log = nullptr)`
/// （`cpp/Analysis/src/TypePack.cpp:317`，同先例姊妹函数 [`size`]，见
/// `size_type_pack.rs`）。
///
/// Rust 形态（§2）：C++ 可空 `TxnLog*` 默认形参 → `Option<&TxnLog>`，全仓调用点
/// 的 null 哨兵（`null_mut()`/`NO_TXN_LOG`）消解为 `None`；`tp` 句柄的 arena 存活
/// 前提与 `type_pack_variant_of` 同一契约，全函数只读遍历，无裸指针解引用。
pub fn finite(tp: TypePackId, log: Option<&TxnLog>) -> bool {
  let tp = match log {
    Some(log) => log.follow_type_pack_id(tp),
    None => follow_type_pack::follow(tp),
  };

  if let Some(pack) = TypePack::get_if(type_pack_variant_of(tp)) {
    return match pack.tail {
      // tail 为同一 arena 存活的 pack 句柄；log 原样透传，只读遍历。
      Some(tail) => finite(tail, log),
      None => true,
    };
  }

  VariadicTypePack::get_if(type_pack_variant_of(tp)).is_none()
}
