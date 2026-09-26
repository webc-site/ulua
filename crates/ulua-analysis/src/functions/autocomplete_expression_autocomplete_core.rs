/// C++ `static AutocompleteContext autocompleteExpression(...)`
/// (AutocompleteCore.cpp:1478-1566).
use alloc::{string::String, vec::Vec};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
    position::Position,
  },
  rtti::{ast_node_is_ptr, ast_node_try_as_ptr},
};
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{
    autocomplete_context::AutocompleteContext, autocomplete_entry_kind::AutocompleteEntryKind,
    prop_index_type::PropIndexType, type_correct_kind::TypeCorrectKind,
  },
  functions::{
    autocomplete_if_else_expression::autocomplete_if_else_expression,
    autocomplete_props_autocomplete_core::autocomplete_props_seed,
    autocomplete_string_singleton::autocomplete_string_singleton,
    check_type_correct_kind::check_type_correct_kind, find_expected_type_at::find_expected_type_at,
    function_is_expected_at::function_is_expected_at,
    get_paren_recommendation::get_paren_recommendation, is_being_defined::is_being_defined,
    is_binding_legal_at_current_position::is_binding_legal_at_current_position,
    to_string_symbol::to_string_symbol,
  },
  records::{
    arena_handle::Handle, autocomplete_entry::AutocompleteEntry,
    autocomplete_result::AutocompleteResult, builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter, module::Module, scope::Scope,
    scope_registry::resolve_scope, type_arena::TypeArena,
  },
  type_aliases::{autocomplete_entry_map::AutocompleteEntryMap, scope_ptr_type::ScopePtr},
};

