//! Source: `Analysis/src/Normalize.cpp:2430-2504` (hand-ported)
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{normalization_result::NormalizationResult, relation::Relation},
  functions::{
    get_mutable_type::get_mutable, get_type_alt_j::get_type_id,
    relate_simplify_alt_b::relate_type_id_type_id,
  },
  records::{
    extern_type::ExternType, normalized_extern_type::NormalizedExternType, normalizer::Normalizer,
    table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};
impl Normalizer {
  pub fn intersect_extern_types_with_shape(
    &mut self,
    heres: &mut NormalizedExternType,
    there: TypeId,
  ) {
    LUAU_ASSERT!(FFlag::LuauExternTypesNormalizeWithShapes.get());

    self.consume_fuel();

    // in this case, we want to take the foreign function types we have here, and we want to intersect a table type into them.
    // the idea here is that table types function as structural definitions for the shape of some data type.

    let shape = get_type_id::<TableType>(there);

    // if the type we're intersecting with isn't a table type, it can't be used to describe a shape.
    let Some(shape) = shape else {
      return;
    };

    // we need to check that every property in the shape is compatible with the main extern type, `hereTy`.
    // if any intersection of their property types is uninhabited, then the whole thing is uninhabited.
    // but if the type is inhabited, we'll want to add it to the shapes on the externtype.

    let mut is_coincident = true;
    for (name, shape_prop) in &shape.props {
      for &here_ty in heres.ordering.iter() {
        // TODO: do we need to take into account any of the negations here as well for the compatibility check?

        // ordering 只存 ExternType（C++ 此处必命中），None 时跳过即可。
        let Some(extern_ty) = get_mutable::<ExternType>(here_ty) else {
          continue;
        };

        let found = extern_ty.props.get(name);
        // if the property isn't present, we can move onto the next property since it's a fine extension.
        let prop = match found {
          None => {
            is_coincident = false;
            continue;
          }
          Some(p) => p.clone(),
        };

        // if the property is present, we need to check that the two properties are compatible.
        // if they're not, then the whole thing is `never`, as with other incompatible intersections.

        // if they both have read properties, we have to check that the intersection of those read properties is inhabited
        if let (Some(prop_read), Some(shape_read)) = (prop.read_ty, shape_prop.read_ty) {
          // if the intersection is uninhabited, then we can reset to `never` and we're done.
          if self.is_intersection_inhabited_type_id_type_id(prop_read, shape_read)
            != NormalizationResult::True
          {
            heres.reset_to_never();
            return;
          }

          if relate_type_id_type_id(prop_read, shape_read) != Relation::Coincident {
            is_coincident = false;
          }
        }

        // if they both have write properties, we also want to check if they're coincident.
        // unlike with read types, we don't have to check that the intersection is inhabited because
        // even if the types don't overlap, something like `{ write prop: string } & { write prop: number }`
        // behaviorally suggests that we could write `string` or `number` into `prop`.
        if let (Some(prop_write), Some(shape_write)) = (prop.write_ty, shape_prop.write_ty) {
          // if the types aren't coincident, then the whole shapes can't be coincident
          if relate_type_id_type_id(prop_write, shape_write) != Relation::Coincident {
            is_coincident = false;
          }
        }
      }
    }

    // if we've made it here, then we should've validated that the intersection between the shape and the extern type is legal, so we can add it to
    // the collection of shapes.
    // TODO: we may want to look into some more deduplication here.
    if !is_coincident {
      heres.shape_extensions.insert_type_id(there);
    }
  }
}
