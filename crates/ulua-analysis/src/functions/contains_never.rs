//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:440:containsNever`
//! Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:440-459, hand-ported)

use crate::{
  functions::{
    begin_type_pack::begin_type_pack_id, end_type_pack::end, follow_type::follow_type_id,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id,
  },
  records::{never_type::NeverType, variadic_type_pack::VariadicTypePack},
  type_aliases::type_pack_id::TypePackId,
};

pub fn contains_never(tp: TypePackId) -> bool {
  let mut it = begin_type_pack_id(tp);
  let end_it = end(tp);

  while it.operator_ne(&end_it) {
    if get_type_id::<NeverType>(follow_type_id(*it.operator_deref())).is_some() {
      return true;
    }
    it.operator_inc();
  }

  if let Some(tail) = it.tail()
    && let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tail)
    && get_type_id::<NeverType>(follow_type_id(vtp.ty)).is_some()
  {
    return true;
  }

  false
}
