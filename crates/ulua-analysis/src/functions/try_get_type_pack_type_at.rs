use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type_pack::follow_type_pack_id,
    get_type_pack::get_type_pack_id,
  },
  records::variadic_type_pack::VariadicTypePack,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn try_get_type_pack_type_at(tp: TypePackId, index: usize) -> Option<TypeId> {
  let (tp_head, tp_tail) = flatten_type_pack_id(tp);

  if index < tp_head.len() {
    return Some(tp_head[index]);
  }

  let tp_tail_id = tp_tail?;
  let follow_tp = unsafe { follow_type_pack_id(tp_tail_id) };
  get_type_pack_id::<VariadicTypePack>(follow_tp).map(|vtp| vtp.ty)
}
