use core::ptr::{null, null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction,
    ast_node::AstNode, ast_stat_local::AstStatLocal, ast_type::AstType,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_pack_variadic::AstTypePackVariadic,
    position::Position,
  },
  rtti::{ast_node_as, ast_node_is},
};

use crate::{
  enums::{autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind},
  functions::{
    follow_type::follow_type_id,
    get_local_type_in_scope_at::get_local_type_in_scope_at,
    get_type_alt_j::get_type_id,
    return_first_nonnull_option_of_type::return_first_nonnull_option_of_type,
    try_add_type_correct_suggestion::try_add_type_correct_suggestion,
    try_get_type_pack_type_at::{
      try_get_type_pack_type_at, try_get_type_pack_type_at as try_get_type_pack_type_at_fn,
    },
  },
  records::{
    autocomplete_entry::AutocompleteEntry, function_type::FunctionType, module::Module,
    union_type::UnionType,
  },
  type_aliases::{
    autocomplete_entry_map::AutocompleteEntryMap, scope_ptr_type::ScopePtr, type_id::TypeId,
  },
};
pub fn autocomplete_type_names(
  module: &Module,
  scope_at_position: &ScopePtr,
  position: &mut Position,
  ancestry: &[*mut AstNode],
) -> AutocompleteEntryMap {
  let mut result: AutocompleteEntryMap = Default::default();

  let start_scope: ScopePtr = scope_at_position.clone();

  let mut scope = Some(start_scope.clone());
  while let Some(scope_ptr) = scope.take() {
    for (name, ty) in scope_ptr.exported_type_bindings.iter() {
      if !result.contains_key(name) {
        result.insert(
          name.clone(),
          AutocompleteEntry {
            kind: AutocompleteEntryKind::Type,
            r#type: Some(ty.r#type),
            deprecated: false,
            wrong_index_type: false,
            type_correct: TypeCorrectKind::None,
            containing_extern_type: None,
            prop: None,
            documentation_symbol: unsafe { (*ty.r#type).documentation_symbol.clone() },
            tags: Default::default(),
            parens: Default::default(),
            insert_text: None,
            indexed_with_self: false,
          },
        );
      }
    }

    for (name, ty) in scope_ptr.private_type_bindings.iter() {
      if !result.contains_key(name) {
        result.insert(
          name.clone(),
          AutocompleteEntry {
            kind: AutocompleteEntryKind::Type,
            r#type: Some(ty.r#type),
            deprecated: false,
            wrong_index_type: false,
            type_correct: TypeCorrectKind::None,
            containing_extern_type: None,
            prop: None,
            documentation_symbol: unsafe { (*ty.r#type).documentation_symbol.clone() },
            tags: Default::default(),
            parens: Default::default(),
            insert_text: None,
            indexed_with_self: false,
          },
        );
      }
    }

    for name in scope_ptr.imported_type_bindings.keys() {
      if let Some(binding) = scope_ptr.linear_search_for_binding(name, true)
        && !result.contains_key(name)
      {
        result.insert(
          name.clone(),
          AutocompleteEntry {
            kind: AutocompleteEntryKind::Module,
            r#type: Some(binding.type_id),
            deprecated: false,
            wrong_index_type: false,
            type_correct: TypeCorrectKind::None,
            containing_extern_type: None,
            prop: None,
            documentation_symbol: None,
            tags: Default::default(),
            parens: Default::default(),
            insert_text: None,
            indexed_with_self: false,
          },
        );
      }
    }

    scope = scope_ptr.parent.clone();
  }

  let mut parent: *mut AstNode = null_mut();
  let mut top_type: *mut AstType = null_mut();

  let mut idx = ancestry.len();
  while idx > 0 {
    idx -= 1;
    let it = ancestry[idx];
    if it.is_null() {
      continue;
    }

    if unsafe { (*it).as_type().is_null() } {
      parent = it;
      break;
    } else {
      let as_type = unsafe { (*it).as_type() };
      top_type = as_type;
    }
  }

  if parent.is_null() {
    return result;
  }

  if unsafe { ast_node_is::<AstStatLocal>(&*parent) } {
    let node = unsafe { ast_node_as::<AstStatLocal>(parent) };
    if !node.is_null() {
      for i in 0..unsafe { (*node).vars.size } {
        let var = unsafe { *(*node).vars.data.add(i) };
        if var.is_null() {
          continue;
        }

        let annotation = unsafe { (*var).annotation };
        if annotation.is_null() {
          continue;
        }

        let annotation_location = unsafe { (*annotation).base.location };
        let in_annotation = annotation_location.contains_closed(*position)
          || (annotation_location.begin <= *position
            && unsafe { (*node).equals_sign_location }.is_some_and(|loc| *position == loc.begin));

        if !in_annotation {
          continue;
        }

        if unsafe { (*node).values.size } == 0 {
          break;
        }

        if i >= unsafe { (*node).values.size } {
          let tail_pos = i - unsafe { (*node).values.size } + 1;
          // i is usize; adjust for single iteration like C++:
          // We'll emulate C++ by reading from last value index.
          let last_index = unsafe { (*node).values.size } - 1;
          let expr = unsafe { (*node).values.data.add(last_index) };
          let expr = unsafe { *expr };
          if expr.is_null() {
            break;
          }
          let inferred_type = unsafe {
            infer_type_for_expr(module, expr, top_type, tail_pos, position, &start_scope)
          };
          if let Some(inferred) = inferred_type {
            try_add_type_correct_suggestion(
              &mut result,
              start_scope.clone(),
              top_type,
              inferred,
              *position,
            );
          }
          break;
        } else {
          let tail_pos = 0;
          let expr_ptr = unsafe { (*node).values.data.add(i) };
          let expr = unsafe { *expr_ptr };
          if expr.is_null() {
            break;
          }

          let inferred_type = unsafe {
            infer_type_for_expr(module, expr, top_type, tail_pos, position, &start_scope)
          };
          if let Some(inferred) = inferred_type {
            try_add_type_correct_suggestion(
              &mut result,
              start_scope.clone(),
              top_type,
              inferred,
              *position,
            );
          }
          break;
        }
      }
    }
  } else if unsafe { ast_node_is::<AstExprFunction>(&*parent) } {
    let node = unsafe { ast_node_as::<AstExprFunction>(parent) };
    if node.is_null() {
      return result;
    }

    unsafe fn try_get_expected_function_type(
      module: &Module,
      expr: *mut AstExpr,
    ) -> *const FunctionType {
      let it = module.ast_expected_types.find(&(expr as *const AstExpr));
      let Some(&ty0) = it else {
        return null();
      };
      let ty = follow_type_id(ty0);

      if let Some(ftv) = get_type_id::<FunctionType>(ty) {
        return ftv;
      }

      if let Some(utv) = get_type_id::<UnionType>(ty) {
        // SAFETY: utv 由有效 TypeId 下转而来，前提与 C++ 一致
        if let Some(p) = unsafe { return_first_nonnull_option_of_type::<FunctionType>(utv) } {
          return p;
        }
      }

      null()
    }

    for i in 0..unsafe { (*node).args.size } {
      let arg = unsafe { *(*node).args.data.add(i) };
      if arg.is_null() {
        continue;
      }

      let annotation = unsafe { (*arg).annotation };
      if annotation.is_null() {
        continue;
      }

      let annotation_location = unsafe { (*annotation).base.location };
      let in_annotation = annotation_location.contains_closed(*position)
        || (annotation_location.end == *position
          && unsafe { (*node).arg_location }.is_some_and(|loc| loc.contains_closed(*position)));

      if in_annotation {
        let ftv = unsafe { try_get_expected_function_type(module, node as *mut AstExpr) };
        if !ftv.is_null() {
          if let Some(ty) = try_get_type_pack_type_at_fn(unsafe { (*ftv).arg_types }, i) {
            try_add_type_correct_suggestion(
              &mut result,
              start_scope.clone(),
              top_type,
              ty,
              *position,
            );
          }
        } else if let Some(inferred) =
          get_local_type_in_scope_at(module, scope_at_position, *position, arg)
        {
          try_add_type_correct_suggestion(
            &mut result,
            start_scope.clone(),
            top_type,
            inferred,
            *position,
          );
        }

        break;
      }
    }

    if unsafe { !(*node).vararg_annotation.is_null() } {
      let arg_tp = unsafe { (*node).vararg_annotation };
      let variadic = unsafe { ast_node_as::<AstTypePackVariadic>(arg_tp as *mut AstNode) };
      if !variadic.is_null()
        && (unsafe { (*variadic).base.base.location.contains_closed(*position) }
          || (unsafe { (*variadic).base.base.location.end <= *position }
            && unsafe { (*node).arg_location }.is_some_and(|loc| loc.contains_closed(*position))))
      {
        let ftv = unsafe { try_get_expected_function_type(module, node as *mut AstExpr) };
        if !ftv.is_null()
          && let Some(ty) = try_get_type_pack_type_at_fn(unsafe { (*ftv).arg_types }, usize::MAX)
        {
          try_add_type_correct_suggestion(
            &mut result,
            start_scope.clone(),
            top_type,
            ty,
            *position,
          );
        }
      }
    }

    let return_annotation = unsafe { (*node).return_annotation };
    if return_annotation.is_null() {
      return result;
    }

    let explicit = unsafe { ast_node_as::<AstTypePackExplicit>(return_annotation as *mut AstNode) };
    if !explicit.is_null() {
      let type_pack = unsafe { (*explicit).type_list.types.data };
      let size = unsafe { (*explicit).type_list.types.size };

      for i in 0..size {
        let ret = unsafe { *type_pack.add(i) };
        if ret.is_null() {
          continue;
        }

        let ret_location = unsafe { (*ret).base.location };
        if ret_location.contains_closed(*position) {
          let ftv = unsafe { try_get_expected_function_type(module, node as *mut AstExpr) };
          if !ftv.is_null()
            && let Some(ty) = try_get_type_pack_type_at_fn(unsafe { (*ftv).ret_types }, i)
          {
            try_add_type_correct_suggestion(
              &mut result,
              start_scope.clone(),
              top_type,
              ty,
              *position,
            );
          }
          break;
        }
      }

      let tail_type = unsafe { (*explicit).type_list.tail_type };
      if !tail_type.is_null() {
        let variadic = unsafe { ast_node_as::<AstTypePackVariadic>(tail_type as *mut AstNode) };
        if !variadic.is_null()
          && unsafe { (*variadic).base.base.location.contains_closed(*position) }
        {
          let ftv = unsafe { try_get_expected_function_type(module, node as *mut AstExpr) };
          if !ftv.is_null()
            && let Some(ty) = try_get_type_pack_type_at_fn(unsafe { (*ftv).ret_types }, usize::MAX)
          {
            try_add_type_correct_suggestion(&mut result, start_scope, top_type, ty, *position);
          }
        }
      }
    } else {
      let variadic =
        unsafe { ast_node_as::<AstTypePackVariadic>(return_annotation as *mut AstNode) };
      if !variadic.is_null() && unsafe { (*variadic).base.base.location.contains_closed(*position) }
      {
        let ftv = unsafe { try_get_expected_function_type(module, node as *mut AstExpr) };
        if !ftv.is_null()
          && let Some(ty) = try_get_type_pack_type_at_fn(unsafe { (*ftv).ret_types }, usize::MAX)
        {
          try_add_type_correct_suggestion(&mut result, start_scope, top_type, ty, *position);
        }
      }
    }
  }

  result
}

unsafe fn infer_type_for_expr(
  module: &Module,
  expr: *mut AstExpr,
  _top_type: *mut AstType,
  tail_pos: usize,
  _position: &Position,
  _start_scope: &ScopePtr,
) -> Option<TypeId> {
  unsafe {
    if expr.is_null() {
      return None;
    }

    if ast_node_is::<AstExprCall>(&*(expr as *mut AstNode)) {
      let expr_call = ast_node_as::<AstExprCall>(expr as *mut AstNode);
      if expr_call.is_null() {
        return None;
      }

      let it = module
        .ast_types
        .find(&((*expr_call).func as *const AstExpr));
      it?;

      let ty = follow_type_id(*it.unwrap());

      let ftv = get_type_id::<FunctionType>(ty)?;
      try_get_type_pack_type_at(ftv.ret_types, tail_pos)
    } else {
      if tail_pos != 0 {
        return None;
      }

      let it = module.ast_types.find(&(expr as *const AstExpr));
      it?;

      Some(*it.unwrap())
    }
  }
}
