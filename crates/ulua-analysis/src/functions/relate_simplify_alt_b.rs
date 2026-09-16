use core::ptr::null;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::relation::Relation, functions::relate_simplify::relate, type_aliases::type_id::TypeId,
};
pub fn relate_type_id_type_id(left: TypeId, right: TypeId) -> Relation {
  let mut seen = DenseHashMap::new((null(), null()));
  relate(left, right, &mut seen)
}
