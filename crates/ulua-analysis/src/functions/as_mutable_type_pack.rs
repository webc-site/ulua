use crate::{records::type_pack_var::TypePackVar, type_aliases::type_pack_id::TypePackId};

/// cpp `asMutable(TypePackId)` 的单一实现（原 alt_d 副本与转发别名已收敛至此）。
pub fn as_mutable_type_pack(tp: TypePackId) -> *mut TypePackVar {
  tp as *mut TypePackVar
}
