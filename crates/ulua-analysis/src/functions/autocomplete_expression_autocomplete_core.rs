/// C++ `static AutocompleteContext autocompleteExpression(...)`
/// (AutocompleteCore.cpp:1478-1566).
use alloc::{string::String, vec::Vec};

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
  ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
  node_handle::OptNode, position::Position,
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
    magic_names::LUAU_AUTOCOMPLETE_ICE, to_string_symbol::to_string_symbol,
  },
  records::{
    arena_handle::{Handle, alias_ref},
    autocomplete_entry::AutocompleteEntry,
    autocomplete_result::AutocompleteResult,
    builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter,
    module::Module,
    scope::Scope,
    scope_registry::resolve_scope,
    type_arena::TypeArena,
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
  // ancestry 末位槽仍是裸指针：经句柄门面 `OptNode::from_ptr` 借出基类引用，
  // 判型/下转走句柄上生命周期正确的 `is`/`try_as`，借用半径由本函数局部句柄
  // 供给，不再锻造假 'static。
  let node_node = OptNode::from_ptr(node);

  if fflag::DebugLuauMagicVariableNames.get() {
    let ice = InternalErrorReporter {
      on_internal_error: None,
      module_name: String::new(),
    };
    // local 槽已句柄化恒非空：.get() 安全借用只读 name。
    if let Some(local) = node_node.try_as::<AstExprLocal>()
      && local.local.get().name == LUAU_AUTOCOMPLETE_ICE
    {
      ice.ice_string_location(
        &format!("{LUAU_AUTOCOMPLETE_ICE} encountered"),
        &local.base.base.location,
      );
    }
    if let Some(global) = node_node.try_as::<AstExprGlobal>()
      && global.name == LUAU_AUTOCOMPLETE_ICE
    {
      ice.ice_string_location(
        &format!("{LUAU_AUTOCOMPLETE_ICE} encountered"),
        &global.base.base.location,
      );
    }
  }

  if node_node.is::<AstExprIndexName>() {
    if let Some(expr_ref) = alias_ref(node).as_expr_const() {
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
  } else if node_node.is::<AstExprFunction>() {
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
      if function_is_expected_at(module, alias_ref(node), position).unwrap_or(false) {
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

    if let Some(ty) = find_expected_type_at(module, alias_ref(node), position) {
      autocomplete_string_singleton(ty, true, alias_ref(node), position, result);
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
