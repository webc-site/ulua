use crate::{
  functions::{
    begin_type_pack::begin_type_pack_id, end_type_pack::end_type_pack_id,
    get_type_pack::get_type_pack_id,
  },
  records::variadic_type_pack::VariadicTypePack,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn first(tp: TypePackId, ignore_hidden_variadics: bool) -> Option<TypeId> {
  let iter = begin_type_pack_id(tp);
  let end_iter = end_type_pack_id(tp);

  if iter.operator_ne(&end_iter) {
    return Some(*iter.operator_deref());
  }

  if let Some(tail) = iter.tail()
    && let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tail)
    && (!vtp.hidden || !ignore_hidden_variadics)
  {
    return Some(vtp.ty);
  }

  None
}
