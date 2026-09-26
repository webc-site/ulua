//! Source: `Analysis/src/TypePack.cpp:369-378` (hand-ported)

use crate::{
  functions::{follow_type_pack, get_type_pack::type_pack_variant_of},
  records::type_pack::TypePack,
  type_aliases::{type_pack_id::TypePackId, type_pack_variant::TypePackVariantMember},
};

pub(crate) fn is_empty(tp: TypePackId) -> bool {
  // pack 变体读取收口在 `type_pack_variant_of`（arena 节点有效性契约同 C++
  // `get(TypePackId)`：follow 结果仍是 arena 内存活句柄，`TypePack::get_if`
  // 命中后 head/tail 借用随该节点存活。递归仅沿 tail 下行、有向无环）。
  let tp = follow_type_pack::follow(tp);
  if let Some(tpp) = TypePack::get_if(type_pack_variant_of(tp)) {
    return tpp.head.is_empty()
      && match tpp.tail {
        Some(tail) => is_empty(tail),
        None => true,
      };
  }

  false
}
