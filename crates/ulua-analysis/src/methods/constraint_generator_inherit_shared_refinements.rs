use ulua_ast::records::location::Location;

use crate::{
  records::{constraint_generator::ConstraintGenerator, scope::Scope, symbol::Symbol},
  type_aliases::scope_ptr_type::ScopePtr,
};

impl ConstraintGenerator {
  pub fn inherit_shared_refinements(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    left_scope: &ScopePtr,
    right_scope: &ScopePtr,
  ) {
    let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
    let left_raw = left_scope.as_ref() as *const Scope as *mut Scope;
    let right_raw = right_scope.as_ref() as *const Scope as *mut Scope;

    unsafe {
      for (left_def, left_ty) in (*left_raw).rvalue_refinements.iter() {
        let left_symbol = self
          .dfg
          .as_ref()
          .and_then(|dfg| dfg.get_symbol_from_def(*left_def))
          .unwrap_or_else(|| (**left_def).name.clone());

        if left_symbol == Symbol::default() {
          continue;
        }

        if scope.lookup_symbol(left_symbol.clone()).is_none() {
          continue;
        };

        if (*left_raw).lvalue_types.find(left_def).is_none() {
          let mut shadowed_by_current_def = false;
          for (candidate_def, _) in (*left_raw).rvalue_refinements.iter() {
            if candidate_def == left_def || (*left_raw).lvalue_types.find(candidate_def).is_none() {
              continue;
            }

            let candidate_symbol = self
              .dfg
              .as_ref()
              .and_then(|dfg| dfg.get_symbol_from_def(*candidate_def))
              .unwrap_or_else(|| (**candidate_def).name.clone());

            if candidate_symbol == left_symbol {
              shadowed_by_current_def = true;
              break;
            }
          }

          if shadowed_by_current_def {
            continue;
          }
        }

        let mut right_match = None;
        for (right_def, right_ty) in (*right_raw).rvalue_refinements.iter() {
          let right_symbol = self
            .dfg
            .as_ref()
            .and_then(|dfg| dfg.get_symbol_from_def(*right_def))
            .unwrap_or_else(|| (**right_def).name.clone());

          if right_symbol != left_symbol {
            continue;
          }

          if (*right_raw).lvalue_types.find(right_def).is_some() {
            right_match = Some((*right_def, *right_ty));
            break;
          }

          if *right_def == *left_def || right_match.is_none() {
            right_match = Some((*right_def, *right_ty));
          }
        }

        if let Some((right_def, _)) = right_match
          && (*right_raw).lvalue_types.find(&right_def).is_none()
        {
          for (candidate_def, candidate_ty) in (*right_raw).rvalue_refinements.iter() {
            if candidate_def == &right_def
              || (*right_raw).lvalue_types.find(candidate_def).is_none()
            {
              continue;
            }

            let candidate_symbol = self
              .dfg
              .as_ref()
              .and_then(|dfg| dfg.get_symbol_from_def(*candidate_def))
              .unwrap_or_else(|| (**candidate_def).name.clone());

            if candidate_symbol == left_symbol {
              right_match = Some((*candidate_def, *candidate_ty));
              break;
            }
          }
        }

        let Some((right_def, right_ty)) = right_match else {
          continue;
        };

        let left_is_current = (*left_raw).lvalue_types.find(left_def).is_some();
        let right_is_current = (*right_raw).lvalue_types.find(&right_def).is_some();
        if !left_is_current && !right_is_current {
          continue;
        }
        if !(left_is_current && self.is_shared_refinement_assignment_type(*left_ty)
          || right_is_current && self.is_shared_refinement_assignment_type(right_ty))
        {
          continue;
        }

        if (*right_raw).lvalue_types.find(&right_def).is_none() {
          let mut shadowed_by_current_def = false;
          for (candidate_def, _) in (*right_raw).rvalue_refinements.iter() {
            if *candidate_def == right_def
              || (*right_raw).lvalue_types.find(candidate_def).is_none()
            {
              continue;
            }

            let candidate_symbol = self
              .dfg
              .as_ref()
              .and_then(|dfg| dfg.get_symbol_from_def(*candidate_def))
              .unwrap_or_else(|| (**candidate_def).name.clone());

            if candidate_symbol == left_symbol {
              shadowed_by_current_def = true;
              break;
            }
          }

          if shadowed_by_current_def {
            continue;
          }
        }

        let ty = if *left_ty == right_ty {
          *left_ty
        } else {
          self
            .make_union_scope_ptr_location_type_id_type_id(scope_raw, location, *left_ty, right_ty)
        };

        self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, *left_def, ty);
        if right_def != *left_def {
          self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, right_def, ty);
        }
      }
    }
  }
}
