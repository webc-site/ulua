/// C++ `static bool allowsNoReturnValues(const TypePackId tp)`.
use crate::type_aliases::type_id::TypeId;
use crate::{
  functions::{
    begin_type_pack::begin, end_type_pack::end, follow_type::follow_type_id,
    get_type_alt_j::get_type_id,
  },
  type_aliases::{error_type::ErrorType, type_pack_id::TypePackId},
};
pub fn allows_no_return_values(tp: TypePackId) -> bool {
  let mut it = begin(tp);
  let end_it = end(tp);

  while it.operator_ne(&end_it) {
    let ty: TypeId = *it.operator_deref();
    let followed_ty = follow_type_id(ty);

    if get_type_id::<ErrorType>(followed_ty).is_none() {
      return false;
    }

    it.operator_inc();
  }

  true
}
