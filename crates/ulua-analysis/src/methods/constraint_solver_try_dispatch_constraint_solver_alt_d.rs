use alloc::sync::Arc;

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack_id,
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id, generalize::generalize,
    generalize_type::generalize_type, generalize_type_pack::generalize_type_pack,
    get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
    get_type_pack::get_type_pack_id, is_known::is_known,
    prune_unnecessary_generics::prune_unnecessary_generics, seal_table::seal_table,
  },
  records::{
    blocked_type::BlockedType, code_too_complex::CodeTooComplex, constraint::Constraint,
    constraint_solver::ConstraintSolver, free_type::FreeType, free_type_pack::FreeTypePack,
    function_type::FunctionType, generalization_constraint::GeneralizationConstraint,
    generalization_params::GeneralizationParams, pending_expansion_type::PendingExpansionType,
    table_type::TableType,
  },
  type_aliases::{type_pack_variant::TypePackVariant, type_variant::TypeVariant},
};

impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_generalization_constraint_not_null_constraint(
    &mut self,
    c: &GeneralizationConstraint,
    constraint: *const Constraint,
  ) -> bool {
    let generalized_type = follow_type_id(c.generalized_type);

    if self.is_blocked_type_id(c.source_type) {
      return self.block_type_id_not_null_constraint(c.source_type, constraint);
    } else if get_type_id::<PendingExpansionType>(generalized_type).is_some() {
      return self.block_type_id_not_null_constraint(generalized_type, constraint);
    }

    let generalized_ty = generalize(
      self.arena,
      self.builtin_types,
      unsafe { (*constraint).scope },
      &mut self.generalized_types_ as *mut _,
      c.source_type,
      None,
    );

    if generalized_ty.is_none() {
      self.report_error_type_error_data_location(CodeTooComplex::default().into(), unsafe {
        &(*constraint).location
      });
    }

    if let Some(generalized_ty) = generalized_ty {
      unsafe {
        prune_unnecessary_generics(
          self.arena,
          self.builtin_types,
          (*constraint).scope,
          &mut self.generalized_types_ as *mut _,
          generalized_ty,
        )
      };

      if get_type_id::<BlockedType>(generalized_type).is_some() {
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(
            constraint,
            generalized_type,
            generalized_ty,
          )
        };
      } else {
        self.constraint_solver_unify(constraint, generalized_type, generalized_ty);
      }

      if c.has_deprecated_attribute
        && let Some(fty) = get_mutable_type_id::<FunctionType>(follow_type_id(generalized_type))
      {
        fty.is_deprecated_function = true;
        fty.deprecated_info = Some(Arc::new(c.deprecated_info.clone()));
      }
    } else {
      self.report_error_type_error_data_location(CodeTooComplex::default().into(), unsafe {
        &(*constraint).location
      });
      unsafe {
        self.bind_not_null_constraint_type_id_type_id(
          constraint,
          c.generalized_type,
          (*self.builtin_types).error_type,
        )
      };
    }

    unsafe {
      let scope = (*constraint).scope;

      if let Some(interior_free_types) = (*scope).interior_free_types.as_ref() {
        let interior_free_types = interior_free_types.clone();
        for ty in interior_free_types {
          let ty = follow_type_id(ty);
          let free_ty = get_type_id::<FreeType>(ty);

          if let Some(free_ty) = free_ty {
            let params = GeneralizationParams {
              found_outside_functions: true,
              use_count: 1,
              polarity: free_ty.polarity,
            };
            let res = generalize_type(self.arena, self.builtin_types, scope, ty, &params);
            if res.resource_limits_exceeded {
              self.report_error_type_error_data_location(
                CodeTooComplex::default().into(),
                &(*scope).location,
              );
            }
          } else if get_type_id::<TableType>(ty).is_some() {
            seal_table(scope, ty);
          }

          self.unblock_type_id_location(ty, (*constraint).location);
        }
      }

      if let Some(interior_free_type_packs) = (*scope).interior_free_type_packs.as_ref() {
        let interior_free_type_packs = interior_free_type_packs.clone();
        for tp in interior_free_type_packs {
          let tp = follow_type_pack_id(tp);
          let free_tp = get_type_pack_id::<FreeTypePack>(tp);

          if let Some(free_tp) = free_tp {
            let params = GeneralizationParams {
              found_outside_functions: true,
              use_count: 1,
              polarity: free_tp.polarity,
            };
            ulua_common::macros::luau_assert::LUAU_ASSERT!(is_known(params.polarity));
            generalize_type_pack(self.arena, self.builtin_types, scope, tp, &params);
          }
        }
      }

      if c.no_generics
        && let Some(ft) = get_mutable_type_id::<FunctionType>(c.source_type)
      {
        for r#gen in ft.generics.iter().copied() {
          (*as_mutable_type_id(r#gen)).ty = TypeVariant::Bound((*self.builtin_types).unknown_type);
        }
        ft.generics.clear();

        for r#gen in ft.generic_packs.iter().copied() {
          (*as_mutable_type_pack_id(r#gen)).ty =
            TypePackVariant::Bound((*self.builtin_types).unknown_type_pack);
        }
        ft.generic_packs.clear();
      }
    }

    true
  }
}
