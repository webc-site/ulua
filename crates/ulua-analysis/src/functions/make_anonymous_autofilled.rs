use alloc::{borrow::Cow, string::String};

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_node::AstNode, node_handle::OptNode,
  position::Position,
};

use crate::{
  enums::{autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind},
  functions::{
    flatten_type_pack::flatten_type_pack_id,
    follow_type, follow_type_pack, get_type, get_type_pack,
    is_variadic_type_pack::is_variadic,
    return_first_nonnull_option_of_type::return_first_nonnull_option_of_type,
    try_get_type_name_in_scope_autocomplete_core::{
      try_get_type_name_in_scope, try_get_type_name_in_scope_scope_ptr_type_pack_id_bool,
    },
  },
  records::{
    arena_handle::alias_ref, autocomplete_entry::AutocompleteEntry, free_type_pack::FreeTypePack,
    function_type::FunctionType, union_type::UnionType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr},
};
pub(crate) fn make_anonymous_autofilled(
  module: &ModulePtr,
  scope_at_position: &ScopePtr,
  position: Position,
  node: *const AstNode,
  ancestry: &[*mut AstNode],
) -> Option<AutocompleteEntry> {
  let node_ptr = node.cast_mut();
  // `node`/`ancestry` 来自 AST 遍历，指向 AST 节点池内有效 AstNode（或 null）：
  // 先经句柄门面 `OptNode::from_ptr` 折叠可空性，判型下转走句柄上的生命周期
  // 正确 `try_as`，借用半径由本函数作用域的两个局部句柄供给，不再锻造假 'static。
  let node_handle = OptNode::from_ptr(node_ptr);
  let prev_handle = if ancestry.len() > 1 {
    OptNode::from_ptr(ancestry[ancestry.len() - 2])
  } else {
    OptNode::default()
  };
  let call_ref = node_handle
    .try_as::<AstExprCall>()
    .or_else(|| prev_handle.try_as::<AstExprCall>())?;
  let call_loc = call_ref.base.base.location;
  let func_loc = alias_ref(call_ref.func).base.location;
  if !call_loc.contains_closed(position) || func_loc.contains_closed(position) {
    return None;
  }

  let type_iter = module.ast_types.find(&(call_ref.func as *const AstExpr))?;

  let followed = follow_type::follow(*type_iter);
  let outer_function = get_type::get::<FunctionType>(followed)?;

  let mut argument: usize = 0;
  let args_arr = &call_ref.args;
  // 找到包含 position 的参数下标
  for (i, &arg_expr) in args_arr.as_slice().iter().enumerate() {
    let arg_loc = alias_ref(arg_expr).base.location;
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

  let followed = follow_type::follow(arg_type);
  let type_ = match get_type::get::<FunctionType>(followed) {
    Some(type_) => type_,
    None => {
      let union_ref = get_type::get::<UnionType>(followed)?;
      return_first_nonnull_option_of_type::<FunctionType>(union_ref)?
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
  for (arg_idx, &arg) in args.iter().enumerate() {
    if !first_arg {
      result.push_str(", ");
    } else {
      first_arg = false;
    }

    // 具名参数零拷贝借用，缺名时回退生成 `a{i}`
    let name = match func_ty.arg_names.get(arg_idx).and_then(|n| n.as_ref()) {
      Some(arg_name) => Cow::Borrowed(arg_name.name.as_str()),
      None => Cow::Owned(format!("a{arg_idx}")),
    };

    result.push_str(&name);
    if let Some(type_name) = try_get_type_name_in_scope(scope.clone(), arg, true) {
      result.push_str(": ");
      result.push_str(&type_name);
    }
  }

  if let Some(tail_id) = tail {
    let followed_tail = follow_type_pack::follow(tail_id);
    let free_tail = get_type_pack::get::<FreeTypePack>(followed_tail);
    if is_variadic(tail_id) || free_tail.is_some() {
      if !first_arg {
        result.push_str(", ");
      }

      let mut var_arg_type = None;
      if let Some(pack) = get_type_pack::get::<VariadicTypePack>(followed_tail) {
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
