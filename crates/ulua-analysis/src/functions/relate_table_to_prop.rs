use crate::{
  enums::relation::Relation,
  functions::relate_simplify::relate,
  records::{property_type::Property, table_type::TableType},
  type_aliases::simplifier_seen_set::SimplifierSeenSet,
};

/// @return The relationship between the single property on the right and its corresponding property
/// in the left table.
pub fn relate_table_to_prop(
  left_table: &TableType,
  prop_name: &str,
  right_prop: &Property,
  seen: &mut SimplifierSeenSet,
) -> Relation {
  // If the left table does not have the property at all,
  // assume an intersection.
  let left_prop = match left_table.props.get(prop_name) {
    Some(p) => p,
    None => return Relation::Intersects,
  };

  if left_prop.is_shared() && right_prop.is_shared() {
    match relate(
      left_prop.read_ty.unwrap(),
      right_prop.read_ty.unwrap(),
      seen,
    ) {
      // The two read properties are disjoint, so the tables are disjoint, e.g.:
      //
      //  { y: string, x: number? } & { read x: string }
      //
      Relation::Disjoint => Relation::Disjoint,
      Relation::Coincident => Relation::Coincident,
      // For _all_ other cases, two shared properties indicate a non-empty intersection.
      //
      // Subset:     { y: string, x: number } & { x: number? }   (widened write type)
      // Superset:   { y: string, x: number? } & { x: number }   (narrowed read type)
      // Intersects: { y: string, x: number? } & { x: string? }  (both)
      Relation::Subset | Relation::Superset | Relation::Intersects => Relation::Intersects,
    }
  } else {
    // Otherwise we want to hard match on the case of:
    //
    //  { ..., x: T } & { read x: U }
    //
    // ... or ...
    //
    //  { ..., read x: T } & { read x: U }
    //
    // We will use the relation between T and U here.
    if left_prop.read_ty.is_none() || !right_prop.is_read_only() {
      return Relation::Intersects;
    }

    match relate(
      left_prop.read_ty.unwrap(),
      right_prop.read_ty.unwrap(),
      seen,
    ) {
      // The two read properties are disjoint, so the tables are disjoint, e.g.:
      //
      //  { y: string, x: number? } & { read x: string }
      //
      Relation::Disjoint => Relation::Disjoint,
      // If the two read types are coincident, then the left property is a
      // subset if it also has a write part.
      Relation::Coincident => {
        if left_prop.write_ty.is_some() {
          Relation::Subset
        } else {
          Relation::Coincident
        }
      }
      // If the left table's property is a subset of the right property, then
      // the left table is a subset, as in:
      //
      //  { y: string, x: number } & { read x: number? } => these tables intersect
      Relation::Subset => Relation::Subset,
      // If the left table's property is a superset of the right property, then
      // the two tables intersect, as in:
      //
      //  { y: string, x: number? } & { read x: number }
      //
      Relation::Superset => Relation::Intersects,
      // If the left table's property intersects with the right property, then
      // the two tables intersect, as in:
      //
      //  { y: string, x: number? } & { read x: string? } => these tables intersect
      Relation::Intersects => Relation::Intersects,
    }
  }
}
