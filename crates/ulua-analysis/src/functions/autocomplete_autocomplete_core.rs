use alloc::string::{String, ToString};
// C++ `kHotComments` (AutocompleteCore.cpp:43).
use alloc::vec::Vec;
use core::{ffi::c_char, str::from_utf8};

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
    ast_expr_table::{AstExprTable, Item},
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
  rtti::{ast_node_as, ast_node_is, ast_node_try_as},
};
use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_timetrace_scope::LUAU_TIMETRACE_SCOPE};

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
// C++ `kParseNameError` (ParseResult.h). AstName 由词法器 intern，必为合法
// ASCII/UTF-8，用字节串比较即可，免去 CStr 的 unsafe。
const K_PARSE_NAME_ERROR_BYTES: &[u8] = b"%error-id%";
// C++ `kGeneratedAnonymousFunctionEntryName` (AutocompleteTypes.h:92).
const K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME: &str = "function (anonymous autofilled)";

fn is_parse_name_error_name(name: AstName) -> bool {
  // 空名 → as_bytes 空切片，必不等于非空常量。
  name.as_bytes() == K_PARSE_NAME_ERROR_BYTES
}

/// 对照 C++ `Remove keys that are already completed`：把 items 中已写成
/// 字符串字面量的键从候选表移除（C++ AutocompleteCore.cpp:479-495）。
fn remove_completed_string_keys(result: &mut AutocompleteEntryMap, items: &[Item]) {
  for item in items {
    if item.key.is_null() {
      continue;
    }

    // SAFETY: key 判非空，AST 节点由 arena 持有，生命周期覆盖全程；
    // class_index 命中后 repr(C) 单继承保证 cast 有效。
    let Some(key) =
      (unsafe { ast_node_as::<AstExprConstantString>(item.key as *mut AstNode).as_ref() })
    else {
      continue;
    };
    // C++ 用原始字节构串（std::string 任意字节合法）；这里安全接口读字节，
    // 非法 UTF-8 回退空串键删除（与 crate 内现行为一致），免 String 分配。
    match from_utf8(key.value.as_bytes()) {
      Ok(s) => {
        result.remove(s);
      }
      Err(_) => {
        result.remove("");
      }
    }
  }
}

fn empty_result() -> AutocompleteResult {
  // C++ `return {};` — default AutocompleteResult (empty map, empty ancestry,
  // Unknown context).
  AutocompleteResult {
    entry_map: Default::default(),
    ancestry: Vec::new(),
    context: AutocompleteContext::Unknown,
  }
}

/// 对照 C++ `ancestry.rbegin() + 1`：取倒数第二个祖先，深度不足时用 dummy 节点。
fn parent_at(ancestry: &[*mut AstNode], dummy: *mut AstNode) -> *mut AstNode {
  match ancestry.len() {
    0 | 1 => dummy,
    n => ancestry[n - 2],
  }
}

