use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{relation::Relation, table_state::TableState},
  functions::{flip::flip, relate_simplify::relate, relate_table_to_prop::relate_table_to_prop},
  records::table_type::TableType,
  type_aliases::simplifier_seen_set::SimplifierSeenSet,
};

pub fn relate_tables(
  left_table: &TableType,
  right_table: &TableType,
  seen: &mut SimplifierSeenSet,
) -> Relation {
  // FIXME CLI-189216: As noted in the body this is not complete.
  if left_table.state != TableState::Sealed || right_table.state != TableState::Sealed {
    return Relation::Intersects;
  }

  if right_table.props.len() == 1 && right_table.indexer.is_none() {
    let (name, prop) = right_table.props.iter().next().unwrap();
    let res = relate_table_to_prop(left_table, name, prop, seen);
    // If the single property is coincident with the member in the left table, then
    // by width subtyping the left table is a subset.
    if res == Relation::Coincident {
      return Relation::Subset;
    }
    return res;
  }

  if left_table.props.len() == 1 && left_table.indexer.is_none() {
    let (name, prop) = left_table.props.iter().next().unwrap();
    let res = flip(relate_table_to_prop(right_table, name, prop, seen));
    // If the single property is coincident with the member in the right table, then
    // by width subtyping the right table is a subset (so we return superset).
    if res == Relation::Coincident {
      return Relation::Superset;
    }
    return res;
  }

  // This can potentially not account for something like
  //
  //  { x: number, y: number } & { x: number, y: number, z: number }
  //
  // ... where we _ought_ to say superset.
  if left_table.props.len() != right_table.props.len()
    || left_table.indexer.is_some() != right_table.indexer.is_some()
  {
    return Relation::Intersects;
  }

  let mut has_subset = false;

  for (right_name, right_prop) in &right_table.props {
    match relate_table_to_prop(left_table, right_name, right_prop, seen) {
      Relation::Disjoint => return Relation::Disjoint,
      // We're being _very_ conservative here. We could update this in the future to
      // account for a case like:
      //
      //  (T & { x: number }) & (T & { read x: number? })
      //
      // ... by running this loop twice.
      Relation::Superset | Relation::Intersects => return Relation::Intersects,
      Relation::Subset => {
        has_subset = true;
      }
      Relation::Coincident => {}
    }
  }

  if left_table.indexer.is_none() {
    LUAU_ASSERT!(right_table.indexer.is_none());
    return if has_subset {
      Relation::Subset
    } else {
      Relation::Coincident
    };
  }

  let left_indexer = left_table.indexer.as_ref().unwrap();
  let right_indexer = right_table.indexer.as_ref().unwrap();

  if relate(left_indexer.index_type, right_indexer.index_type, seen) != Relation::Coincident {
    return Relation::Intersects;
  }

  if relate(left_indexer.index_type, right_indexer.index_type, seen) != Relation::Coincident {
    return Relation::Intersects;
  }

  if has_subset {
    Relation::Subset
  } else {
    Relation::Coincident
  }
}
