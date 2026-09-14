use alloc::vec::Vec;

use crate::{
  functions::{begin_type_pack::begin, end_type_pack::end},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
pub fn flatten_type_pack_id(tp: TypePackId) -> (Vec<TypeId>, Option<TypePackId>) {
  let mut res = Vec::new();

  let mut iter = begin(tp);
  let end_iter = end(tp);

  while iter.operator_ne(&end_iter) {
    res.push(*iter.operator_deref());
    iter.operator_inc();
  }

  (res, iter.tail())
}
