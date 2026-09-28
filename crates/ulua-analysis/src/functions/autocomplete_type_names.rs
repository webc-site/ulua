use core::ptr::{NonNull, null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction,
    ast_node::AstNode, ast_stat_local::AstStatLocal, ast_type::AstType,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_pack_variadic::AstTypePackVariadic,
    position::Position,
  },
  rtti::{ast_node_try_as, ast_node_try_as_ptr},
};

use crate::{
  enums::autocomplete_entry_kind::AutocompleteEntryKind,
  functions::{
    follow_type, get_local_type_in_scope_at::get_local_type_in_scope_at, get_type,
    return_first_nonnull_option_of_type::return_first_nonnull_option_of_type,
    try_add_type_correct_suggestion::try_add_type_correct_suggestion,
    try_get_type_pack_type_at::try_get_type_pack_type_at,
  },
  records::{
    autocomplete_entry::AutocompleteEntry, function_type::FunctionType, module::Module,
    scope::Scope, scope_registry::resolve_scope, type_fun::TypeFun, union_type::UnionType,
  },
  type_aliases::{
    autocomplete_entry_map::AutocompleteEntryMap, collections::HashMap, name_type::Name,
    scope_ptr_type::ScopePtr, type_id::TypeId,
  },
};
pub(crate) fn autocomplete_type_names(
  module: &Module,
  scope_at_position: &ScopePtr,
  position: &mut Position,
  ancestry: &[*mut AstNode],
) -> AutocompleteEntryMap {
  let mut result: AutocompleteEntryMap = Default::default();

  let start_scope: ScopePtr = scope_at_position.clone();

  // 对照 C++ `for (ScopePtr scope = startScope; scope; scope = scope->parent)`。
  let mut scope: Option<&Scope> = Some(scope_at_position.as_ref());
  while let Some(scope_ptr) = scope {
    add_type_bindings(&mut result, &scope_ptr.exported_type_bindings);
    add_type_bindings(&mut result, &scope_ptr.private_type_bindings);

    for name in scope_ptr.imported_type_bindings.keys() {
      if let Some(binding) = scope_ptr.linear_search_for_binding(name, true)
        && !result.contains_key(name)
      {
        result.insert(
          name.to_string(),
          AutocompleteEntry {
            kind: AutocompleteEntryKind::Module,
            r#type: Some(binding.type_id),
            ..Default::default()
          },
        );
      }
    }

    scope = scope_ptr.parent.and_then(resolve_scope);
  }

  let mut parent: *mut AstNode = null_mut();
  let mut top_type: *mut AstType = null_mut();

  // 对照 C++：倒序找首个非类型祖先，沿途记录最近的类型祖先。
  for &it in ancestry.iter().rev() {
    if it.is_null() {
      continue;
    }

    // SAFETY: it 判非空，AST 节点由 arena 持有；下转结果按 *mut 沿用到
    // find_type_element_at_*，故直接经裸指针取，避免先造共享引用再要可变借用。
    let as_type = unsafe { (*it).as_type() }.map_or(null_mut(), NonNull::as_ptr);
    if as_type.is_null() {
      parent = it;
      break;
    }
    top_type = as_type;
  }

  if parent.is_null() {
    return result;
  }

  // SAFETY: parent 判非空，指向 arena 存活节点。
  let parent_ref: &AstNode = unsafe { &*parent };

  if let Some(node) = ast_node_try_as::<AstStatLocal>(parent_ref) {
    // Try to provide inferred type of the local
    for (i, &var) in node.vars.as_slice().iter().enumerate() {
      if var.is_null() {
        continue;
      }

      // SAFETY: var 判非空，AST 节点由 arena 持有。
      let Some(annotation) = (unsafe { (*var).annotation.as_ref() }) else {
        continue;
      };

      // C++（AutocompleteCore.cpp:1186）仅判断 annotation 区域闭合包含光标。
      if !annotation.base.location.contains_closed(*position) {
        continue;
      }

      let values = node.values.as_slice();
      if values.is_empty() {
        break;
      }

      // C++：多返回值时取最后一个调用返回 type pack（i 重赋值模拟为元组）。
      let (expr, tail_pos) = if i >= values.len() {
        (values[values.len() - 1], i - values.len() + 1)
      } else {
        (values[i], 0)
      };

      // SAFETY: expr 判空后才解引用（C++ 同款前提），AST 节点由 arena 持有。
      let Some(expr) = (unsafe { expr.as_ref() }) else {
        break;
      };

      if let Some(inferred) = infer_type_for_expr(module, expr, tail_pos) {
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
  } else if let Some(node) = ast_node_try_as::<AstExprFunction>(parent_ref) {
    // repr(C) 单继承：AstExprFunction 与 AstExpr 同址，upcast 等价。
    let fn_expr = node as *const AstExprFunction as *const AstExpr;

    // For lookup inside expected function type if that's available
    for (i, arg) in node.args.iter().enumerate() {
      let Some(annotation) = (unsafe { arg.annotation.as_ref() }) else {
        continue;
      };

      // C++（AutocompleteCore.cpp:1263）仅判断 annotation 区域闭合包含光标。
      if !annotation.base.location.contains_closed(*position) {
        continue;
      }

      if let Some(ftv) = try_get_expected_function_type(module, fn_expr) {
        if let Some(ty) = try_get_type_pack_type_at(ftv.arg_types, i) {
          try_add_type_correct_suggestion(
            &mut result,
            start_scope.clone(),
            top_type,
            ty,
            *position,
          );
        }
      } else if let Some(inferred) = get_local_type_in_scope_at(
        module,
        scope_at_position,
        *position,
        node.args.at(i).as_ptr(),
      ) {
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

    if let Some(variadic) = (unsafe {
      // Safety: vararg_annotation 是 parser 产出的可空 *mut AstTypePack，
      // ast_node_try_as_ptr 对 null 返回 None；class_index 命中后 repr(C) 单继承保证下转类型正确。
      ast_node_try_as_ptr::<AstTypePackVariadic>(node.vararg_annotation)
    })
      // C++（AutocompleteCore.cpp:1284）仅判断 variadic 区域闭合包含光标。
      && variadic.base.base.location.contains_closed(*position)
      && let Some(ftv) = try_get_expected_function_type(module, fn_expr)
      && let Some(ty) = try_get_type_pack_type_at(ftv.arg_types, usize::MAX)
    {
      try_add_type_correct_suggestion(&mut result, start_scope.clone(), top_type, ty, *position);
    }

    let return_annotation = node.return_annotation;
    if return_annotation.is_null() {
      return result;
    }

    if let Some(explicit) = unsafe {
      // Safety: return_annotation 在上方 is_null 分支已提前返回，此处非空且指向 arena
      // 存活节点；class_index 命中后 repr(C) 单继承保证下转为 AstTypePackExplicit。
      ast_node_try_as_ptr::<AstTypePackExplicit>(return_annotation)
    } {
      for (i, &ret) in explicit.type_list.types.as_slice().iter().enumerate() {
        if ret.is_null() {
          continue;
        }

        // SAFETY: ret 判非空，AST 节点由 arena 持有。
        let ret_location = unsafe { (*ret).base.location };
        if ret_location.contains_closed(*position) {
          if let Some(ftv) = try_get_expected_function_type(module, fn_expr)
            && let Some(ty) = try_get_type_pack_type_at(ftv.ret_types, i)
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

      // SAFETY: tail_type 可空，ast_node_try_as_ptr 对 null 返回 None；class_index 命中后
      // repr(C) 单继承保证 cast 有效。
      if let Some(variadic) =
        (unsafe { ast_node_try_as_ptr::<AstTypePackVariadic>(explicit.type_list.tail_type) })
        && variadic.base.base.location.contains_closed(*position)
        && let Some(ftv) = try_get_expected_function_type(module, fn_expr)
        && let Some(ty) = try_get_type_pack_type_at(ftv.ret_types, usize::MAX)
      {
        try_add_type_correct_suggestion(&mut result, start_scope, top_type, ty, *position);
      }
    } else if let Some(variadic) = (unsafe {
      // Safety: return_annotation 已判非空（is_null 提前返回），指向 arena 存活节点；
      // class_index 命中后 repr(C) 单继承保证下转为 AstTypePackVariadic，未命中返回 None。
      ast_node_try_as_ptr::<AstTypePackVariadic>(return_annotation)
    }) && variadic.base.base.location.contains_closed(*position)
      && let Some(ftv) = try_get_expected_function_type(module, fn_expr)
      && let Some(ty) = try_get_type_pack_type_at(ftv.ret_types, usize::MAX)
    {
      try_add_type_correct_suggestion(&mut result, start_scope, top_type, ty, *position);
    }
  }

  result
}

/// 对照 C++：把 exported/private 类型绑定填入候选表（两处结构共用）。
fn add_type_bindings(result: &mut AutocompleteEntryMap, bindings: &HashMap<Name, TypeFun>) {
  for (name, ty) in bindings {
    if !result.contains_key(name) {
      result.insert(
        name.clone(),
        AutocompleteEntry {
          kind: AutocompleteEntryKind::Type,
          r#type: Some(ty.r#type),
          documentation_symbol: unsafe {
            // Safety: ty.r#type 是会话期内 arena 持有的 *const Type，块地址稳定；
            // 解引用读取 documentation_symbol 后克隆，此处无并存可变借用。
            (*ty.r#type).documentation_symbol.clone()
          },
          ..Default::default()
        },
      );
    }
  }
}

/// 对照 C++ `tryGetExpectedFunctionType` lambda（AutocompleteCore.cpp:1264）：
/// 从期望类型提取函数类型；可选函数类型走联合类型首个非空选项。
fn try_get_expected_function_type(
  module: &Module,
  expr: *const AstExpr,
) -> Option<&'static FunctionType> {
  // 对照 C++ `if (!it) return nullptr;`
  let &ty0 = module.ast_expected_types.find(&expr)?;
  let ty = follow_type::follow(ty0);

  if let Some(ftv) = get_type::get::<FunctionType>(ty) {
    return Some(ftv);
  }

  let utv = get_type::get::<UnionType>(ty)?;
  return_first_nonnull_option_of_type::<FunctionType>(utv)
}

/// 对照 C++（AutocompleteCore.cpp:1210-1245）：从表达式推断类型，
/// 调用表达式取返回 type pack 第 tail_pos 项。
fn infer_type_for_expr(module: &Module, expr: &AstExpr, tail_pos: usize) -> Option<TypeId> {
  // class_index 判定调用表达式（与 ast_node_is 同语义）。
  if let Some(expr_call) = ast_node_try_as::<AstExprCall>(&expr.base) {
    let func_key = expr_call.func as *const AstExpr;
    let it = module.ast_types.find(&func_key)?;

    let ty = follow_type::follow(*it);
    let ftv = get_type::get::<FunctionType>(ty)?;
    try_get_type_pack_type_at(ftv.ret_types, tail_pos)
  } else {
    if tail_pos != 0 {
      return None;
    }

    let key = expr as *const AstExpr;
    Some(*module.ast_types.find(&key)?)
  }
}
