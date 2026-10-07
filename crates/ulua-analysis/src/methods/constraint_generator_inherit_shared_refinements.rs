use ulua_ast::records::location::Location;

use crate::{
  functions::shared_mut::shared_mut,
  records::{
    arena_handle::alias_ref, constraint_generator::ConstraintGenerator, def_registry::def_ref,
    scope::Scope, symbol::Symbol,
  },
  type_aliases::{def_id_def::DefId, scope_ptr_type::ScopePtr},
};

impl ConstraintGenerator {
  /// 五处同构的「def → Symbol」解析：优先走 DFG 的 def→symbol 映射，退回
  /// Def arena 的名字字段（cpp `getDefLogicalName` 同款回退）。
  fn resolve_symbol(&self, def: DefId) -> Symbol {
    self
      .dfg_ref()
      .get_symbol_from_def(def)
      .unwrap_or_else(|| def_ref(def).expect("Def arena 存活句柄契约").name.clone())
  }

  /// 两处同构的 `shadowedByCurrentDef`：`def` 是否被同一 scope 内带 lvalue
  /// 类型的同名 def 遮蔽（`symbol` 为待比对的 def 名，两处均为 `left_symbol`）。
  /// `scope` 为调用方保活的存活 `Scope` 共享借用；本函数只读其
  /// `rvalue_refinements`/`lvalue_types` 两字段，不写入、不派生长引用。
  fn shadowed_by_current_def(&self, scope: &Scope, def: DefId, symbol: &Symbol) -> bool {
    for (candidate_def, _) in scope.rvalue_refinements.iter() {
      if *candidate_def == def || scope.lvalue_types.find(candidate_def).is_none() {
        continue;
      }

      if self.resolve_symbol(*candidate_def) == *symbol {
        return true;
      }
    }

    false
  }

  /// C++ `inheritSharedRefinements(const ScopePtr& scope, Location,
  /// const ScopePtr& leftScope, const ScopePtr& rightScope)`。形参链已引用化：
  /// 两侧 `Arc<Scope>` 只读遍历经 alias_ref 门面收口为共享借用，写入只落在
  /// `update_r_value_refinements_*` 的即物化窗口内（单线程串行）。
  pub fn inherit_shared_refinements(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    left_scope: &ScopePtr,
    right_scope: &ScopePtr,
  ) {
    let left = alias_ref(shared_mut(left_scope));
    let right = alias_ref(shared_mut(right_scope));

    for (left_def, left_ty) in left.rvalue_refinements.iter() {
      let left_symbol = self.resolve_symbol(*left_def);

      if left_symbol == Symbol::default() {
        continue;
      }

      if scope.lookup_symbol(left_symbol.clone()).is_none() {
        continue;
      };

      if left.lvalue_types.find(left_def).is_none()
        && self.shadowed_by_current_def(left, *left_def, &left_symbol)
      {
        continue;
      }

      let mut right_match = None;
      for (right_def, right_ty) in right.rvalue_refinements.iter() {
        if self.resolve_symbol(*right_def) != left_symbol {
          continue;
        }

        if right.lvalue_types.find(right_def).is_some() {
          right_match = Some((*right_def, *right_ty));
          break;
        }

        if *right_def == *left_def || right_match.is_none() {
          right_match = Some((*right_def, *right_ty));
        }
      }

      if let Some((right_def, _)) = right_match
        && right.lvalue_types.find(&right_def).is_none()
      {
        for (candidate_def, candidate_ty) in right.rvalue_refinements.iter() {
          if *candidate_def != right_def
            && right.lvalue_types.find(candidate_def).is_some()
            && self.resolve_symbol(*candidate_def) == left_symbol
          {
            right_match = Some((*candidate_def, *candidate_ty));
            break;
          }
        }
      }

      let Some((right_def, right_ty)) = right_match else {
        continue;
      };

      let left_is_current = left.lvalue_types.find(left_def).is_some();
      let right_is_current = right.lvalue_types.find(&right_def).is_some();
      if !left_is_current && !right_is_current {
        continue;
      }
      if !(left_is_current && self.is_shared_refinement_assignment_type(*left_ty)
        || right_is_current && self.is_shared_refinement_assignment_type(right_ty))
      {
        continue;
      }

      if right.lvalue_types.find(&right_def).is_none()
        && self.shadowed_by_current_def(right, right_def, &left_symbol)
      {
        continue;
      }

      let ty = if *left_ty == right_ty {
        *left_ty
      } else {
        self.make_union_scope_ptr_location_type_id_type_id(scope, location, *left_ty, right_ty)
      };

      self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, *left_def, ty);
      if right_def != *left_def {
        self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, right_def, ty);
      }
    }
  }
}
