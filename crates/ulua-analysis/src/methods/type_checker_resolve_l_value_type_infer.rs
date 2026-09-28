use alloc::vec::Vec;
use core::ptr::null;

use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{baseof::baseof, get_base_symbol::get_base_symbol, get_l_value::get_l_value},
  records::{
    field::Field, scope::Scope, scope_registry::resolve_scope, symbol::Symbol,
    type_checker::TypeChecker,
  },
  type_aliases::{
    l_value::{LValue, LValueMember},
    refinement_map::RefinementMap,
    scope_ptr_type::ScopePtr,
    type_id::TypeId,
  },
};

impl TypeChecker {
  pub fn resolve_l_value_scope_ptr_l_value(
    &mut self,
    scope: ScopePtr,
    lvalue: &LValue,
  ) -> Option<TypeId> {
    // We want to be walking the Scope parents.
    // We'll also want to walk up the LValue path. As we do this, we need to save each LValue because we must walk back.
    let symbol: Symbol = get_base_symbol(lvalue);

    let mut current_scope: Option<&Scope> = Some(scope.as_ref());
    while let Some(cur) = current_scope {
      let mut found: Option<TypeId> = None;

      let mut top_lvalue: *const LValue = null();

      {
        let mut curr: *const LValue = lvalue;
        while !curr.is_null() {
          // Safety: while 守卫 `!curr.is_null()` 保证 curr 非空；它指向 caller 传入的
          // &LValue 或 baseof 返回的 Arc<LValue> 父节点（整条链存活期内被持有），对齐
          // 且存活，取只读借用供 RefinementMap::get 查询。
          if let Some(it) = cur.refinements.get(unsafe { &*curr }) {
            found = Some(*it);
            top_lvalue = curr;
            break;
          }
          // Safety: 同上 curr 非空存活，baseof 仅只读该 LValue 以取其父链指针。
          curr = baseof(unsafe { &*curr });
        }
        if found.is_none() {
          // top_lvalue stays null (the loop above terminated without a match).
          top_lvalue = curr;
        }
      }

      if found.is_none() {
        // Should not be using scope->lookup. This is already recursive.
        if let Some(binding) = cur.bindings.get(&symbol) {
          found = Some(binding.type_id);
        } else {
          // Nothing exists in this Scope. Just skip and try the parent one.
          current_scope = cur.parent.and_then(resolve_scope);
          continue;
        }
      }

      // We need to walk the l-value path in reverse, so we collect components into a vector
      let mut child_keys: Vec<*const LValue> = Vec::new();

      {
        let mut curr: *const LValue = lvalue;
        while curr != top_lvalue {
          child_keys.push(curr);
          // Safety: 循环仅当 curr != top_lvalue 时推进，而 top_lvalue 是链上终点（匹配
          // 节点或 null），故此处 curr 必为链上存活非空 LValue（caller lvalue 或 Arc 父），
          // baseof 仅只读其父链指针。
          curr = baseof(unsafe { &*curr });
        }
      }

      for &key_ptr in child_keys.iter().rev() {
        // Safety: key_ptr 来自 child_keys——第二循环在 curr 非空且未到终点时压入的存活
        // LValue 指针（caller lvalue 或 Arc 父节点），链在函数返回前持续存活，故解引用有效。
        let key: &LValue = unsafe { &*key_ptr };

        // Symbol can happen. Skip.
        if !get_l_value::<Symbol>(key).is_null() {
          continue;
        } else if let Some(field) = <Field as LValueMember>::get_if(key) {
          found = self.get_index_type_from_type(
            scope.clone(),
            // 入口 Some 由上方 `found.is_none()` 早退链蕴含；后续轮次传入本轮
            // 回写值（cpp 同位 `*found` 直解同前提，None 属上游同款退化角例）。
            found.expect("循环入口 Some 由 is_none 早退链蕴含；cpp *found 同前提"),
            &field.key,
            &Location::default(),
            false,
          );
          found?;
        } else {
          LUAU_ASSERT!(false); // "New LValue alternative not handled here."
        }
      }

      return found;
    }

    // No entry for it at all. Can happen when LValue root is a global.
    None
  }

  pub fn resolve_l_value_refinement_map_scope_ptr_l_value(
    &mut self,
    refis: &RefinementMap,
    scope: ScopePtr,
    lvalue: &LValue,
  ) -> Option<TypeId> {
    if let Some(ty) = refis.get(lvalue) {
      Some(*ty)
    } else {
      self.resolve_l_value_scope_ptr_l_value(scope, lvalue)
    }
  }
}
