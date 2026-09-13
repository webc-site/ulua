use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
    ast_expr_error::AstExprError, ast_expr_interp_string::AstExprInterpString, ast_node::AstNode,
    position::Position,
  },
  rtti::{ast_node_as, ast_node_as_const, ast_node_is},
};

use crate::{
  functions::{
    convert_require_suggestions_to_autocomplete_entry_map::convert_require_suggestions_to_autocomplete_entry_map,
    follow_type::follow_type_id,
    get_method_containing_extern_type::get_method_containing_extern_type,
    get_string_contents::get_string_contents, get_type_alt_j::get_type_id,
    process_require_suggestions::process_require_suggestions,
  },
  records::{
    file_resolver::FileResolver, function_type::FunctionType, intersection_type::IntersectionType,
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
  file_resolver: Option<&FileResolver>,
  callback: StringCompletionCallback,
) -> Option<AutocompleteEntryMap> {
  if nodes.len() < 2 {
    return None;
  }

  let last = *nodes.last()?;
  if !unsafe { ast_node_is::<AstExprConstantString>(&*last) }
    && !is_simple_interpolated_string(last)
    && !unsafe { ast_node_is::<AstExprError>(&*last) }
  {
    return None;
  }

  if !unsafe { ast_node_is::<AstExprError>(&*last) } {
    let last_location = unsafe { (*last).location };
    if last_location.end == position || last_location.begin == position {
      return None;
    }
  }

  let candidate_node = nodes[nodes.len() - 2];
  let candidate = unsafe { ast_node_as::<AstExprCall>(candidate_node) };
  if candidate.is_null() {
    return None;
  }

  let candidate_ref = unsafe { &*candidate };

  if candidate_ref.args.size > 1 {
    let first_arg = unsafe { *candidate_ref.args.data };
    let first_arg_location = unsafe { (*first_arg).base.location };
    if !first_arg_location.contains(position) {
      return None;
    }
  }

  let it = module
    .ast_types
    .find(&(candidate_ref.func as *const AstExpr))?;
  let candidate_string = unsafe { get_string_contents(last as *const AstNode) };

  let perform_callback = |func_type: &FunctionType| -> Option<AutocompleteEntryMap> {
    for tag in &func_type.tags {
      if tag == K_REQUIRE_TAG_NAME
        && let Some(file_resolver) = file_resolver
      {
        let suggestions = file_resolver
          .require_suggester
          .as_ref()
          .and_then(|suggester| {
            suggester.get_require_suggestions_impl(&module.name, &candidate_string)
          });
        return convert_require_suggestions_to_autocomplete_entry_map(process_require_suggestions(
          suggestions,
        ));
      }

      if let Some(ret) = callback(
        tag.clone(),
        unsafe { get_method_containing_extern_type(module, candidate_ref.func) },
        candidate_string.clone(),
      ) {
        return Some(ret);
      }
    }

    None
  };

  let followed_id = follow_type_id(*it);
  if let Some(function_type) = get_type_id::<FunctionType>(followed_id) {
    return perform_callback(function_type);
  }

  if let Some(intersect) = get_type_id::<IntersectionType>(followed_id) {
    for part in &intersect.parts {
      let part = follow_type_id(*part);
      if let Some(candidate_function_type) = get_type_id::<FunctionType>(part)
        && let Some(ret) = perform_callback(candidate_function_type)
      {
        return Some(ret);
      }
    }
  }

  None
}

fn is_simple_interpolated_string(node: *const AstNode) -> bool {
  let interp_string = unsafe { ast_node_as_const::<AstExprInterpString>(node) };
  !interp_string.is_null() && unsafe { (*interp_string).expressions.size == 0 }
}
