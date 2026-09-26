//! Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:440-459, hand-ported)

use crate::{
  functions::{begin_type_pack::begin, end_type_pack::end, follow_type, get_type, get_type_pack},
  records::{never_type::NeverType, variadic_type_pack::VariadicTypePack},
  type_aliases::type_pack_id::TypePackId,
};

pub fn contains_never(tp: TypePackId) -> bool {
  let mut it = begin(tp);
  let end_it = end(tp);

  while it != end_it {
    if get_type::get::<NeverType>(follow_type::follow(*it.current())).is_some() {
      return true;
    }
    it.advance();
  }

  if let Some(tail) = it.tail()
    && let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tail)
    && get_type::get::<NeverType>(follow_type::follow(vtp.ty)).is_some()
  {
    return true;
  }

  false
}