/// 对照 C++：构造一组 Keyword 候选（单关键词即单元素特例）。
fn keywords_result(
  names: &[&str],
  ancestry: Vec<*mut AstNode>,
  context: AutocompleteContext,
) -> AutocompleteResult {
  let mut map: AutocompleteEntryMap = Default::default();
  for name in names {
    map.insert(
      String::from(*name),
      AutocompleteEntry {
        kind: AutocompleteEntryKind::Keyword,
        ..Default::default()
      },
    );
  }
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
  pub file_resolver: *mut dyn FileResolver,
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
    global_scope: _,
    scope_at_position,
    position,
    file_resolver,
    callback,
    is_in_hot_comment,
  } = args;
  LUAU_TIMETRACE_SCOPE!("Luau::autocomplete_", "AutocompleteCore");

  let module_ref: &Module = module;

  if is_in_hot_comment {
    let mut result: AutocompleteEntryMap = Default::default();

    for hc in K_HOT_COMMENTS {
      result.entry(String::from(hc)).or_insert(AutocompleteEntry {
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

  // C++ `node = ancestry.back()`：Frontend 契约保证 ancestry 非空。
  let mut node: *mut AstNode = *ancestry.last().unwrap();

  let mut dummy = AstExprConstantNil::new(Location::default());
  let dummy_node = &mut dummy as *mut AstExprConstantNil as *mut AstNode;
  let mut parent: *mut AstNode = parent_at(ancestry, dummy_node);

  // If we are inside a body of a function that doesn't have a completed
  // argument list, ignore the body node
  // SAFETY: parent 指向 arena 存活节点或 dummy（栈上存活至函数结束）。
  if let Some(func) = ast_node_try_as::<AstExprFunction>(unsafe { &*parent })
    && func.arg_location.is_none()
    && node == func.body as *mut AstNode
  {
    ancestry.pop();

    node = *ancestry.last().unwrap();
    parent = parent_at(ancestry, dummy_node);
  }

  // SAFETY: node/parent 指向 arena 存活节点（parent 可能为 dummy，栈上存活）；
  // 引用建立后全程只读，后续变更仅作用于 ancestry Vec 与裸指针字段。
  let node_ref: &AstNode = unsafe { &*node };
  let parent_ref: &AstNode = unsafe { &*parent };

  let index_name = ast_node_try_as::<AstExprIndexName>(node_ref);
  let type_reference = ast_node_try_as::<AstTypeReference>(node_ref);

  if let Some(index_name) = index_name {
    let expr_key = index_name.expr as *const AstExpr;
    let it = match module_ref.ast_types.find(&expr_key) {
      Some(t) => t,
      None => return empty_result(),
    };

    let ty = follow_type_id(*it);
    let index_type = if index_name.op == b':' as c_char {
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
  } else if let Some(type_reference) = type_reference {
    let mut pos = position;
    if let Some(prefix) = type_reference.prefix {
      let prefix_str = prefix.as_str_or_empty().to_string();
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
  } else if ast_node_is::<AstTypeError>(node_ref) {
    let mut pos = position;
    return AutocompleteResult {
      entry_map: autocomplete_type_names(module_ref, scope_at_position, &mut pos, ancestry),
      ancestry: ancestry.clone(),
      context: AutocompleteContext::Type,
    };
  } else if let Some(stat_local) = ast_node_try_as::<AstStatLocal>(node_ref) {
    let eq_location = stat_local.equals_sign_location.as_ref();
    if stat_local.vars.size == 1 && eq_location.is_none_or(|l| position < l.begin) {
      return keywords_result(
        &["function"],
        ancestry.clone(),
        AutocompleteContext::Unknown,
      );
    } else if eq_location.is_some_and(|l| position >= l.end) {
      return autocomplete_expression_result(
        module_ref,
        builtin_types,
        type_arena,
        ancestry,
        scope_at_position,
        position,
      );
    } else {
      return empty_result();
    }
  }

  let stat_for = extract_stat::<AstStatFor>(ancestry);
  // SAFETY: extract_stat 判空后才解引用，AST 节点由 arena 持有。
  if let Some(stat_for) = unsafe { stat_for.as_ref() } {
    // SAFETY: from/to 由解析器保证非空（C++ 同款前提），arena 持有。
    let from = unsafe { &*stat_for.from };
    let to = unsafe { &*stat_for.to };
    // SAFETY: step 可空，as_ref 对 null 返回 None（短路保护）。
    let step = unsafe { stat_for.step.as_ref() };
    if !stat_for.has_do || position < stat_for.do_location.begin {
      if from.base.location.contains_closed(position)
        || to.base.location.contains_closed(position)
        || step.is_some_and(|s| s.base.location.contains_closed(position))
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

      if !ast_node_is::<AstExprError>(&from.base)
        && !ast_node_is::<AstExprError>(&to.base)
        && step.is_none_or(|s| !ast_node_is::<AstExprError>(&s.base))
      {
        return keywords_result(&["do"], ancestry.clone(), AutocompleteContext::Keyword);
      }
      return empty_result();
    }

    let mut pos = position;
    return AutocompleteResult {
      entry_map: autocomplete_statement(module_ref, ancestry, scope_at_position, &mut pos),
      ancestry: ancestry.clone(),
      context: AutocompleteContext::Statement,
    };
  }

  let stat_for_in_parent = ast_node_try_as::<AstStatForIn>(parent_ref);
  if let Some(stat_for_in) = stat_for_in_parent
    && (ast_node_is::<AstStatBlock>(node_ref) || unsafe { is_identifier(node) })
  {
    if !stat_for_in.has_in || position <= stat_for_in.in_location.begin {
      // SAFETY: !has_in 分支内 vars 至少一个元素（解析器保证，C++ 同款前提）；
      // as_slice 对空数组安全，last 判空兜底。
      let Some(&last_name) = stat_for_in.vars.as_slice().last() else {
        return empty_result();
      };
      // SAFETY: last_name 为判空后的名字节点，arena 持有。
      let last_name = unsafe { &*last_name };
      if is_parse_name_error_name(last_name.name) || last_name.location.contains_closed(position) {
        // Here we are either working with a missing binding (as would be
        // the case in a bare "for" keyword) or the cursor is still touching
        // a binding name. The user is still typing a new name, so we should
        // not offer any suggestions.
        return empty_result();
      }

      return keywords_result(&["in"], ancestry.clone(), AutocompleteContext::Keyword);
    }

    if !stat_for_in.has_do || position <= stat_for_in.do_location.begin {
      LUAU_ASSERT!(stat_for_in.values.size > 0);
      // SAFETY: values 至少一个元素（LUAU_ASSERT 同款前提），arena 持有；
      // as_slice 对空数组安全，last 判空兜底。
      let Some(&last_expr) = stat_for_in.values.as_slice().last() else {
        return empty_result();
      };
      // SAFETY: last_expr 为最后一个值表达式，解析器保证非空，arena 持有。
      let last_expr = unsafe { &*last_expr };

      if last_expr.base.location.contains_closed(position) {
        return autocomplete_expression_result(
          module_ref,
          builtin_types,
          type_arena,
          ancestry,
          scope_at_position,
          position,
        );
      }

      if position > last_expr.base.location.end {
        return keywords_result(&["do"], ancestry.clone(), AutocompleteContext::Keyword);
      }

      return empty_result(); // Not sure what this means
    }
  } else {
    let stat_for_in = extract_stat::<AstStatForIn>(ancestry);
    // SAFETY: extract_stat 判空后才解引用，AST 节点由 arena 持有。
    if let Some(stat_for_in) = unsafe { stat_for_in.as_ref() } {
      // The AST looks a bit differently if the cursor is at a position where
      // only the "do" keyword is allowed. ex "for f in f do"
      // （C++ AutocompleteCore.cpp:2223-2231）
      if !stat_for_in.has_do {
        return keywords_result(&["do"], ancestry.clone(), AutocompleteContext::Keyword);
      }

      let mut pos = position;
      return AutocompleteResult {
        entry_map: autocomplete_statement(module_ref, ancestry, scope_at_position, &mut pos),
        ancestry: ancestry.clone(),
        context: AutocompleteContext::Statement,
      };
    }

    let stat_while_parent = ast_node_try_as::<AstStatWhile>(parent_ref);
    if ast_node_is::<AstStatBlock>(node_ref)
      && let Some(stat_while) = stat_while_parent
    {
      // SAFETY: !has_do 时 condition 由解析器保证非空（&& 短路后才解引用，
      // C++ 同款前提）；as_ref 对 null 返回 None。
      let condition = unsafe { stat_while.condition.as_ref() };
      if !stat_while.has_do
        && condition
          .is_some_and(|c| !ast_node_is::<AstStatError>(&c.base) && position > c.base.location.end)
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
      // SAFETY: extract_stat 判空后才解引用，AST 节点由 arena 持有。
      let while_condition_ok = unsafe { stat_while.as_ref() }.is_some_and(|sw| {
        (!sw.has_do || sw.do_location.contains_closed(position))
          && unsafe { sw.condition.as_ref() }
            .is_some_and(|c| !c.base.location.contains_closed(position))
      });
      if while_condition_ok {
        return autocomplete_while_loop_keywords(ancestry.clone());
      }

      let stat_if_node = ast_node_try_as::<AstStatIf>(node_ref);
      if let Some(if_node) = stat_if_node
        && if_node.else_location.is_none()
      {
        return keywords_result(
          &["else", "elseif"],
          ancestry.clone(),
          AutocompleteContext::Keyword,
        );
      }

      let stat_if_parent = ast_node_try_as::<AstStatIf>(parent_ref);
      if let Some(stat_if) = stat_if_parent
        && ast_node_is::<AstStatBlock>(node_ref)
      {
        // SAFETY: 完成解析的 if 语句 condition 由解析器保证非空（C++ 同款前提）；
        // as_ref 对 null 返回 None。
        let condition = unsafe { stat_if.condition.as_ref() };
        if condition.is_some_and(ast_node_is::<AstExprError>) {
          return autocomplete_expression_result(
            module_ref,
            builtin_types,
            type_arena,
            ancestry,
            scope_at_position,
            position,
          );
        } else if stat_if
          .then_location
          .as_ref()
          .is_none_or(|l| l.contains_closed(position))
        {
          return keywords_result(&["then"], ancestry.clone(), AutocompleteContext::Keyword);
        }
      } else {
        let stat_if = extract_stat::<AstStatIf>(ancestry);
        // SAFETY: extract_stat 判空后才解引用，AST 节点由 arena 持有。
        let if_then_ok = unsafe { stat_if.as_ref() }.is_some_and(|si| {
          si.then_location
            .as_ref()
            .is_none_or(|l| l.contains_closed(position))
            && unsafe { si.condition.as_ref() }
              .is_some_and(|c| !c.base.location.contains_closed(position))
        });
        if if_then_ok {
          return keywords_result(
            &["then", "and", "or"],
            ancestry.clone(),
            AutocompleteContext::Keyword,
          );
        }

        let stat_repeat_node = ast_node_try_as::<AstStatRepeat>(node_ref);
        if let Some(stat_repeat_node) = stat_repeat_node {
          // SAFETY: 完成解析的 repeat 语句 condition 由解析器保证非空（C++
          // 同款前提）；as_ref 对 null 返回 None。
          if (unsafe { stat_repeat_node.condition.as_ref() })
            .is_some_and(ast_node_is::<AstExprError>)
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
        }

        let stat_repeat = extract_stat::<AstStatRepeat>(ancestry);
        // SAFETY: extract_stat 判空后才解引用，AST 节点由 arena 持有。
        if (unsafe { stat_repeat.as_ref() }).is_some() {
          let mut pos = position;
          return AutocompleteResult {
            entry_map: autocomplete_statement(module_ref, ancestry, scope_at_position, &mut pos),
            ancestry: ancestry.clone(),
            context: AutocompleteContext::Statement,
          };
        }

        let expr_table = ast_node_try_as::<AstExprTable>(parent_ref);
        if let Some(expr_table) = expr_table
          && (ast_node_is::<AstExprGlobal>(node_ref)
            || ast_node_is::<AstExprConstantString>(node_ref)
            || ast_node_is::<AstExprInterpString>(node_ref))
        {
          let items = expr_table.items.as_slice();
          for item in items {
            let key = item.key;
            let value = item.value;
            // If item doesn't have a key, maybe the value is actually the key
            let matched = if !key.is_null() {
              (key as *mut AstNode) == node
            } else {
              ast_node_is::<AstExprGlobal>(node_ref) && ((value as *mut AstNode) == node)
            };
            if matched {
              // repr(C) 单继承：AstExprTable 与 AstNode 同址，upcast 等价。
              let expr_table_key = parent as *const AstExpr;
              if let Some(it) = module_ref.ast_expected_types.find(&expr_table_key) {
                let mut result = autocomplete_props_result(
                  module_ref,
                  type_arena,
                  builtin_types,
                  *it,
                  PropIndexType::Key,
                  ancestry,
                );

                let node_expr = node_ref.as_expr_const();
                if let Some(node_it) = module_ref.ast_expected_types.find(&node_expr) {
                  autocomplete_string_singleton(
                    *node_it,
                    !ast_node_is::<AstExprConstantString>(node_ref),
                    node_ref,
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
                      node_ref,
                      position,
                      &mut result,
                    );
                  }
                }

                remove_completed_string_keys(&mut result, items);
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
          let expr_table_node = ast_node_try_as::<AstExprTable>(node_ref);
          if let Some(expr_table) = expr_table_node {
            let mut result: AutocompleteEntryMap = Default::default();

            // repr(C) 单继承：AstExprTable 与 AstNode 同址，upcast 等价。
            let expr_table_key = node as *const AstExpr;
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
                  node_ref,
                  position,
                  &mut result,
                );
              }

              let items = expr_table.items.as_slice();
              remove_completed_string_keys(&mut result, items);
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
            && (ast_node_is::<AstStatExpr>(parent_ref) || ast_node_is::<AstStatError>(parent_ref))
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
    // SAFETY: file_resolver 有效性由 Frontend 契约保证（C++ 同款可空指针）。
    unsafe { file_resolver.as_ref() },
    callback,
  ) {
    return AutocompleteResult {
      entry_map: ret,
      ancestry: ancestry.clone(),
      context: AutocompleteContext::String,
    };
  } else if ast_node_is::<AstExprConstantString>(node_ref)
    || unsafe { is_simple_interpolated_string(node as *const AstNode) }
  {
    let mut result: AutocompleteEntryMap = Default::default();

    if ancestry.len() >= 2 {
      // SAFETY: prev 取自 ancestry，指向 arena 存活节点。
      let prev_ref: &AstNode = unsafe { &*ancestry[ancestry.len() - 2] };
      if let Some(idx_expr) = ast_node_try_as::<AstExprIndexExpr>(prev_ref) {
        let key = idx_expr.expr as *const AstExpr;
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
      } else if let Some(bin_expr) = ast_node_try_as::<AstExprBinary>(prev_ref) {
        let op = bin_expr.op;
        if op == AstExprBinaryOp::CompareEq || op == AstExprBinaryOp::CompareNe {
          // repr(C) 单继承：AstExpr 与 AstNode 同址，upcast 等价。
          let other: *const AstExpr = if node == bin_expr.left as *mut AstNode {
            bin_expr.right.cast_const()
          } else {
            bin_expr.left.cast_const()
          };
          if let Some(it) = module_ref.ast_types.find(&other) {
            autocomplete_string_singleton(*it, false, node_ref, position, &mut result);
          }
        }
      }
    }

    let node_expr = node_ref.as_expr_const();
    if let Some(it) = module_ref.ast_expected_types.find(&node_expr) {
      autocomplete_string_singleton(*it, false, node_ref, position, &mut result);
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
  } else if let Some(func) = ast_node_try_as::<AstExprFunction>(node_ref) {
    for attr in func.attributes.as_slice() {
      // SAFETY: attr 为判空后的属性节点，arena 持有。
      let Some(attr_ref) = (unsafe { (*attr).as_ref() }) else {
        continue;
      };
      if attr_ref.base.location.contains_closed(position) && attr_ref.r#type == AstAttrType::Unknown
      {
        return keywords_result(
          &K_KNOWN_ATTRIBUTES,
          ancestry.clone(),
          AutocompleteContext::Keyword,
        );
      }
    }
  }

  if ast_node_is::<AstExprConstantNumber>(node_ref) {
    return empty_result();
  }

  if !node_ref.as_expr_const().is_null() {
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
  } else if !node_ref.as_stat_const().is_null() {
    let mut pos = position;
    return AutocompleteResult {
      entry_map: autocomplete_statement(module_ref, ancestry, scope_at_position, &mut pos),
      context: AutocompleteContext::Statement,
      ancestry: ancestry.clone(),
    };
  }

  empty_result()
}
