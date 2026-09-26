use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::relation::Relation,
  functions::{
    lookup_extern_type_prop::lookup_extern_type_prop, relate_simplify::relate_type_id_type_id,
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
      // Safety: 紧邻 LUAU_ASSERT 钉住双侧 read_ty 为 Some（extern 属性必带读类型）。
      match relate_type_id_type_id(
        prop
          .read_ty
          .expect("cpp LUAU_ASSERT(prop.readTy)：紧邻断言钉住必为 Some"),
        prop_in_extern_type
          .read_ty
          .expect("cpp LUAU_ASSERT(prop.readTy)：紧邻断言钉住必为 Some"),
      ) {
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
