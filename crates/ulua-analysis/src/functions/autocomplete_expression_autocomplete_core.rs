/// C++ `static AutocompleteContext autocompleteExpression(...)`
/// (AutocompleteCore.cpp:1478-1566).
use alloc::string::String;
use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
    position::Position,
  },
  rtti::{ast_node_as, ast_node_is},
};
use ulua_common::FFlag;

use crate::{
  enums::{
    autocomplete_context::AutocompleteContext, autocomplete_entry_kind::AutocompleteEntryKind,
    prop_index_type::PropIndexType, type_correct_kind::TypeCorrectKind,
  },
  functions::{
    autocomplete_if_else_expression::autocomplete_if_else_expression,
    autocomplete_props_autocomplete_core_alt_b::autocomplete_props as autocomplete_props_seed,
    autocomplete_string_singleton::autocomplete_string_singleton,
    check_type_correct_kind::check_type_correct_kind, find_expected_type_at::find_expected_type_at,
    function_is_expected_at::function_is_expected_at,
    get_paren_recommendation::get_paren_recommendation, is_being_defined::is_being_defined,
    is_binding_legal_at_current_position::is_binding_legal_at_current_position,
    to_string_symbol::to_string_symbol,
  },
  records::{
    autocomplete_entry::AutocompleteEntry, builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter, module::Module, type_arena::TypeArena,
  },
  type_aliases::{autocomplete_entry_map::AutocompleteEntryMap, scope_ptr_type::ScopePtr},
};
pub(crate) fn autocomplete_expression(
  module: &Module,
  builtin_types: &BuiltinTypes,
  type_arena: *mut TypeArena,
  ancestry: &Vec<*mut AstNode>,
  scope_at_position: &ScopePtr,
  position: Position,
  result: &mut AutocompleteEntryMap,
) -> AutocompleteContext {
  ulua_common::macros::luau_assert::LUAU_ASSERT!(!ancestry.is_empty());

  let node: *mut AstNode = ancestry[ancestry.len() - 1];

  if FFlag::DebugLuauMagicVariableNames.get() {
    let ice = InternalErrorReporter {
      on_internal_error: None,
      module_name: String::new(),
    };
    let local = unsafe { ast_node_as::<AstExprLocal>(node) };
    if !local.is_null()
      && unsafe {
        (*(*local).local)
          .name
          .operator_eq_c_char(c"_luau_autocomplete_ice")
      }
    {
      ice.ice_string_location("_luau_autocomplete_ice encountered", unsafe {
        &(*local).base.base.location
      });
    }
    let global = unsafe { ast_node_as::<AstExprGlobal>(node) };
    if !global.is_null() && unsafe { (*global).name.operator_eq_c_char(c"_luau_autocomplete_ice") }
    {
      ice.ice_string_location("_luau_autocomplete_ice encountered", unsafe {
        &(*global).base.base.location
      });
    }
  }

  if unsafe { ast_node_is::<AstExprIndexName>(&*node) } {
    let expr = unsafe { (*node).as_expr_const() };
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
  } else if unsafe {
    autocomplete_if_else_expression(
      node as *const AstNode,
      &mut ancestry.clone(),
      position,
      result,
    )
  } {
    return AutocompleteContext::Keyword;
  } else if unsafe { ast_node_is::<AstExprFunction>(&*node) } {
    return AutocompleteContext::Unknown;
  } else {
    // This is inefficient. :(
    let mut scope: Option<ScopePtr> = Some(scope_at_position.clone());

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
          let type_correct = unsafe {
            check_type_correct_kind(
              module,
              type_arena,
              builtin_types,
              node,
              position,
              binding.type_id,
            )
          };

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

      scope = scope_ref.parent.clone();
    }

    let correct_for_nil = unsafe {
      check_type_correct_kind(
        module,
        type_arena,
        builtin_types,
        node,
        position,
        builtin_types.nil_type,
      )
    };
    let correct_for_true = unsafe {
      check_type_correct_kind(
        module,
        type_arena,
        builtin_types,
        node,
        position,
        builtin_types.true_type,
      )
    };
    let correct_for_false = unsafe {
      check_type_correct_kind(
        module,
        type_arena,
        builtin_types,
        node,
        position,
        builtin_types.false_type,
      )
    };
    let correct_for_function =
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

    // SAFETY: node 来自 ancestry，整段生命周期内有效（与 C++ 相同前提）
    if let Some(ty) = find_expected_type_at(module, unsafe { &*node }, position) {
      // SAFETY: node 来自 ancestry，整段生命周期内有效（与 C++ 相同前提）
      autocomplete_string_singleton(ty, true, unsafe { &*node }, position, result);
    }
  }

  AutocompleteContext::Expression
}
