use alloc::string::String;
// C++ `kHotComments` (AutocompleteCore.cpp:43).
use alloc::vec::Vec;
use core::{
  ffi::{CStr, c_char},
  slice::from_raw_parts,
  str::from_utf8_unchecked,
};

use ulua_ast::{
  records::{
    ast_attr::AstAttrType,
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_constant_nil::AstExprConstantNil,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_interp_string::AstExprInterpString,
    ast_expr_table::AstExprTable,
    ast_name::AstName,
    ast_node::AstNode,
    ast_stat_block::AstStatBlock,
    ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr,
    ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn,
    ast_stat_if::AstStatIf,
    ast_stat_local::AstStatLocal,
    ast_stat_repeat::AstStatRepeat,
    ast_stat_while::AstStatWhile,
    ast_type_error::AstTypeError,
    ast_type_reference::AstTypeReference,
    location::Location,
    position::Position,
  },
  rtti::{ast_node_as, ast_node_is},
};
use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::{
  enums::{
    autocomplete_context::AutocompleteContext, autocomplete_entry_kind::AutocompleteEntryKind,
    prop_index_type::PropIndexType,
  },
  functions::{
    autocomplete_expression_autocomplete_core::autocomplete_expression as autocomplete_expression_into,
    autocomplete_expression_autocomplete_core_alt_b::autocomplete_expression as autocomplete_expression_result,
    autocomplete_module_types::autocomplete_module_types,
    autocomplete_props_autocomplete_core_alt_b::autocomplete_props as autocomplete_props_into,
    autocomplete_props_autocomplete_core_alt_c::autocomplete_props as autocomplete_props_result,
    autocomplete_statement::autocomplete_statement,
    autocomplete_string_params::autocomplete_string_params,
    autocomplete_string_singleton::autocomplete_string_singleton,
    autocomplete_type_names::autocomplete_type_names,
    autocomplete_while_loop_keywords::autocomplete_while_loop_keywords, extract_stat::extract_stat,
    follow_type::follow_type_id, get_type_alt_j::get_type_id, is_identifier::is_identifier,
    is_simple_interpolated_string::is_simple_interpolated_string,
    make_anonymous_autofilled::make_anonymous_autofilled,
    string_part_of_interp_string::string_part_of_interp_string,
  },
  records::{
    autocomplete_entry::AutocompleteEntry, autocomplete_result::AutocompleteResult,
    builtin_types::BuiltinTypes, file_resolver::FileResolver, module::Module, scope::Scope,
    table_type::TableType, type_arena::TypeArena,
  },
  type_aliases::{
    autocomplete_entry_map::AutocompleteEntryMap, module_ptr_module::ModulePtr,
    scope_ptr_type::ScopePtr, string_completion_callback::StringCompletionCallback,
  },
};
const K_HOT_COMMENTS: [&str; 6] = [
  "nolint",
  "nocheck",
  "nonstrict",
  "strict",
  "optimize",
  "native",
];
// C++ `kKnownAttributes` (AutocompleteCore.cpp:45).
const K_KNOWN_ATTRIBUTES: [&str; 3] = ["checked", "deprecated", "native"];
// C++ `kParseNameError` (ParseResult.h).
const K_PARSE_NAME_ERROR_CSTR: &CStr = c"%error-id%";
// C++ `kGeneratedAnonymousFunctionEntryName` (AutocompleteTypes.h:92).
const K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME: &str = "function (anonymous autofilled)";

fn is_parse_name_error_name(name: AstName) -> bool {
  if name.value.is_null() {
    return false;
  }

  unsafe { CStr::from_ptr(name.value) == K_PARSE_NAME_ERROR_CSTR }
}

fn empty_result(ancestry: &Vec<*mut AstNode>) -> AutocompleteResult {
  // C++ `return {};` — default AutocompleteResult (empty map, empty ancestry,
  // Unknown context).
  let _ = ancestry;
  AutocompleteResult {
    entry_map: Default::default(),
    ancestry: Vec::new(),
    context: AutocompleteContext::Unknown,
  }
}

