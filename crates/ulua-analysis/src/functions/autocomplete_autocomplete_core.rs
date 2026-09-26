use alloc::string::{String, ToString};
// C++ `kHotComments` (AutocompleteCore.cpp:43).
use alloc::vec::Vec;
use core::{
  ptr::{NonNull, from_mut, null},
  str::from_utf8,
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
  rtti::{AstNodePtr, ast_node_is, ast_node_try_as, ast_node_try_as_ptr},
};
use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_timetrace_scope::LUAU_TIMETRACE_SCOPE};

use crate::{
  enums::{
    autocomplete_context::AutocompleteContext, autocomplete_entry_kind::AutocompleteEntryKind,
    prop_index_type::PropIndexType,
  },
  functions::{
    autocomplete_expression_autocomplete_core::{
      autocomplete_expression as autocomplete_expression_into, autocomplete_expression_result,
    },
    autocomplete_keywords::insert_keyword,
    autocomplete_module_types::autocomplete_module_types,
    autocomplete_props_autocomplete_core::{
      autocomplete_props_result, autocomplete_props_seed as autocomplete_props_into,
    },
    autocomplete_statement::autocomplete_statement,
    autocomplete_string_params::autocomplete_string_params,
    autocomplete_string_singleton::autocomplete_string_singleton,
    autocomplete_type_names::autocomplete_type_names,
    autocomplete_while_loop_keywords::autocomplete_while_loop_keywords,
    extract_stat::extract_stat,
    follow_type, get_type,
    is_identifier::is_identifier,
    is_simple_interpolated_string::is_simple_interpolated_string,
    make_anonymous_autofilled::make_anonymous_autofilled,
    string_part_of_interp_string::string_part_of_interp_string,
  },
  records::{
    arena_handle::Handle, autocomplete_entry::AutocompleteEntry,
    autocomplete_result::AutocompleteResult, builtin_types::BuiltinTypes,
    file_resolver::FileResolver, module::Module, scope::Scope, table_type::TableType,
    type_arena::TypeArena,
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
    // Safety: `item.key` 是 `AstExprTable.items` 里的 `*mut AstExpr`，指向
    // `Module::allocator`（AST arena）持有的 repr(C) 节点；`ast_node_try_as_ptr`
    // 门面自带判空与 class_index 判型，命中返回只读借用，未命中返回 None。
    let Some(key) = (unsafe { ast_node_try_as_ptr::<AstExprConstantString>(item.key) }) else {
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
pub(crate) fn keywords_result(
  names: &[&str],
  ancestry: Vec<*mut AstNode>,
  context: AutocompleteContext,
) -> AutocompleteResult {
  let mut map: AutocompleteEntryMap = Default::default();
  for name in names {
    insert_keyword(&mut map, name);
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
  pub type_arena: Handle<TypeArena>,
  pub ancestry: &'a mut Vec<*mut AstNode>,
  pub global_scope: *mut Scope,
  pub scope_at_position: &'a ScopePtr,
  pub position: Position,
  /// `dyn` 保留：`FileResolver` 实现方集合运行期开放（宿主跨 crate 注入）。
  pub file_resolver: Option<&'a dyn FileResolver>,
  pub callback: StringCompletionCallback,
  pub is_in_hot_comment: bool,
}

/// C++ `AutocompleteResult autocomplete_(...)` (AutocompleteCore.cpp:1911-2235).
///
/// # Safety
///
/// 本函数把 C++ 的 `autocomplete(...)` 自由函数直译成裸指针形态，函数体内对传入的
/// 裸指针与 `ancestry` 元素做只读解引用。调用方须逐项保证：
/// - `module`：存活的 `&ModulePtr`，其 `Module::allocator`（AST arena）拥有 `ancestry`
///   中所有 `*mut AstNode` 指向的节点；这些节点须在整次调用期间存活且不被并发可变借用
///   ——函数体内每一处 `&*node`/`&*parent`/`extract_stat`/`as_ref()` 的有效性都以此为根基。
/// - `ancestry`：非空（内部 `ancestry.last().expect(..)`）；其元素要么为 null（`extract_stat`、
///   `parent_at` 会判空），要么是指向上述 arena 存活 AST 节点的指针。
/// - `type_arena`：转发给下游按引用使用的 `TypeArena` 句柄，非空与调用期内
///   存活由 `Handle` 类型契约编码（C++ 侧为 `TypeArena&`）。
/// - `file_resolver`：`Option<&'a dyn FileResolver>`，`None` 表示无 resolver
///   （等价于 C++ 允许传 nullptr），`Some` 时引用须指向调用内存活对象。
/// - `builtin_types`/`scope_at_position`/`callback`/`position`/`is_in_hot_comment`：
///   普通引用或值，仅需满足各自的 `'a` 生命周期，无额外裸指针不变量（`global_scope` 在本
///   函数中未使用）。
///
/// 违反 `module`/`ancestry` 中的存活性或非空约定，会使函数体内的裸指针解引用成为 UB。
pub(crate) unsafe fn autocomplete_(args: AutocompleteArgs<'_>) -> AutocompleteResult {
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
  let mut node: *mut AstNode = *ancestry
    .last()
    .expect("# Safety 入参契约：ancestry 非空（Frontend 保证）");

  let mut dummy = AstExprConstantNil::new(Location::default());
  let dummy_node = from_mut(&mut dummy).as_ast_node();
  let mut parent: *mut AstNode = parent_at(ancestry, dummy_node);

  // If we are inside a body of a function that doesn't have a completed
  // argument list, ignore the body node
  // Safety: `parent` = `parent_at(ancestry, dummy_node)`——`ancestry.len() < 2` 时返回栈上
  // `dummy`（`AstExprConstantNil`，存活至本函数结束），否则返回 `ancestry[n-2]`，即函数级
  // 契约保证的 module arena 存活 AST 节点；两者经 repr(C) 首字段与 `AstNode` 同址，只读解引用有效。
  if let Some(func) = ast_node_try_as::<AstExprFunction>(unsafe { &*parent })
    && func.arg_location.is_none()
    && node == func.body.as_ast_node()
  {
    ancestry.pop();

    node = *ancestry
      .last()
      .expect("pop 后仍非空：ancestry 恒留 dummy/根帧（Frontend 契约）");
    parent = parent_at(ancestry, dummy_node);
  }

  // Safety: `node` = `*ancestry.last().expect(..)`，函数级契约保证 `ancestry` 非空且末位指向
  // module arena 中存活的 repr(C) AST 节点；本次调用对该节点只读、无并存 `&mut`，共享解引用成立。
  let node_ref: &AstNode = unsafe { &*node };
  // Safety: `parent` 同上方 `parent_at` 结果（arena 节点或栈上 dummy，均存活），由仍在期的
  // `ancestry`/`dummy` 借用保证有效；后续 `ancestry` 的变更只动 Vec 槽位，不改这些 arena 节点本身。
  let parent_ref: &AstNode = unsafe { &*parent };

  let index_name = ast_node_try_as::<AstExprIndexName>(node_ref);
  let type_reference = ast_node_try_as::<AstTypeReference>(node_ref);

  if let Some(index_name) = index_name {
    // expr 已句柄化恒非空；ast_types 身份键为既有指针形态，经 as_ptr 桥接。
    let expr_key = index_name.expr.as_ptr().cast_const();
    let it = match module_ref.ast_types.find(&expr_key) {
      Some(t) => t,
      None => return empty_result(),
    };

    let ty = follow_type::follow(*it);
    let index_type = if index_name.op == b':' {
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

  if let Some(stat_for) = extract_stat::<AstStatFor>(ancestry) {
    // from/to 已句柄化为非空 Node（解析器必建上下界，随本 `&AstStatFor`
    // 借用存续），`.get()` 直出安全共享引用；step 落可空 OptNode，`get()`
    // 即 Option（下方以 Option 短路保护）。
    let from = stat_for.from.get();
    let to = stat_for.to.get();
    let step = stat_for.step.get();
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
    && (ast_node_is::<AstStatBlock>(node_ref) || is_identifier(node_ref))
  {
    if !stat_for_in.has_in || position <= stat_for_in.in_location.begin {
      // SAFETY: !has_in 分支内 vars 至少一个元素（解析器保证，C++ 同款前提）；
      // as_slice 对空数组安全，last 判空兜底。
      let Some(&last_name) = stat_for_in.vars.as_slice().last() else {
        return empty_result();
      };
      // Safety: `last_name` 是 `stat_for_in.vars: AstArray<*mut AstLocal>` 的末元素，上一行
      // `last()` 已证数组非空；解析器保证每个绑定名槽为指向 module arena 存活 `AstLocal` 的
      // 非空指针（C++ 直接 `vars.data[...]` 解引用的同款前提），故共享解引用有效。
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
      // Safety: `last_expr` 取自 `stat_for_in.values: AstArray<*mut AstExpr>` 末元素，上一行
      // `LUAU_ASSERT!(values.size > 0)` 与 `last()` 共同保证取到非空槽位；值表达式由解析器写入
      // module arena 的存活 `AstExpr` 指针，仅共享解引用、无别名。
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
    if let Some(stat_for_in) = extract_stat::<AstStatForIn>(ancestry) {
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
      // Safety: `stat_while` 来自对 `parent_ref`（module arena 存活节点）的 `ast_node_try_as`，
      // 是 `&AstStatWhile`；其 `condition` 字段在表达式未写完时可空，`as_ref()` 把 null 折成
      // `None`，非空时指向解析器写入 module arena 的存活 `AstExpr`，仅共享解引用。
      let condition = stat_while.condition.get();
      if !stat_while.has_do
        && !ast_node_is::<AstStatError>(&condition.base)
        && position > condition.base.location.end
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
      let while_condition_ok = extract_stat::<AstStatWhile>(ancestry).is_some_and(|sw| {
        (!sw.has_do || sw.do_location.contains_closed(position))
          && !sw.condition.base.location.contains_closed(position)
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
        let condition = stat_if.condition.get();
        if ast_node_is::<AstExprError>(condition) {
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
        let if_then_ok = extract_stat::<AstStatIf>(ancestry).is_some_and(|si| {
          si.then_location
            .as_ref()
            .is_none_or(|l| l.contains_closed(position))
            && !si.condition.base.location.contains_closed(position)
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
          // condition 已句柄化为 Node（parser 恒非空，无名/未完条件亦由
          // parse_expr 交回 alloc 的 AstExprError 占位节点），`.get()` 即安全
          // 只读视图，原判空折叠的 `as_ref()` 门面消失。
          if ast_node_is::<AstExprError>(stat_repeat_node.condition.get()) {
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

        if extract_stat::<AstStatRepeat>(ancestry).is_some() {
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
              (key.as_ast_node()) == node
            } else {
              ast_node_is::<AstExprGlobal>(node_ref) && ((value.as_ast_node()) == node)
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

                let node_expr = node_ref
                  .as_expr_const()
                  .map_or(null(), |e| NonNull::from(e).as_ptr());
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
                  let ttv = get_type::get::<TableType>(follow_type::follow(*it));
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
              let ttv = get_type::get::<TableType>(follow_type::follow(*it));
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
          } else if is_identifier(node_ref)
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
    // `file_resolver` 已是带生命周期的共享引用（`None` 即 C++ 的空 resolver），
    // 直接透传给下游只读语义的 `Option<&dyn FileResolver>` 形参，无 unsafe。
    file_resolver,
    callback,
  ) {
    return AutocompleteResult {
      entry_map: ret,
      ancestry: ancestry.clone(),
      context: AutocompleteContext::String,
    };
  } else if ast_node_is::<AstExprConstantString>(node_ref)
    // Safety: 传入的 `node` 即 `*ancestry.last().expect(..)`，函数级契约保证它指向 module arena
    // 中存活的 AST 节点；`is_simple_interpolated_string` 为 unsafe fn，仅需 null-or-live 入参
    // 且内部自行判空，故本次转发满足其调用契约。
    || unsafe { is_simple_interpolated_string(node as *const AstNode) }
  {
    let mut result: AutocompleteEntryMap = Default::default();

    if ancestry.len() >= 2 {
      // Safety: 上行 `ancestry.len() >= 2` 保证下标 `len-2` 有效；`ancestry` 元素由函数级契约
      // 给出，均为指向 module arena 存活 repr(C) AST 节点的裸指针，本函数对其只读、无别名。
      let prev_ref: &AstNode = unsafe { &*ancestry[ancestry.len() - 2] };
      if let Some(idx_expr) = ast_node_try_as::<AstExprIndexExpr>(prev_ref) {
        // expr 已句柄化恒非空；ast_types 身份键为既有指针形态，经 as_ptr 桥接。
        let key = idx_expr.expr.as_ptr().cast_const();
        if let Some(it) = module_ref.ast_types.find(&key) {
          autocomplete_props_into(
            module_ref,
            type_arena,
            builtin_types,
            follow_type::follow(*it),
            PropIndexType::Point,
            ancestry,
            &mut result,
          );
        }
      } else if let Some(bin_expr) = ast_node_try_as::<AstExprBinary>(prev_ref) {
        let op = bin_expr.op;
        if matches!(op, AstExprBinaryOp::CompareEq | AstExprBinaryOp::CompareNe) {
          // repr(C) 单继承：AstExpr 与 AstNode 同址，upcast 等价。
          // left/right 已句柄化恒非空；as_ptr 桥接既有指针门面（cast_const 保只读键形）。
          let other: *const AstExpr = if node == bin_expr.left.as_ast_node() {
            bin_expr.right.as_ptr().cast_const()
          } else {
            bin_expr.left.as_ptr().cast_const()
          };
          if let Some(it) = module_ref.ast_types.find(&other) {
            autocomplete_string_singleton(*it, false, node_ref, position, &mut result);
          }
        }
      }
    }

    let node_expr = node_ref
      .as_expr_const()
      .map_or(null(), |e| NonNull::from(e).as_ptr());
    if let Some(it) = module_ref.ast_expected_types.find(&node_expr) {
      autocomplete_string_singleton(*it, false, node_ref, position, &mut result);
    }

    return AutocompleteResult {
      entry_map: result,
      ancestry: ancestry.clone(),
      context: AutocompleteContext::String,
    };
  } else if
  // Safety: `node` 即 `*ancestry.last().expect(..)`，函数级契约保证它指向 module arena 存活的
  // AST 节点；`string_part_of_interp_string` 为 unsafe fn，契约要求 null-or-live 入参且内部
  // 逐层判空，`position` 为普通值，故本次转发满足其调用前提。
  unsafe { string_part_of_interp_string(node as *const AstNode, position) } {
    // We're not a simple interpolated string, we're something like
    // `a{"b"}@1`, and we can't know what to format to
    let map: AutocompleteEntryMap = Default::default();
    return AutocompleteResult {
      entry_map: map,
      ancestry: ancestry.clone(),
      context: AutocompleteContext::String,
    };
  } else if let Some(func) = ast_node_try_as::<AstExprFunction>(node_ref) {
    for attr in func.attributes.iter() {
      if attr.base.location.contains_closed(position) && attr.r#type == AstAttrType::Unknown {
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

  if node_ref.as_expr_const().is_some() {
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
  } else if node_ref.as_stat_const().is_some() {
    let mut pos = position;
    return AutocompleteResult {
      entry_map: autocomplete_statement(module_ref, ancestry, scope_at_position, &mut pos),
      context: AutocompleteContext::Statement,
      ancestry: ancestry.clone(),
    };
  }

  empty_result()
}
