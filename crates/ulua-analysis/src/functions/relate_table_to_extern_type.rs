use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::relation::Relation,
  functions::{
    lookup_extern_type_prop::lookup_extern_type_prop, relate_simplify_alt_b::relate_type_id_type_id,
  },
  records::{extern_type::ExternType, table_type::TableType},
  type_aliases::simplifier_seen_set::SimplifierSeenSet,
};

pub fn relate_table_to_extern_type(
  table: &TableType,
  cls: &ExternType,
  _seen: &mut SimplifierSeenSet,
) -> Relation {
  if table.indexer.is_some() || cls.indexer.is_some() {
    return Relation::Intersects;
  }
  for (name, prop) in &table.props {
    if let Some(prop_in_extern_type) = lookup_extern_type_prop(cls, name) {
      LUAU_ASSERT!(prop.read_ty.is_some() && prop_in_extern_type.read_ty.is_some());
      match relate_type_id_type_id(prop.read_ty.unwrap(), prop_in_extern_type.read_ty.unwrap()) {
        Relation::Disjoint => return Relation::Disjoint,
        Relation::Coincident => {}
        Relation::Intersects => return Relation::Intersects,
        Relation::Subset => return Relation::Intersects,
        Relation::Superset => {}
      }
    }
  }
  Relation::Superset
}
