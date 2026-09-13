use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::{
    follow_type::follow_type_id, get_mutable_type::get_mutable, get_type_alt_j::get_type_id,
    is_approximately_falsy_type::is_approximately_falsy_type,
    is_approximately_truthy_type::is_approximately_truthy_type, is_type_variable::is_type_variable,
    shallow_clone_clone_alt_b::shallow_clone,
  },
  records::{
    any_type::AnyType, clone_state::CloneState, error_type::ErrorType,
    intersection_type::IntersectionType, negation_type::NegationType, never_type::NeverType,
    table_type::TableType, type_function_instance_type::TypeFunctionInstanceType,
    type_ids::TypeIds, type_simplifier::TypeSimplifier, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
    &self,
    target: TypeId,
    discriminant: TypeId,
    seen: &mut DenseHashSet<TypeId>,
  ) -> Option<TypeId> {
    let builtin_types = unsafe { &*self.builtin_types };
    if seen.contains(&target) {
      return None;
    }
    let target = follow_type_id(target);
    let discriminant = follow_type_id(discriminant);
    if seen.contains(&target) {
      return None;
    }

    if let Some(ut) = get_type_id::<UnionType>(target) {
      seen.insert(target);
      let mut options = TypeIds::new();
      for &option in &ut.options {
        let res = self.intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
          option,
          discriminant,
          seen,
        )?;
        if get_type_id::<UnknownType>(res).is_some() {
          return Some(builtin_types.unknown_type);
        }
        if get_type_id::<NeverType>(res).is_some() {
          continue;
        }
        options.insert_type_id(res);
      }
      if options.empty() {
        return Some(builtin_types.never_type);
      }
      if options.size() == 1 {
        return Some(options.front());
      }
      let arena = unsafe { &mut *self.arena.cast_mut() };
      return Some(arena.add_type(UnionType {
        options: options.take(),
      }));
    }

    if let Some(it) = get_type_id::<IntersectionType>(target) {
      seen.insert(target);
      let mut parts = TypeIds::new();
      for &part in &it.parts {
        let res = self.intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
          part,
          discriminant,
          seen,
        )?;

        if get_type_id::<NeverType>(res).is_some() {
          return Some(builtin_types.never_type);
        }

        if let Some(sub_intersection) = get_type_id::<IntersectionType>(res) {
          for &sub_option in &sub_intersection.parts {
            if get_type_id::<NeverType>(sub_option).is_some() {
              return Some(builtin_types.never_type);
            }
            if get_type_id::<UnknownType>(sub_option).is_none() {
              parts.insert_type_id(sub_option);
            }
          }
        } else if get_type_id::<UnknownType>(res).is_none() {
          parts.insert_type_id(res);
        }
      }

      if parts.empty() {
        return Some(builtin_types.unknown_type);
      }
      if parts.size() == 1 {
        return Some(parts.front());
      }
      let arena = unsafe { &mut *self.arena.cast_mut() };
      return Some(arena.add_type(IntersectionType {
        parts: parts.take(),
      }));
    }

    if let Some(ttv) = get_type_id::<TableType>(target)
      && let Some(disc_ttv) = get_type_id::<TableType>(discriminant)
    {
      // The precondition of this function is that `discriminant` is
      // simple, so if it's a table it *must* be a sealed table with
      // a single property and no indexer.
      LUAU_ASSERT!(disc_ttv.props.len() == 1 && disc_ttv.indexer.is_none());
      let (disc_prop_name, disc_prop) = disc_ttv.props.iter().next().unwrap();
      if let Some(ty_prop) = ttv.props.get(disc_prop_name) {
        let property = self.intersect_property(ty_prop, disc_prop, seen)?;
        if let Some(read_ty) = property.read_ty
          && get_type_id::<NeverType>(follow_type_id(read_ty)).is_some()
        {
          return Some(builtin_types.never_type);
        }
        if let Some(write_ty) = property.write_ty
          && get_type_id::<NeverType>(follow_type_id(write_ty)).is_some()
        {
          return Some(builtin_types.never_type);
        }

        // If the property we get back is pointer identical to the
        // original property, return the underlying property as an
        // optimization.
        if ty_prop.read_ty == property.read_ty && ty_prop.write_ty == property.write_ty {
          return Some(target);
        }

        let mut cs = CloneState::new(unsafe { &mut *self.builtin_types.cast_mut() });
        let result = unsafe {
          shallow_clone(
            target,
            &mut *self.arena.cast_mut(),
            &mut cs,
            /* clonePersistentTypes */ true,
          )
        };
        // C++: LUAU_ASSERT(resultTtv)
        let result_ttv = get_mutable::<TableType>(result).expect("shallow_clone yields TableType");
        result_ttv.props.insert(disc_prop_name.clone(), property);
        // Shallow cloning clears out scopes, so let's put back the
        // scope from the original type.
        result_ttv.scope = ttv.scope;
        return Some(result);
      }

      let mut cs = CloneState::new(unsafe { &mut *self.builtin_types.cast_mut() });
      let result = unsafe {
        shallow_clone(
          target,
          &mut *self.arena.cast_mut(),
          &mut cs,
          /* clonePersistentTypes */ true,
        )
      };
      // C++: LUAU_ASSERT(resultTtv)
      let result_ttv = get_mutable::<TableType>(result).expect("shallow_clone yields TableType");
      // C++ `props.emplace` only inserts if the key is absent; the
      // `ty_prop` lookup above already established it is absent here.
      result_ttv
        .props
        .entry(disc_prop_name.clone())
        .or_insert_with(|| disc_prop.clone());
      // Shallow cloning clears out scopes, so let's put back the
      // scope from the original type.
      result_ttv.scope = ttv.scope;
      return Some(result);
    }

    // At this point, we're doing something like:
    //
    //  { ... } & ~nil
    //
    // Which can be handled via fallthrough.

    if is_type_variable(target) || get_type_id::<TypeFunctionInstanceType>(target).is_some() {
      return None;
    }

    if is_approximately_truthy_type(discriminant) {
      return self.basic_intersect_with_truthy(target);
    }
    if is_approximately_truthy_type(target) {
      return self.basic_intersect_with_truthy(discriminant);
    }
    if is_approximately_falsy_type(discriminant) {
      return self.basic_intersect_with_falsy(target);
    }
    if is_approximately_falsy_type(target) {
      return self.basic_intersect_with_falsy(discriminant);
    }

    if get_type_id::<AnyType>(target).is_some() {
      let arena = unsafe { &mut *self.arena.cast_mut() };
      return Some(arena.add_type(UnionType {
        options: alloc::vec![builtin_types.error_type, discriminant],
      }));
    }
    if get_type_id::<ErrorType>(target).is_some() {
      return Some(builtin_types.error_type);
    }
    if let Some(nty) = get_type_id::<NegationType>(discriminant) {
      return self.subtract_one(target, nty.ty);
    }

    self.intersect_one(target, discriminant)
  }
}
