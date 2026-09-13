use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    any_type::AnyType, blocked_type::BlockedType, extern_type::ExternType, free_type::FreeType,
    function_type::FunctionType, generic_type::GenericType, intersection_type::IntersectionType,
    iterative_type_visitor::IterativeTypeVisitor, lazy_type::LazyType,
    metatable_type::MetatableType, negation_type::NegationType, never_type::NeverType,
    no_refine_type::NoRefineType, pending_expansion_type::PendingExpansionType,
    primitive_type::PrimitiveType, singleton_type::SingletonType, table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{bound_type::BoundType, error_type::ErrorType, type_id::TypeId},
};
impl IterativeTypeVisitor {
  pub fn process_type_id(&mut self, mut ty: TypeId) {
    // Morally, if `skipBoundTypes` is set, then whenever we encounter a bound
    // type we should "skip" ahead to the first non-bound type.
    //
    // We do this check here so that we treat all bound types as if they're
    // direct pointers to some final non-bound type. If we do the check later,
    // then we might get slightly different behavior depending on the exact
    // entry point for cyclic types.
    if self.skip_bound_types {
      if get_type_id::<BoundType>(ty).is_some() {
        ty = follow_type_id(ty);
      } else if let Some(tt) = get_type_id::<TableType>(ty)
        && tt.bound_to.is_some()
      {
        ty = follow_type_id(ty);
      }
    }

    if self.iterative_type_visitor_has_seen(ty as *const c_void) {
      return;
    }

    if let Some(btv) = get_type_id::<BoundType>(ty) {
      // At this point, we know that `skipBoundTypes` is false, as
      // otherwise we would have hit the above branch.
      LUAU_ASSERT!(!self.skip_bound_types);
      if self.visit_type_id_bound_type(ty, btv) {
        self.traverse_type_id(btv.bound_to);
      }
    } else if let Some(ftv) = get_type_id::<FreeType>(ty) {
      if self.visit_type_id_free_type(ty, ftv) {
        LUAU_ASSERT!(!ftv.lower_bound.is_null());
        LUAU_ASSERT!(!ftv.upper_bound.is_null());

        self.traverse_type_id(ftv.lower_bound);
        self.traverse_type_id(ftv.upper_bound);
      }
    } else if let Some(gtv) = get_type_id::<GenericType>(ty) {
      self.visit_type_id_generic_type(ty, gtv);
    } else if let Some(etv) = get_type_id::<ErrorType>(ty) {
      self.visit_type_id_error_type(ty, etv);
    } else if let Some(ptv) = get_type_id::<PrimitiveType>(ty) {
      self.visit_type_id_primitive_type(ty, ptv);
    } else if let Some(ftv) = get_type_id::<FunctionType>(ty) {
      if self.visit_type_id_function_type(ty, ftv) {
        self.traverse_type_pack_id(ftv.arg_types);
        self.traverse_type_pack_id(ftv.ret_types);
      }
    } else if let Some(ttv) = get_type_id::<TableType>(ty) {
      // Some visitors want to see bound tables, that's why we traverse the original type
      LUAU_ASSERT!(!self.skip_bound_types || ttv.bound_to.is_none());
      if let Some(bound_to) = ttv.bound_to
        && self.skip_bound_types
      {
        self.traverse_type_id(bound_to);
      } else if self.visit_type_id_table_type(ty, ttv) {
        if let Some(bound_to) = ttv.bound_to {
          self.traverse_type_id(bound_to);
        } else {
          for prop in ttv.props.values() {
            if let Some(read_ty) = prop.read_ty {
              self.traverse_type_id(read_ty);
            }

            // In the case that the readType and the writeType
            // are the same pointer, just traverse once.
            // Traversing each property twice has pretty
            // significant performance consequences.
            if let Some(write_ty) = prop.write_ty
              && !prop.is_shared()
            {
              self.traverse_type_id(write_ty);
            }
          }

          if let Some(indexer) = &ttv.indexer {
            self.traverse_type_id(indexer.index_type);
            self.traverse_type_id(indexer.index_result_type);
          }
        }
      }
    } else if let Some(mtv) = get_type_id::<MetatableType>(ty) {
      if self.visit_type_id_metatable_type(ty, mtv) {
        self.traverse_type_id(mtv.table);
        self.traverse_type_id(mtv.metatable);
      }
    } else if let Some(etv) = get_type_id::<ExternType>(ty) {
      if self.visit_type_id_extern_type(ty, etv) {
        for prop in etv.props.values() {
          if let Some(read_ty) = prop.read_ty {
            self.traverse_type_id(read_ty);
          }

          // In the case that the readType and the writeType are
          // the same pointer, just traverse once. Traversing each
          // property twice would have pretty significant
          // performance consequences.
          if let Some(write_ty) = prop.write_ty
            && !prop.is_shared()
          {
            self.traverse_type_id(write_ty);
          }
        }

        if let Some(parent) = etv.parent {
          self.traverse_type_id(parent);
        }

        if let Some(metatable) = etv.metatable {
          self.traverse_type_id(metatable);
        }

        if let Some(indexer) = &etv.indexer {
          self.traverse_type_id(indexer.index_type);
          self.traverse_type_id(indexer.index_result_type);
        }
      }
    } else if let Some(atv) = get_type_id::<AnyType>(ty) {
      self.visit_type_id_any_type(ty, atv);
    } else if let Some(nrt) = get_type_id::<NoRefineType>(ty) {
      self.visit_type_id_no_refine_type(ty, nrt);
    } else if let Some(utv) = get_type_id::<UnionType>(ty) {
      if self.visit_type_id_union_type(ty, utv) {
        let mut union_changed = false;
        // 克隆后再遍历：遍历过程中 visit 可能改写该 union 的 options
        let options = utv.options.clone();
        for opt_ty in options {
          self.traverse_type_id(opt_ty);
          if get_type_id::<UnionType>(follow_type_id(ty)).is_none() {
            union_changed = true;
            break;
          }
        }

        if union_changed {
          self.traverse_type_id(ty);
        }
      }
    } else if let Some(itv) = get_type_id::<IntersectionType>(ty) {
      if self.visit_type_id_intersection_type(ty, itv) {
        let mut intersection_changed = false;
        let parts = itv.parts.clone();
        for part_ty in parts {
          self.traverse_type_id(part_ty);
          if get_type_id::<IntersectionType>(follow_type_id(ty)).is_none() {
            intersection_changed = true;
            break;
          }
        }

        if intersection_changed {
          self.traverse_type_id(ty);
        }
      }
    } else if let Some(ltv) = get_type_id::<LazyType>(ty) {
      let unwrapped: TypeId = ltv.unwrapped;
      if !unwrapped.is_null() {
        self.traverse_type_id(unwrapped);
      }

      // Visiting into LazyType that hasn't been unwrapped may necessarily
      // cause infinite expansion, so we don't do that on purpose. Asserting
      // also makes no sense, because the type _will_ happen here, most likely
      // as a property of some ExternType that doesn't need to be expanded.
    } else if let Some(stv) = get_type_id::<SingletonType>(ty) {
      self.visit_type_id_singleton_type(ty, stv);
    } else if let Some(btv) = get_type_id::<BlockedType>(ty) {
      self.visit_type_id_blocked_type(ty, btv);
    } else if let Some(utv) = get_type_id::<UnknownType>(ty) {
      self.visit_type_id_unknown_type(ty, utv);
    } else if let Some(ntv) = get_type_id::<NeverType>(ty) {
      self.visit_type_id_never_type(ty, ntv);
    } else if let Some(petv) = get_type_id::<PendingExpansionType>(ty) {
      if self.visit_type_id_pending_expansion_type(ty, petv) {
        let type_arguments = petv.type_arguments.clone();
        for a in type_arguments {
          self.traverse_type_id(a);
        }

        let pack_arguments = petv.pack_arguments.clone();
        for a in pack_arguments {
          self.traverse_type_pack_id(a);
        }
      }
    } else if let Some(ntv) = get_type_id::<NegationType>(ty) {
      if self.visit_type_id_negation_type(ty, ntv) {
        self.traverse_type_id(ntv.ty);
      }
    } else if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(ty) {
      if self.visit_type_id_type_function_instance_type(ty, tfit) {
        let type_arguments = tfit.type_arguments.clone();
        for p in type_arguments {
          self.traverse_type_id(p);
        }

        let pack_arguments = tfit.pack_arguments.clone();
        for p in pack_arguments {
          self.traverse_type_pack_id(p);
        }
      }
    } else {
      LUAU_ASSERT!(
        false /* "GenericTypeVisitor::traverse(TypeId) is not exhaustive!" */
      );
    }

    self.iterative_type_visitor_unsee(ty as *const c_void);
  }
}
