use crate::{
  functions::{begin_type_pack::begin, end_type_pack::end_type_pack_id, get_type_pack},
  records::variadic_type_pack::VariadicTypePack,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn first(tp: TypePackId, ignore_hidden_variadics: bool) -> Option<TypeId> {
  let iter = begin(tp);
  let end_iter = end_type_pack_id(tp);

  if iter != end_iter {
    return Some(*iter.current());
  }

  if let Some(tail) = iter.tail()
    && let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tail)
    && (!vtp.hidden || !ignore_hidden_variadics)
  {
    return Some(vtp.ty);
  }

  None
}
