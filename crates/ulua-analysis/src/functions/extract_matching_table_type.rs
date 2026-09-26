use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::relation::Relation,
  functions::{
    begin_type::begin_union_type, follow_type, get_type, relate_simplify::relate_type_id_type_id,
  },
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, free_type::FreeType, table_type::TableType,
    type_ids::TypeIds, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};
pub fn extract_matching_table_type(
  expected_union: &UnionType,
  expr_type: TypeId,
  _builtin_types: Handle<BuiltinTypes>,
) -> Option<TypeId> {
  LUAU_ASSERT!(fflag::LuauBidirectionalInferenceBetterUnionHandling.get());
  let expr_table = get_type::get::<TableType>(follow_type::follow(expr_type))?;

  // Try to filter out tables based on property names, for example
  // if we are considering the type ...
  //
  //  { foo: number, bar: string } | { foo: number, baz: boolean }
  //
  // ... and the table in question looks like ...
  //
  //  { baz = true }
  //
  // ... the user probably intends the second definition.
  let mut potential_tables = TypeIds::new();

  // C++ iterates `for (TypeId ty : expectedUnion)`, which uses
  // `UnionTypeIterator`: each yielded `ty` is already `follow`-ed and
  // nested unions are flattened (`TypeIterator::operator*`/`descend`).
  // Iterating `expected_union.options` directly would hand un-followed
  // bound types to `get<TableType>`, tripping its no-bound-type assert.
  for ty in begin_union_type(expected_union) {
    let tt = get_type::get::<TableType>(ty);
    if let Some(tt) = tt {
      let mut is_disjoint = false;
      // NOTE: We iterate over the expected properties for structural subtyping reasons,
      // consider:
      //
      //  local t: { foo: number? } = {
      //      foo = 42,
      //      -- 10,000 properties not shown.
      //  }
      //
      // Those 10k properties do not matter here.
      for (name, expected_prop) in &tt.props {
        // If the property from the expected type is not in the
        // expression, skip it.
        let prop_in_table_expr = match expr_table.props.get(name) {
          Some(p) => p,
          None => continue,
        };

        // Also, if the expected type does not have a read component, skip this.
        let expected_read_ty = match expected_prop.read_ty {
          Some(t) => t,
          None => continue,
        };

        let expr_prop = prop_in_table_expr;

        // If the expression property doesn't have a read type, then
        // we cannot reasonably check this against the read type of
        // the expected property.
        let expr_read_ty = match expr_prop.read_ty {
          Some(t) => t,
          None => {
            // Also assert here: we should never encounter an inferred
            // write-only type from an expression.
            LUAU_ASSERT!(
              false /* "Unexpected write-only property inside table literal." */
            );
            continue;
          }
        };

        let expected_prop_type = follow_type::follow(expected_read_ty);
        let expr_prop_type = follow_type::follow(expr_read_ty);

        if relate_type_id_type_id(expected_prop_type, expr_prop_type) == Relation::Disjoint {
          is_disjoint = true;
          break;
        }

        let ft = get_type::get::<FreeType>(expr_prop_type);
        if let Some(ft) = ft
          && relate_type_id_type_id(ft.lower_bound, expected_prop_type) == Relation::Disjoint
        {
          is_disjoint = true;
          break;
        }
      }

      if !is_disjoint {
        potential_tables.insert_type_id(ty);
      }
    }
  }

  if potential_tables.size() == 1 {
    return Some(potential_tables.order[0]);
  }

  None
}
