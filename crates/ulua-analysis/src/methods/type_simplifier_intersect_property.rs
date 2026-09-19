//! Faithful port of `TypeSimplifier::intersectProperty` (Simplify.cpp:1790-1827).
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{property_type::Property, type_simplifier::TypeSimplifier},
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_property(
    &self,
    target: &Property,
    discriminant: &Property,
    seen: &mut DenseHashSet<TypeId>,
  ) -> Option<Property> {
    // NOTE: I invite the reader to refactor the below code as a fun coding
    // exercise. It looks ugly to me, but I don't think we can make it
    // any cleaner.

    let mut prop = Property {
      deprecated: target.deprecated || discriminant.deprecated,
      ..Default::default()
    };

    // We're trying to follow the following rules for both read and write types:
    // * If the type is present on both properties, intersect it, and return
    //   `None` if we fail.
    // * If the type only exists on one property or the other, take that.

    match (target.read_ty, discriminant.read_ty) {
      (Some(l), Some(r)) => {
        prop.read_ty = self
          .intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(l, r, seen);
        prop.read_ty?;
      }
      (Some(l), None) => prop.read_ty = Some(l),
      (None, Some(r)) => prop.read_ty = Some(r),
      (None, None) => {}
    }

    match (target.write_ty, discriminant.write_ty) {
      (Some(l), Some(r)) => {
        prop.write_ty = self
          .intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(l, r, seen);
        prop.write_ty?;
      }
      (Some(l), None) => prop.write_ty = Some(l),
      (None, Some(r)) => prop.write_ty = Some(r),
      (None, None) => {}
    }

    Some(prop)
  }
}