fn keyword_result(
  name: &str,
  ancestry: Vec<*mut AstNode>,
  context: AutocompleteContext,
) -> AutocompleteResult {
  let mut map: AutocompleteEntryMap = Default::default();
  map.insert(
    String::from(name),
    AutocompleteEntry {
      kind: AutocompleteEntryKind::Keyword,
      ..Default::default()
    },
  );
  AutocompleteResult {
    entry_map: map,
    ancestry,
    context,
  }
}

/// `autocomplete_` 的参数包（对应 C++ 十参数签名 AutocompleteCore.h:15-26）。
pub struct AutocompleteArgs<'a> {
  pub module: &'a ModulePtr,
  pub builtin_types: &'a BuiltinTypes,
  pub type_arena: *mut TypeArena,
  pub ancestry: &'a mut Vec<*mut AstNode>,
  pub global_scope: *mut Scope,
  pub scope_at_position: &'a ScopePtr,
  pub position: Position,
  pub file_resolver: *mut FileResolver,
  pub callback: StringCompletionCallback,
  pub is_in_hot_comment: bool,
}

/// C++ `AutocompleteResult autocomplete_(...)` (AutocompleteCore.cpp:1911-2235).
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn autocomplete_(args: AutocompleteArgs<'_>) -> AutocompleteResult {
  // 按原参数顺序解包，保持与 C++ 一一对应。
  let AutocompleteArgs {
    module,
    builtin_types,
    type_arena,
    ancestry,
    global_scope,
    scope_at_position,
    position,
    file_resolver,
    callback,
    is_in_hot_comment,
  } = args;
  LUAU_TIMETRACE_SCOPE!("Luau::autocomplete_", "AutocompleteCore");
  let _ = global_scope;

  let module_ref: &Module = module;

  if is_in_hot_comment {
    let mut result: AutocompleteEntryMap = Default::default();

    for hc in K_HOT_COMMENTS.iter() {
      result
        .entry(String::from(*hc))
        .or_insert(AutocompleteEntry {
          kind: AutocompleteEntryKind::HotComment,
          ..Default::default()
        });
    }
    return AutocompleteResult {
      entry_map: result,
      ancestry: ancestry.clone(),
      context: AutocompleteContext::HotComment,
    };
  }

  let mut node: *mut AstNode = *ancestry.last().unwrap();

  let mut dummy = AstExprConstantNil::new(Location::default());
  let dummy_node = &mut dummy as *mut AstExprConstantNil as *mut AstNode;
  let mut parent: *mut AstNode = if ancestry.len() >= 2 {
    ancestry[ancestry.len() - 2]
  } else {
    dummy_node
  };

  // If we are inside a body of a function that doesn't have a completed
  // argument list, ignore the body node
  let expr_function = unsafe { ast_node_as::<AstExprFunction>(parent) };
  if !expr_function.is_null()
    && unsafe { (*expr_function).arg_location.is_none() }
    && node == unsafe { (*expr_function).body as *mut AstNode }
  {
    ancestry.pop();

    node = *ancestry.last().unwrap();
    parent = if ancestry.len() >= 2 {
      ancestry[ancestry.len() - 2]
    } else {
      dummy_node
    };
  }

  let index_name = unsafe { ast_node_as::<AstExprIndexName>(node) };
  let type_reference = unsafe { ast_node_as::<AstTypeReference>(node) };

  if !index_name.is_null() {
    let expr_key = unsafe { (*index_name).expr } as *const AstExpr;
    let it = module_ref.ast_types.find(&expr_key);
    let it = match it {
      Some(t) => t,
      None => return empty_result(ancestry),
    };

    let ty = follow_type_id(*it);
    let index_type = if unsafe { (*index_name).op } == b':' as c_char {
      PropIndexType::Colon
    } else {
      PropIndexType::Point
    };

    let map = autocomplete_props_result(
      module_ref,
      type_arena,
      builtin_types,
      ty,
      index_type,
      ancestry,
    );
    return AutocompleteResult {
      entry_map: map,
      ancestry: ancestry.clone(),
      context: AutocompleteContext::Property,
    };
  } else if !type_reference.is_null() {
    let mut pos = position;
    if let Some(prefix) = unsafe { (*type_reference).prefix } {
      let prefix_str = unsafe { CStr::from_ptr(prefix.value).to_string_lossy().into_owned() };
      return AutocompleteResult {
        entry_map: autocomplete_module_types(module_ref, scope_at_position, position, &prefix_str),
        ancestry: ancestry.clone(),
        context: AutocompleteContext::Type,
      };
    } else {
      return AutocompleteResult {
        entry_map: autocomplete_type_names(module_ref, scope_at_position, &mut pos, ancestry),
        ancestry: ancestry.clone(),
        context: AutocompleteContext::Type,
      };
    }
  } else if unsafe { ast_node_is::<AstTypeError>(&*node) } {
    let mut pos = position;
    return AutocompleteResult {
      entry_map: autocomplete_type_names(module_ref, scope_at_position, &mut pos, ancestry),
      ancestry: ancestry.clone(),
      context: AutocompleteContext::Type,
    };
  } else if !unsafe { ast_node_as::<AstExprFunction>(node) }.is_null() {
    let function = unsafe { ast_node_as::<AstExprFunction>(node) };
    let function = unsafe { &*function };
    let in_arg_annotation_gap = function.arg_location.is_some_and(|loc| {
      loc.contains_closed(position)
        && (function.args.iter().any(|arg| unsafe {
          let annotation = (**arg).annotation;
          !annotation.is_null()
            && ((*annotation).base.location.contains_closed(position)
              || (*annotation).base.location.end == position)
        }) || unsafe {
          let annotation = function.vararg_annotation;
          !annotation.is_null()
            && ((*annotation).base.location.contains_closed(position)
              || (*annotation).base.location.end == position)
        })
    });

    if in_arg_annotation_gap {
      let mut pos = position;
      return AutocompleteResult {
        entry_map: autocomplete_type_names(module_ref, scope_at_position, &mut pos, ancestry),
        ancestry: ancestry.clone(),
        context: AutocompleteContext::Type,
      };
    }
  } else if !unsafe { ast_node_as::<AstStatLocal>(node) }.is_null() {
    let stat_local = unsafe { ast_node_as::<AstStatLocal>(node) };
    let stat_local = unsafe { &*stat_local };
    let in_local_annotation_gap = stat_local.equals_sign_location.is_some_and(|loc| {
      position == loc.begin
        && stat_local.vars.iter().any(|var| unsafe {
          let annotation = (**var).annotation;
          !annotation.is_null() && (*annotation).base.location.begin <= position
        })
    });

    if in_local_annotation_gap {
      let mut pos = position;
      return AutocompleteResult {
        entry_map: autocomplete_type_names(module_ref, scope_at_position, &mut pos, ancestry),
        ancestry: ancestry.clone(),
        context: AutocompleteContext::Type,
      };
    }

    if stat_local.vars.size == 1
      && (stat_local.equals_sign_location.is_none()
        || position < stat_local.equals_sign_location.as_ref().unwrap().begin)
    {
      let mut map: AutocompleteEntryMap = Default::default();
      map.insert(
        String::from("function"),
        AutocompleteEntry {
          kind: AutocompleteEntryKind::Keyword,
          ..Default::default()
        },
      );
      return AutocompleteResult {
        entry_map: map,
        ancestry: ancestry.clone(),
        context: AutocompleteContext::Unknown,
      };
    } else if stat_local.equals_sign_location.is_some()
      && position >= stat_local.equals_sign_location.as_ref().unwrap().end
    {
      return autocomplete_expression_result(
        module_ref,
        builtin_types,
        type_arena,
        ancestry,
        scope_at_position,
        position,
      );
    } else {
      return empty_result(ancestry);
    }
  }

  let stat_for = extract_stat::<AstStatFor>(ancestry);
  if !stat_for.is_null() {
    let stat_for = unsafe { &*stat_for };
    if !stat_for.has_do || position < stat_for.do_location.begin {
      if unsafe { (*stat_for.from).base.location.contains_closed(position) }
        || unsafe { (*stat_for.to).base.location.contains_closed(position) }
        || (!stat_for.step.is_null()
          && unsafe { (*stat_for.step).base.location.contains_closed(position) })
      {
        return autocomplete_expression_result(
          module_ref,
          builtin_types,
          type_arena,
          ancestry,
          scope_at_position,
          position,
        );
      }

      if !unsafe { ast_node_is::<AstExprError>(&(*stat_for.from).base) }
        && !unsafe { ast_node_is::<AstExprError>(&(*stat_for.to).base) }
        && (stat_for.step.is_null()
          || !unsafe { ast_node_is::<AstExprError>(&(*stat_for.step).base) })
      {
        return keyword_result("do", ancestry.clone(), AutocompleteContext::Keyword);
      }
      return empty_result(ancestry);
    }

    let mut pos = position;
    return AutocompleteResult {
      entry_map: autocomplete_statement(module_ref, ancestry, scope_at_position, &mut pos),
      ancestry: ancestry.clone(),
      context: AutocompleteContext::Statement,
    };
  }

  let stat_for_in_parent = unsafe { ast_node_as::<AstStatForIn>(parent) };
  if !stat_for_in_parent.is_null()
    && (unsafe { ast_node_is::<AstStatBlock>(&*node) } || unsafe { is_identifier(node) })
  {
    let stat_for_in = unsafe { &*stat_for_in_parent };
    if !stat_for_in.has_in || position <= stat_for_in.in_location.begin {
      let last_name = unsafe { *stat_for_in.vars.data.add(stat_for_in.vars.size - 1) };
      if is_parse_name_error_name(unsafe { (*last_name).name })
        || unsafe { (*last_name).location.contains_closed(position) }
      {
        // Here we are either working with a missing binding (as would be
        // the case in a bare "for" keyword) or the cursor is still touching
        // a binding name. The user is still typing a new name, so we should
        // not offer any suggestions.
        return empty_result(ancestry);
      }

      return keyword_result("in", ancestry.clone(), AutocompleteContext::Keyword);
    }

    if !stat_for_in.has_do || position <= stat_for_in.do_location.begin {
      ulua_common::macros::luau_assert::LUAU_ASSERT!(stat_for_in.values.size > 0);
      let last_expr = unsafe { *stat_for_in.values.data.add(stat_for_in.values.size - 1) };

      if unsafe { (*last_expr).base.location.contains_closed(position) } {
        return autocomplete_expression_result(
          module_ref,
          builtin_types,
          type_arena,
          ancestry,
          scope_at_position,
          position,
        );
      }

      if position > unsafe { (*last_expr).base.location.end } {
        return keyword_result("do", ancestry.clone(), AutocompleteContext::Keyword);
      }

      return empty_result(ancestry); // Not sure what this means
    }
  } else {
    let stat_for_in = extract_stat::<AstStatForIn>(ancestry);
    if !stat_for_in.is_null() {
      let stat_for_in = unsafe { &*stat_for_in };
      if !stat_for_in.has_in || position <= stat_for_in.in_location.begin {
        if stat_for_in.vars.size == 0 {
          return empty_result(ancestry);
        }

        let last_name = unsafe { *stat_for_in.vars.data.add(stat_for_in.vars.size - 1) };
        if is_parse_name_error_name(unsafe { (*last_name).name })
          || unsafe { (*last_name).location.contains_closed(position) }
        {
          return empty_result(ancestry);
        }

        return keyword_result("in", ancestry.clone(), AutocompleteContext::Keyword);
      }

      if !stat_for_in.has_do || position <= stat_for_in.do_location.begin {
        if stat_for_in.values.size == 0 {
          return empty_result(ancestry);
        }

        let last_expr = unsafe { *stat_for_in.values.data.add(stat_for_in.values.size - 1) };

        if unsafe { (*last_expr).base.location.contains_closed(position) } {
          return autocomplete_expression_result(
            module_ref,
            builtin_types,
            type_arena,
            ancestry,
            scope_at_position,
            position,
          );
        }

        if position > unsafe { (*last_expr).base.location.end } {
          return keyword_result("do", ancestry.clone(), AutocompleteContext::Keyword);
        }

        return empty_result(ancestry);
      }

      if !stat_for_in.has_do {
        return keyword_result("do", ancestry.clone(), AutocompleteContext::Keyword);
      }

      let mut pos = position;
      return AutocompleteResult {
        entry_map: autocomplete_statement(module_ref, ancestry, scope_at_position, &mut pos),
        ancestry: ancestry.clone(),
        context: AutocompleteContext::Statement,
      };
    }

    let stat_while_parent = unsafe { ast_node_as::<AstStatWhile>(parent) };
    if unsafe { ast_node_is::<AstStatBlock>(&*node) } && !stat_while_parent.is_null() {
      let stat_while = unsafe { &*stat_while_parent };
      if !stat_while.has_do
        && !unsafe { ast_node_is::<AstStatError>(&*(stat_while.condition as *mut AstNode)) }
        && position > unsafe { (*stat_while.condition).base.location.end }
      {
        return autocomplete_while_loop_keywords(ancestry.clone());
      }

      if !stat_while.has_do || position < stat_while.do_location.begin {
        return autocomplete_expression_result(
          module_ref,
          builtin_types,
          type_arena,
          ancestry,
          scope_at_position,
          position,
        );
      }

      if stat_while.has_do && position > stat_while.do_location.end {
        let mut pos = position;
        return AutocompleteResult {
          entry_map: autocomplete_statement(module_ref, ancestry, scope_at_position, &mut pos),
          ancestry: ancestry.clone(),
          context: AutocompleteContext::Statement,
        };
      }
    } else {
      let stat_while = extract_stat::<AstStatWhile>(ancestry);
      let while_condition_ok = !stat_while.is_null()
        && unsafe {
          let sw = &*stat_while;
          (!sw.has_do || sw.do_location.contains_closed(position))
            && !sw.condition.is_null()
            && !(*sw.condition).base.location.contains_closed(position)
        };
      if while_condition_ok {
        return autocomplete_while_loop_keywords(ancestry.clone());
      }

      let stat_if_node = unsafe { ast_node_as::<AstStatIf>(node) };
      if !stat_if_node.is_null() && unsafe { (*stat_if_node).else_location.is_none() } {
        let mut map: AutocompleteEntryMap = Default::default();
        map.insert(
          String::from("else"),
          AutocompleteEntry {
            kind: AutocompleteEntryKind::Keyword,
            ..Default::default()
          },
        );
        map.insert(
          String::from("elseif"),
          AutocompleteEntry {
            kind: AutocompleteEntryKind::Keyword,
            ..Default::default()
          },
        );
        return AutocompleteResult {
          entry_map: map,
          ancestry: ancestry.clone(),
          context: AutocompleteContext::Keyword,
        };
      }

      let stat_if_parent = unsafe { ast_node_as::<AstStatIf>(parent) };
      if !stat_if_parent.is_null() && unsafe { ast_node_is::<AstStatBlock>(&*node) } {
        let stat_if = unsafe { &*stat_if_parent };
        if unsafe { ast_node_is::<AstExprError>(&(*stat_if.condition).base) } {
          return autocomplete_expression_result(
            module_ref,
            builtin_types,
            type_arena,
            ancestry,
            scope_at_position,
            position,
          );
        } else if stat_if.then_location.is_none()
          || stat_if
            .then_location
            .as_ref()
            .unwrap()
            .contains_closed(position)
        {
          return keyword_result("then", ancestry.clone(), AutocompleteContext::Keyword);
        }
      } else {
        let stat_if = extract_stat::<AstStatIf>(ancestry);
        let if_then_ok = !stat_if.is_null()
          && unsafe {
            let si = &*stat_if;
            (si.then_location.is_none()
              || si.then_location.as_ref().unwrap().contains_closed(position))
              && (!si.condition.is_null()
                && !(*si.condition).base.location.contains_closed(position))
          };
        if if_then_ok {
          let mut ret: AutocompleteEntryMap = Default::default();
          ret.insert(
            String::from("then"),
            AutocompleteEntry {
              kind: AutocompleteEntryKind::Keyword,
              ..Default::default()
            },
          );
          ret.insert(
            String::from("and"),
            AutocompleteEntry {
              kind: AutocompleteEntryKind::Keyword,
              ..Default::default()
            },
          );
          ret.insert(
            String::from("or"),
            AutocompleteEntry {
              kind: AutocompleteEntryKind::Keyword,
              ..Default::default()
            },
          );
          return AutocompleteResult {
            entry_map: ret,
            ancestry: ancestry.clone(),
            context: AutocompleteContext::Keyword,
          };
        }

        let stat_repeat_node = unsafe { ast_node_as::<AstStatRepeat>(node) };
        if !stat_repeat_node.is_null()
          && unsafe { ast_node_is::<AstExprError>(&(*(*stat_repeat_node).condition).base) }
        {
          return autocomplete_expression_result(
            module_ref,
            builtin_types,
            type_arena,
            ancestry,
            scope_at_position,
            position,
          );
        }

        let stat_repeat = extract_stat::<AstStatRepeat>(ancestry);
        if !stat_repeat.is_null() {
          let mut pos = position;
          return AutocompleteResult {
            entry_map: autocomplete_statement(module_ref, ancestry, scope_at_position, &mut pos),
            ancestry: ancestry.clone(),
            context: AutocompleteContext::Statement,
          };
        }

        let expr_table_parent = unsafe { ast_node_as::<AstExprTable>(parent) };
        if !expr_table_parent.is_null()
          && (unsafe { ast_node_is::<AstExprGlobal>(&*node) }
            || unsafe { ast_node_is::<AstExprConstantString>(&*node) }
            || unsafe { ast_node_is::<AstExprInterpString>(&*node) })
        {
          let expr_table = unsafe { &*expr_table_parent };
          let items = expr_table.items.as_slice();
          for item in items {
            let key = item.key;
            let value = item.value;
            // If item doesn't have a key, maybe the value is actually the key
            let matched: bool = if !key.is_null() {
              (key as *mut AstNode) == node
            } else {
              let is_global = unsafe { ast_node_is::<AstExprGlobal>(&*node) };
              is_global && ((value as *mut AstNode) == node)
            };
            if matched {
              let expr_table_key = expr_table_parent as *const AstExpr;
              if let Some(it) = module_ref.ast_expected_types.find(&expr_table_key) {
                let mut result = autocomplete_props_result(
                  module_ref,
                  type_arena,
                  builtin_types,
                  *it,
                  PropIndexType::Key,
                  ancestry,
                );

                let node_expr = unsafe { (*node).as_expr_const() };
                if let Some(node_it) = module_ref.ast_expected_types.find(&node_expr) {
                  autocomplete_string_singleton(
                    *node_it,
                    !unsafe { ast_node_is::<AstExprConstantString>(&*node) },
                    // SAFETY: node 来自 ancestry，整段生命周期内有效（与 C++ 相同前提）
                    unsafe { &*node },
                    position,
                    &mut result,
                  );
                }

                if key.is_null() {
                  // If there is "no key," it may be that the user
                  // intends for the current token to be the key,
                  // but has yet to type the `=` sign.
                  //
                  // If the key type is a union of singleton
                  // strings, suggest those too.
                  let ttv = get_type_id::<TableType>(follow_type_id(*it));
                  if let Some(ttv) = ttv
                    && let Some(indexer) = ttv.indexer.as_ref()
                  {
                    autocomplete_string_singleton(
                      indexer.index_type,
                      false,
                      unsafe { &*node },
                      position,
                      &mut result,
                    );
                  }
                }

                // Remove keys that are already completed
                for item in items {
                  if item.key.is_null() {
                    continue;
                  }

                  let string_key =
                    unsafe { ast_node_as::<AstExprConstantString>(item.key as *mut AstNode) };
                  if !string_key.is_null() {
                    let s = unsafe {
                      from_utf8_unchecked(from_raw_parts(
                        (*string_key).value.data as *const u8,
                        (*string_key).value.size,
                      ))
                      .to_string()
                    };
                    result.remove(&s);
                  }
                }

                // If we know for sure that a key is being written,
                // do not offer general expression suggestions
                if key.is_null() {
                  autocomplete_expression_into(
                    module_ref,
                    builtin_types,
                    type_arena,
                    ancestry,
                    scope_at_position,
                    position,
                    &mut result,
                  );
                }

                return AutocompleteResult {
                  entry_map: result,
                  ancestry: ancestry.clone(),
                  context: AutocompleteContext::Property,
                };
              }

              break;
            }
          }
        } else {
          let expr_table_node = unsafe { ast_node_as::<AstExprTable>(node) };
          if !expr_table_node.is_null() {
            let expr_table = unsafe { &*expr_table_node };
            let mut result: AutocompleteEntryMap = Default::default();

            let expr_table_key = expr_table_node as *const AstExpr;
            if let Some(it) = module_ref.ast_expected_types.find(&expr_table_key) {
              result = autocomplete_props_result(
                module_ref,
                type_arena,
                builtin_types,
                *it,
                PropIndexType::Key,
                ancestry,
              );

              // If the key type is a union of singleton strings,
              // suggest those too.
              let ttv = get_type_id::<TableType>(follow_type_id(*it));
              if let Some(ttv) = ttv
                && let Some(indexer) = ttv.indexer.as_ref()
              {
                autocomplete_string_singleton(
                  indexer.index_type,
                  false,
                  unsafe { &*node },
                  position,
                  &mut result,
                );
              }

              // Remove keys that are already completed
              let items = expr_table.items.as_slice();
              for item in items {
                if item.key.is_null() {
                  continue;
                }

                let string_key =
                  unsafe { ast_node_as::<AstExprConstantString>(item.key as *mut AstNode) };
                if !string_key.is_null() {
                  let s = unsafe {
                    from_utf8_unchecked(from_raw_parts(
                      (*string_key).value.data as *const u8,
                      (*string_key).value.size,
                    ))
                    .to_string()
                  };
                  result.remove(&s);
                }
              }
            }

            // Also offer general expression suggestions
            autocomplete_expression_into(
              module_ref,
              builtin_types,
              type_arena,
              ancestry,
              scope_at_position,
              position,
              &mut result,
            );

            return AutocompleteResult {
              entry_map: result,
              ancestry: ancestry.clone(),
              context: AutocompleteContext::Property,
            };
          } else if unsafe { is_identifier(node) }
            && (unsafe { ast_node_is::<AstStatExpr>(&*parent) }
              || unsafe { ast_node_is::<AstStatError>(&*parent) })
          {
            let mut pos = position;
            return AutocompleteResult {
              entry_map: autocomplete_statement(module_ref, ancestry, scope_at_position, &mut pos),
              ancestry: ancestry.clone(),
              context: AutocompleteContext::Statement,
            };
          }
        }
      }
    }
  }

  if let Some(ret) = autocomplete_string_params(
    module,
    ancestry,
    position,
    // SAFETY: file_resolver 有效性由 Frontend 契约保证（C++ 同款可空指针）
    unsafe { file_resolver.as_ref() },
    callback,
  ) {
    return AutocompleteResult {
      entry_map: ret,
      ancestry: ancestry.clone(),
      context: AutocompleteContext::String,
    };
  } else if unsafe { ast_node_is::<AstExprConstantString>(&*node) }
    || unsafe { is_simple_interpolated_string(node as *const AstNode) }
  {
    let mut result: AutocompleteEntryMap = Default::default();

    if ancestry.len() >= 2 {
      let prev = ancestry[ancestry.len() - 2];
      let idx_expr = unsafe { ast_node_as::<AstExprIndexExpr>(prev) };
      let bin_expr = unsafe { ast_node_as::<AstExprBinary>(prev) };
      if !idx_expr.is_null() {
        let key = unsafe { (*idx_expr).expr } as *const AstExpr;
        if let Some(it) = module_ref.ast_types.find(&key) {
          autocomplete_props_into(
            module_ref,
            type_arena,
            builtin_types,
            follow_type_id(*it),
            PropIndexType::Point,
            ancestry,
            &mut result,
          );
        }
      } else if !bin_expr.is_null() {
        let op = unsafe { (*bin_expr).op };
        if op == AstExprBinaryOp::CompareEq || op == AstExprBinaryOp::CompareNe {
          let other = if node == unsafe { (*bin_expr).left } as *mut AstNode {
            unsafe { (*bin_expr).right }
          } else {
            unsafe { (*bin_expr).left }
          } as *const AstExpr;
          if let Some(it) = module_ref.ast_types.find(&other) {
            autocomplete_string_singleton(*it, false, unsafe { &*node }, position, &mut result);
          }
        }
      }
    }

    let node_expr = unsafe { (*node).as_expr_const() };
    if let Some(it) = module_ref.ast_expected_types.find(&node_expr) {
      autocomplete_string_singleton(*it, false, unsafe { &*node }, position, &mut result);
    }

    return AutocompleteResult {
      entry_map: result,
      ancestry: ancestry.clone(),
      context: AutocompleteContext::String,
    };
  } else if unsafe { string_part_of_interp_string(node as *const AstNode, position) } {
    // We're not a simple interpolated string, we're something like
    // `a{"b"}@1`, and we can't know what to format to
    let map: AutocompleteEntryMap = Default::default();
    return AutocompleteResult {
      entry_map: map,
      ancestry: ancestry.clone(),
      context: AutocompleteContext::String,
    };
  } else {
    let func = unsafe { ast_node_as::<AstExprFunction>(node) };
    if !func.is_null() {
      for &attr in unsafe { (*func).attributes.as_slice() } {
        let attr_ref = unsafe { &*attr };
        if attr_ref.base.location.begin <= position
          && position <= attr_ref.base.location.end
          && attr_ref.r#type == AstAttrType::Unknown
        {
          let mut ret: AutocompleteEntryMap = Default::default();
          for attr in K_KNOWN_ATTRIBUTES.iter() {
            ret.insert(
              String::from(*attr),
              AutocompleteEntry {
                kind: AutocompleteEntryKind::Keyword,
                ..Default::default()
              },
            );
          }
          return AutocompleteResult {
            entry_map: ret,
            ancestry: ancestry.clone(),
            context: AutocompleteContext::Keyword,
          };
        }
      }
    }
  }

  if unsafe { ast_node_is::<AstExprConstantNumber>(&*node) } {
    return empty_result(ancestry);
  }

  if !unsafe { (*node).as_expr_const() }.is_null() {
    let mut ret = autocomplete_expression_result(
      module_ref,
      builtin_types,
      type_arena,
      ancestry,
      scope_at_position,
      position,
    );
    if let Some(generated) = make_anonymous_autofilled(
      module,
      scope_at_position,
      position,
      node as *const AstNode,
      ancestry,
    ) {
      ret.entry_map.insert(
        String::from(K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME),
        generated,
      );
    }
    return ret;
  } else if !unsafe { (*node).as_stat_const() }.is_null() {
    let mut pos = position;
    return AutocompleteResult {
      entry_map: autocomplete_statement(module_ref, ancestry, scope_at_position, &mut pos),
      ancestry: ancestry.clone(),
      context: AutocompleteContext::Statement,
    };
  }

  empty_result(ancestry)
}
