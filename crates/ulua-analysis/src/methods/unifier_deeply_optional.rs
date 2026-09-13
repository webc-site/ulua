use core::ptr::null;
use std::collections::HashMap;

use crate::{
  functions::{
    follow_type::follow_type_id, get_mutable_type::get_mutable_type_id,
    get_type_alt_j::get_type_id, is_optional::is_optional,
  },
  records::{table_type::TableType, unifier::Unifier, union_type::UnionType},
  type_aliases::type_id::TypeId,
};
impl Unifier {
  pub fn unifier_deeply_optional(
    &mut self,
    mut ty: TypeId,
    seen: &mut HashMap<TypeId, TypeId>,
  ) -> TypeId {
    ty = follow_type_id(ty);

    if is_optional(ty) {
      return ty;
    }

    if let Some(ttv) = get_type_id::<TableType>(ty) {
      if let Some(&result) = seen.get(&ty) {
        return result;
      }

      // Add the table type to the arena first to get a result TypeId
      let result = unsafe { (*self.types).add_type(ttv.clone()) };

      // Store the result in seen map to handle cycles
      seen.insert(ty, result);

      let result_ttv = get_mutable_type_id::<TableType>(result).unwrap();

      // Recursively make each property deeply optional
      for prop in result_ttv.props.values_mut() {
        let prop_ty = prop.read_ty.or(prop.write_ty).unwrap_or(null());
        let new_prop_ty = self.unifier_deeply_optional(prop_ty, seen);
        prop.read_ty = Some(new_prop_ty);
        prop.write_ty = Some(new_prop_ty);
      }

      // Return nil | result
      let builtin_types = unsafe { &*self.builtin_types };
      let union_types = alloc::vec![builtin_types.nil_type, result];
      let union_type = UnionType {
        options: union_types,
      };
      unsafe { (*self.types).add_type(union_type) }
    } else {
      // Return nil | ty
      let builtin_types = unsafe { &*self.builtin_types };
      let union_types = alloc::vec![builtin_types.nil_type, ty];
      let union_type = UnionType {
        options: union_types,
      };
      unsafe { (*self.types).add_type(union_type) }
    }
  }
}
