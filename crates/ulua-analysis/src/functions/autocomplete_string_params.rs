use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
    ast_expr_error::AstExprError, ast_node::AstNode, position::Position,
  },
  rtti::{ast_node_is, ast_node_try_as},
};

use crate::{
  functions::{
    convert_require_suggestions_to_autocomplete_entry_map::convert_require_suggestions_to_autocomplete_entry_map,
    follow_type, get_method_containing_extern_type::get_method_containing_extern_type,
    get_string_contents::get_string_contents, get_type,
    is_simple_interpolated_string::is_simple_interpolated_string,
    process_require_suggestions::process_require_suggestions,
  },
  records::{
    arena_handle::{alias_opt, alias_ref},
    file_resolver::FileResolver,
    function_type::FunctionType,
    intersection_type::IntersectionType,
  },
  type_aliases::{
    autocomplete_entry_map::AutocompleteEntryMap, module_ptr_module::ModulePtr,
    string_completion_callback::StringCompletionCallback,
  },
};
const K_REQUIRE_TAG_NAME: &str = "require";

pub fn autocomplete_string_params(
  module: &ModulePtr,
  nodes: &[*mut AstNode],
  position: Position,
  // `dyn` 保留：`FileResolver` 实现方集合运行期开放（宿主跨 crate 注入）。
  file_resolver: Option<&dyn FileResolver>,
  callback: StringCompletionCallback,
) -> Option<AutocompleteEntryMap> {
  if nodes.len() < 2 {
    return None;
  }

  let last = *nodes.last()?;
  let last_ref = alias_opt(last)?;
  if !ast_node_is::<AstExprConstantString>(last_ref)
    && !is_simple_interpolated_string(Some(last_ref))
    && !ast_node_is::<AstExprError>(last_ref)
  {
    return None;
  }

  if !ast_node_is::<AstExprError>(last_ref) {
    let last_location = last_ref.location;
    if last_location.end == position || last_location.begin == position {
      return None;
    }
  }

  let candidate_node = nodes[nodes.len() - 2];
  let candidate_ref = alias_opt(candidate_node).and_then(ast_node_try_as::<AstExprCall>)?;

  if candidate_ref.args.size > 1 {
    // size>1 ⇒ 元素 0 存在，用安全 `as_slice` 取代裸指针 `*args.data` 解引用。
    let first_arg = candidate_ref.args.as_slice()[0];
    let first_arg_location = alias_ref(first_arg).base.location;
    if !first_arg_location.contains(position) {
      return None;
    }
  }

  let it = module
    .ast_types
    .find(&(candidate_ref.func as *const AstExpr))?;
  let candidate_string = get_string_contents(Some(last_ref));

  let perform_callback = |func_type: &FunctionType| -> Option<AutocompleteEntryMap> {
    for tag in &func_type.tags {
      if tag == K_REQUIRE_TAG_NAME
        && let Some(file_resolver) = file_resolver
      {
        let suggestions = file_resolver.require_suggester().and_then(|suggester| {
          suggester.get_require_suggestions_impl(&module.name, &candidate_string)
        });
        return convert_require_suggestions_to_autocomplete_entry_map(process_require_suggestions(
          suggestions,
        ));
      }

      if let Some(ret) = callback(
        tag.clone(),
        // `candidate_ref.func` 是已判空的 AstExprCall 内 parser 绑定的
        // 非空 AstExpr 句柄，随 AST arena 存活；被调方仅做 RTTI 下转与只读遍历。
        get_method_containing_extern_type(module, candidate_ref.func),
        candidate_string.clone(),
      ) {
        return Some(ret);
      }
    }

    None
  };

  let followed_id = follow_type::follow(*it);
  if let Some(function_type) = get_type::get::<FunctionType>(followed_id) {
    return perform_callback(function_type);
  }

  if let Some(intersect) = get_type::get::<IntersectionType>(followed_id) {
    for part in &intersect.parts {
      let part = follow_type::follow(*part);
      if let Some(candidate_function_type) = get_type::get::<FunctionType>(part)
        && let Some(ret) = perform_callback(candidate_function_type)
      {
        return Some(ret);
      }
    }
  }

  None
}
