use alloc::string::{String, ToString};

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_node::AstNode, position::Position},
  rtti::ast_node_as,
};

use crate::{
  enums::{autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind},
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    follow_type_pack::follow_type_pack_id, get_type_alt_j::get_type_id,
    get_type_pack::get_type_pack_id, is_variadic_type_pack::is_variadic,
    return_first_nonnull_option_of_type::return_first_nonnull_option_of_type,
    try_get_type_name_in_scope_autocomplete_core::try_get_type_name_in_scope,
    try_get_type_name_in_scope_autocomplete_core_alt_b::try_get_type_name_in_scope_scope_ptr_type_pack_id_bool,
  },
  records::{
    autocomplete_entry::AutocompleteEntry, free_type_pack::FreeTypePack,
    function_type::FunctionType, union_type::UnionType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr},
};
pub fn make_anonymous_autofilled(
  module: &ModulePtr,
  scope_at_position: &ScopePtr,
  position: Position,
  node: *const AstNode,
  ancestry: &[*mut AstNode],
) -> Option<AutocompleteEntry> {
  let node_ptr = node as *mut AstNode;
  // SAFETY: node 来自 ancestry 遍历，指向 AST 节点池内有效 AstNode。
  let call = unsafe { ast_node_as::<AstExprCall>(node_ptr) };
  let call = if call.is_null() && ancestry.len() > 1 {
    let prev_node = ancestry[ancestry.len() - 2];
    // SAFETY: ancestry 索引已判界，指向 AST 节点池内有效 AstNode。
    unsafe { ast_node_as::<AstExprCall>(prev_node) }
  } else {
    call
  };

  if call.is_null() {
    return None;
  }
  // SAFETY: 上方已判空，call 指向 AST 节点池内有效 AstExprCall。
  let call_ref = unsafe { &*call };
  let call_loc = call_ref.base.base.location;
  let func_loc = unsafe { (*call_ref.func).base.location };
  if !call_loc.contains_closed(position) || func_loc.contains_closed(position) {
    return None;
  }

  let type_iter = module.ast_types.find(&(call_ref.func as *const AstExpr))?;

  let followed = follow_type_id(*type_iter);
  let outer_function = get_type_id::<FunctionType>(followed)?;

  let mut argument: usize = 0;
  let args_arr = &call_ref.args;
  // 找到包含 position 的参数下标
  for (i, &arg_expr) in args_arr.as_slice().iter().enumerate() {
    let arg_loc = unsafe { (*arg_expr).base.location };
    if arg_loc.contains_closed(position) {
      argument = i;
      break;
    }
  }

  if call_ref.self_ {
    argument += 1;
  }

  let (args, _) = flatten_type_pack_id(outer_function.arg_types);
  let arg_type = if argument < args.len() {
    Some(args[argument])
  } else {
    None
  }?;

  let followed = follow_type_id(arg_type);
  let type_ = match get_type_id::<FunctionType>(followed) {
    Some(type_) => type_,
    None => {
      let union_ref = get_type_id::<UnionType>(followed)?;
      // SAFETY: union_ref 指向会话期 UnionType。
      let nonnull_func = unsafe { return_first_nonnull_option_of_type::<FunctionType>(union_ref) }?;
      // SAFETY: 返回指针来自 union 选项，指向会话期 FunctionType。
      unsafe { &*nonnull_func }
    }
  };

  let entry = AutocompleteEntry {
    kind: AutocompleteEntryKind::GeneratedFunction,
    r#type: Some(arg_type),
    deprecated: false,
    wrong_index_type: false,
    type_correct: TypeCorrectKind::Correct,
    containing_extern_type: None,
    prop: None,
    documentation_symbol: None,
    tags: Default::default(),
    parens: Default::default(),
    insert_text: Some(make_anonymous_local(scope_at_position, type_)),
    indexed_with_self: false,
  };

  Some(entry)
}

fn make_anonymous_local(scope: &ScopePtr, func_ty: &FunctionType) -> String {
  let mut result = String::from("function(");

  let (args, tail) = flatten_type_pack_id(func_ty.arg_types);

  let mut first_arg = true;
  for (arg_idx, _) in args.iter().enumerate() {
    if !first_arg {
      result.push_str(", ");
    } else {
      first_arg = false;
    }

    let name = if arg_idx < func_ty.arg_names.len() {
      if let Some(arg_name) = &func_ty.arg_names[arg_idx] {
        arg_name.name.clone()
      } else {
        "a".to_string() + &arg_idx.to_string()
      }
    } else {
      "a".to_string() + &arg_idx.to_string()
    };

    if let Some(type_name) = try_get_type_name_in_scope(scope.clone(), args[arg_idx], true) {
      result.push_str(&name);
      result.push_str(": ");
      result.push_str(&type_name);
    } else {
      result.push_str(&name);
    }
  }

  if let Some(tail_id) = tail {
    let followed_tail = unsafe { follow_type_pack_id(tail_id) };
    let free_tail = get_type_pack_id::<FreeTypePack>(followed_tail);
    if is_variadic(tail_id) || free_tail.is_some() {
      if !first_arg {
        result.push_str(", ");
      }

      let mut var_arg_type = None;
      if let Some(pack) = get_type_pack_id::<VariadicTypePack>(followed_tail) {
        var_arg_type = try_get_type_name_in_scope(scope.clone(), pack.ty, true);
      }

      if let Some(var_arg_type) = var_arg_type {
        result.push_str("...: ");
        result.push_str(&var_arg_type);
      } else {
        result.push_str("...");
      }
    }
  }

  result.push(')');

  let (rets, ret_tail) = flatten_type_pack_id(func_ty.ret_types);
  let total_ret_size = rets.len() + if ret_tail.is_some() { 1 } else { 0 };
  if total_ret_size > 0
    && let Some(return_types) =
      try_get_type_name_in_scope_scope_ptr_type_pack_id_bool(scope.clone(), func_ty.ret_types, true)
  {
    result.push_str(": ");
    let wrap = total_ret_size != 1;
    if wrap {
      result.push('(');
    }
    result.push_str(&return_types);
    if wrap {
      result.push(')');
    }
  }

  result.push_str("  end");
  result
}