pub(crate) fn autocomplete_expression(
  module: &Module,
  builtin_types: &BuiltinTypes,
  type_arena: Handle<TypeArena>,
  ancestry: &Vec<*mut AstNode>,
  scope_at_position: &ScopePtr,
  position: Position,
  result: &mut AutocompleteEntryMap,
) -> AutocompleteContext {
  LUAU_ASSERT!(!ancestry.is_empty());

  let node: *mut AstNode = ancestry[ancestry.len() - 1];

  if fflag::DebugLuauMagicVariableNames.get() {
    let ice = InternalErrorReporter {
      on_internal_error: None,
      module_name: String::new(),
    };
    // Safety: `node` 为 ancestry 末位（LUAU_ASSERT 已证非空），指向解析 arena 的存活
    // 节点；`ast_node_try_as_ptr` 按 class_index 判型，命中即合法下转为只读借用。
    // local 槽已句柄化恒非空：.get() 安全借用只读 name。
    if let Some(local) = unsafe { ast_node_try_as_ptr::<AstExprLocal>(node) }
      && local.local.get().name == "_luau_autocomplete_ice"
    {
      ice.ice_string_location(
        "_luau_autocomplete_ice encountered",
        &local.base.base.location,
      );
    }
    if let Some(global) = unsafe { ast_node_try_as_ptr::<AstExprGlobal>(node) }
      && global.name == "_luau_autocomplete_ice"
    {
      ice.ice_string_location(
        "_luau_autocomplete_ice encountered",
        &global.base.base.location,
      );
    }
  }

  if unsafe { ast_node_is_ptr::<AstExprIndexName>(node) } {
    // Safety: 上一行已确认 node 动态类型为 AstExprIndexName（AstExpr 子类），解引用
    // 合法；`as_expr_const` 只返回偏移 0 处 AstExpr 基类指针，用作 `module.ast_types`
    // 的指针身份查找键，不形成引用别名。
    if let Some(expr_ref) = unsafe { (*node).as_expr_const() } {
      let expr: *const AstExpr = expr_ref;
      if let Some(it) = module.ast_types.find(&expr) {
        autocomplete_props_seed(
          module,
          type_arena,
          builtin_types,
          *it,
          PropIndexType::Point,
          ancestry,
          result,
        );
      }
    }
  } else if autocomplete_if_else_expression(
    node as *const AstNode,
    &mut ancestry.clone(),
    position,
    result,
  ) {
    return AutocompleteContext::Keyword;
  } else if unsafe { ast_node_is_ptr::<AstExprFunction>(node) } {
    return AutocompleteContext::Unknown;
  } else {
    // This is inefficient. :(
    let mut scope: Option<&Scope> = Some(scope_at_position.as_ref());

    while let Some(scope_ref) = scope {
      for (name, binding) in &scope_ref.bindings {
        if !is_binding_legal_at_current_position(name, binding, position) {
          continue;
        }

        if is_being_defined(ancestry, name) {
          continue;
        }

        let n = to_string_symbol(name);
        if !result.contains_key(&n) {
          // node 是 ancestry 末位的 arena 存活节点，type_arena 为调用方传入的
          // Frontend 所属模块类型 arena，binding.type_id 指向同一 arena；
          // 调用期三方均只读。
          let type_correct = check_type_correct_kind(
            module,
            type_arena,
            builtin_types,
            node,
            position,
            binding.type_id,
          );

          result.insert(
            n.clone(),
            AutocompleteEntry {
              kind: AutocompleteEntryKind::Binding,
              r#type: Some(binding.type_id),
              deprecated: binding.deprecated,
              wrong_index_type: false,
              type_correct,
              containing_extern_type: None,
              prop: None,
              documentation_symbol: binding.documentation_symbol.clone(),
              tags: Default::default(),
              parens: get_paren_recommendation(binding.type_id, ancestry, type_correct),
              insert_text: None,
              indexed_with_self: false,
            },
          );
        }
      }

      scope = scope_ref.parent.and_then(resolve_scope);
    }

    // 指针前提与 binding 查询处一致——node 为 ancestry 末位存活 arena 节点、
    // type_arena 独占可用；候选类型换成 builtin 的 nil/true/false 单例 TypeId
    // （活在 builtin_types 所属 arena 内），被调方只读取类型图、不留存指针。
    let correct_for_nil = check_type_correct_kind(
      module,
      type_arena,
      builtin_types,
      node,
      position,
      builtin_types.nil_type,
    );
    let correct_for_true = check_type_correct_kind(
      module,
      type_arena,
      builtin_types,
      node,
      position,
      builtin_types.true_type,
    );
    let correct_for_false = check_type_correct_kind(
      module,
      type_arena,
      builtin_types,
      node,
      position,
      builtin_types.false_type,
    );
    let correct_for_function =
      // Safety: `function_is_expected_at` 内部解引用 `node`（ancestry 末位的存活
      // arena 节点）并只读查询 module 的期望类型表；module 借用在整个本函数期间
      // 有效且此处无写入，满足其非空/存活指针前提。
      if unsafe { function_is_expected_at(module, node, position) }.unwrap_or(false) {
        TypeCorrectKind::Correct
      } else {
        TypeCorrectKind::None
      };

    result.insert(
      String::from("if"),
      AutocompleteEntry {
        kind: AutocompleteEntryKind::Keyword,
        r#type: None,
        deprecated: false,
        wrong_index_type: false,
        ..Default::default()
      },
    );
    result.insert(
      String::from("true"),
      AutocompleteEntry {
        kind: AutocompleteEntryKind::Keyword,
        r#type: Some(builtin_types.boolean_type),
        deprecated: false,
        wrong_index_type: false,
        type_correct: correct_for_true,
        ..Default::default()
      },
    );
    result.insert(
      String::from("false"),
      AutocompleteEntry {
        kind: AutocompleteEntryKind::Keyword,
        r#type: Some(builtin_types.boolean_type),
        deprecated: false,
        wrong_index_type: false,
        type_correct: correct_for_false,
        ..Default::default()
      },
    );
    result.insert(
      String::from("nil"),
      AutocompleteEntry {
        kind: AutocompleteEntryKind::Keyword,
        r#type: Some(builtin_types.nil_type),
        deprecated: false,
        wrong_index_type: false,
        type_correct: correct_for_nil,
        ..Default::default()
      },
    );
    result.insert(
      String::from("not"),
      AutocompleteEntry {
        kind: AutocompleteEntryKind::Keyword,
        ..Default::default()
      },
    );
    result.insert(
      String::from("function"),
      AutocompleteEntry {
        kind: AutocompleteEntryKind::Keyword,
        r#type: None,
        deprecated: false,
        wrong_index_type: false,
        type_correct: correct_for_function,
        ..Default::default()
      },
    );

    // Safety: `&*node` 把 ancestry 末位指针裸化为共享引用——该节点由解析 arena 持有、
    // 全程存活，本函数为只读补全查询，引用存续期间无别处可变访问。
    if let Some(ty) = find_expected_type_at(module, unsafe { &*node }, position) {
      // Safety: 同一 `node` 再次裸化借用，`autocomplete_string_singleton` 只读取节点
      // 的 class_index/位置信息，与上方共享借用同为只读，无非别名冲突。
      autocomplete_string_singleton(ty, true, unsafe { &*node }, position, result);
    }
  }

  AutocompleteContext::Expression
}

/// C++ `static AutocompleteResult autocompleteExpression(...)`
/// (AutocompleteCore.cpp:1568-1580), the six-parameter overload that builds a
/// fresh result.
pub(crate) fn autocomplete_expression_result(
  module: &Module,
  builtin_types: &BuiltinTypes,
  type_arena: Handle<TypeArena>,
  ancestry: &Vec<*mut AstNode>,
  scope_at_position: &ScopePtr,
  position: Position,
) -> AutocompleteResult {
  let mut result: AutocompleteEntryMap = Default::default();
  let context: AutocompleteContext = autocomplete_expression(
    module,
    builtin_types,
    type_arena,
    ancestry,
    scope_at_position,
    position,
    &mut result,
  );
  AutocompleteResult {
    entry_map: result,
    ancestry: ancestry.clone(),
    context,
  }
}
