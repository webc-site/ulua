// C++ `kParseNameError` from ParseResult.h.
use alloc::{collections::BTreeSet, string::String, sync::Arc, vec::Vec};
use std::collections::HashSet;

use ulua_ast::records::{ast_node::AstNode, position::Position};

use crate::{
  enums::{
    autocomplete_entry_kind::AutocompleteEntryKind,
    parentheses_recommendation::ParenthesesRecommendation, prop_index_type::PropIndexType,
    type_correct_kind::TypeCorrectKind,
  },
  functions::{
    check_type_correct_kind::check_type_correct_kind, check_type_match::check_type_match,
    first::first, follow_type::follow_type_id, get_paren_recommendation::get_paren_recommendation,
    get_singleton_type::get_singleton_type, get_type_alt_j::get_type_id, is_nil::is_nil,
  },
  records::{
    autocomplete_entry::AutocompleteEntry, builtin_types::BuiltinTypes, extern_type::ExternType,
    function_type::FunctionType, intersection_type::IntersectionType,
    metatable_type::MetatableType, module::Module, primitive_type::PrimitiveType,
    property_type::Property, property_type_path, scope::Scope, singleton_type::SingletonType,
    string_singleton::StringSingleton, table_type::TableType, type_arena::TypeArena,
    union_type::UnionType,
  },
  type_aliases::{
    autocomplete_entry_map::AutocompleteEntryMap, props_type::Props, type_id::TypeId,
  },
};
const K_PARSE_NAME_ERROR: &str = "%error-id%";

/// `autocomplete_props` 的不变上下文（模块/类型环境与索引方式，递归全程共享）。
#[derive(Clone, Copy)]
pub(crate) struct AutocompletePropsCtx<'a> {
  pub module: &'a Module,
  pub type_arena: *mut TypeArena,
  pub builtin_types: &'a BuiltinTypes,
  pub root_ty: TypeId,
  pub index_type: PropIndexType,
  pub nodes: &'a Vec<*mut AstNode>,
}

/// `autocomplete_props` 的单步参数（每个递归分支各不相同的目标与输出）。
pub(crate) struct AutocompletePropsStep<'a> {
  pub ty: TypeId,
  pub result: &'a mut AutocompleteEntryMap,
  pub seen: &'a mut HashSet<TypeId>,
  pub containing_extern_type: Option<*const ExternType>,
}

/// C++ `static void autocompleteProps(...)` (AutocompleteCore.cpp:267-545), the
/// full nine-parameter recursive overload.
pub(crate) fn autocomplete_props(ctx: &AutocompletePropsCtx<'_>, step: AutocompletePropsStep<'_>) {
  // 按原参数顺序解包，保持与 C++ 一一对应。
  let AutocompletePropsCtx {
    module,
    type_arena,
    builtin_types,
    root_ty,
    index_type,
    nodes,
  } = *ctx;
  let AutocompletePropsStep {
    ty,
    result,
    seen,
    containing_extern_type,
  } = step;

  let root_ty = follow_type_id(root_ty);
  let ty = follow_type_id(ty);

  // 递归共享的上下文（root_ty/ty 已按原实现 follow）。
  let rec_ctx = AutocompletePropsCtx {
    module,
    type_arena,
    builtin_types,
    root_ty,
    index_type,
    nodes,
  };

  if seen.contains(&ty) {
    return;
  }
  seen.insert(ty);

  let module_scope = module.get_module_scope();
  let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;
  let builtin_types_ptr = builtin_types as *const BuiltinTypes as *mut BuiltinTypes;

  // `isWrongIndexer` lambda from C++.
  let is_wrong_indexer = |type_id: TypeId| -> bool {
    if index_type == PropIndexType::Key {
      return false;
    }

    let called_with_self = index_type == PropIndexType::Colon;

    // `isCompatibleCall` nested lambda.
    let is_compatible_call = |ftv: &FunctionType| -> bool {
      // Strong match with definition is a success
      if called_with_self == ftv.has_self {
        return true;
      }

      // Calls on extern types require strict match between how function is
      // declared and how it's called
      if get_type_id::<ExternType>(root_ty).is_some() {
        return false;
      }

      // When called with ':', but declared without 'self', it is invalid if
      // a function has incompatible first argument or no arguments at all.
      // When called with '.', but declared with 'self', it is considered
      // invalid if first argument is compatible.
      if let Some(first_arg_ty) = first(ftv.arg_types, true)
        && unsafe {
          check_type_match(
            module,
            root_ty,
            first_arg_ty,
            module_scope_ptr,
            type_arena,
            builtin_types_ptr,
          )
        }
      {
        return called_with_self;
      }

      !called_with_self
    };

    if let Some(ftv) = get_type_id::<FunctionType>(type_id) {
      return !is_compatible_call(ftv);
    }

    // For intersections, any part that is successful makes the whole call successful
    if let Some(itv) = get_type_id::<IntersectionType>(type_id) {
      for &sub_type in &itv.parts {
        if let Some(ftv) = get_type_id::<FunctionType>(follow_type_id(sub_type))
          && is_compatible_call(ftv)
        {
          return false;
        }
      }
    }

    called_with_self
  };

  // `maybeFillSingletonProp` lambda from C++.
  let maybe_fill_singleton_prop = |result: &mut AutocompleteEntryMap, type_id: TypeId| {
    if let Some(singleton_ty) = get_type_id::<SingletonType>(type_id)
      && let Some(string_singleton) = get_singleton_type::<StringSingleton>(singleton_ty)
    {
      let type_correct = if index_type == PropIndexType::Key {
        TypeCorrectKind::Correct
      } else {
        unsafe {
          check_type_correct_kind(
            module,
            type_arena,
            builtin_types,
            *nodes.last().unwrap(),
            Position::default(),
            type_id,
          )
        }
      };

      let parens = if index_type == PropIndexType::Key {
        ParenthesesRecommendation::None
      } else {
        get_paren_recommendation(ty, nodes, type_correct)
      };

      result.insert(
        string_singleton.value.clone(),
        AutocompleteEntry {
          kind: AutocompleteEntryKind::String,
          r#type: Some(type_id),
          deprecated: false,
          wrong_index_type: is_wrong_indexer(type_id),
          type_correct,
          containing_extern_type,
          prop: None,
          documentation_symbol: None,
          tags: Default::default(),
          parens,
          insert_text: None,
          indexed_with_self: index_type == PropIndexType::Colon,
        },
      );
    }
  };

  // `fillProps` lambda from C++.
  let fill_props = |result: &mut AutocompleteEntryMap, props: &Props| {
    for (name, prop) in props {
      // We are walking up the class hierarchy, so if we encounter a property
      // that we have already populated, it takes precedence over the
      // property we found just now.
      if !result.contains_key(name) && name != K_PARSE_NAME_ERROR {
        let type_id: TypeId = if let Some(ty) = prop.read_ty {
          follow_type_id(ty)
        } else {
          continue;
        };

        let type_correct = if index_type == PropIndexType::Key {
          TypeCorrectKind::Correct
        } else {
          unsafe {
            check_type_correct_kind(
              module,
              type_arena,
              builtin_types,
              *nodes.last().unwrap(),
              Position::default(),
              type_id,
            )
          }
        };

        let parens = if index_type == PropIndexType::Key {
          ParenthesesRecommendation::None
        } else {
          get_paren_recommendation(type_id, nodes, type_correct)
        };

        result.insert(
          name.clone(),
          AutocompleteEntry {
            kind: AutocompleteEntryKind::Property,
            r#type: Some(type_id),
            deprecated: prop.deprecated,
            wrong_index_type: is_wrong_indexer(type_id),
            type_correct,
            containing_extern_type,
            prop: Some(prop as *const Property as *const property_type_path::Property),
            documentation_symbol: prop.documentation_symbol.clone(),
            tags: Default::default(),
            parens,
            insert_text: None,
            indexed_with_self: index_type == PropIndexType::Colon,
          },
        );
      }
    }
  };

  // `fillMetatableProps` lambda from C++.
  let fill_metatable_props =
    |result: &mut AutocompleteEntryMap, seen: &mut HashSet<TypeId>, mtable: &TableType| {
      if let Some(index_prop) = mtable.props.get("__index") {
        let followed = match index_prop.read_ty {
          Some(t) => t,
          None => return,
        };
        let followed = follow_type_id(followed);

        if get_type_id::<TableType>(followed).is_some()
          || get_type_id::<MetatableType>(followed).is_some()
        {
          autocomplete_props(
            &rec_ctx,
            AutocompletePropsStep {
              ty: followed,
              result,
              seen,
              containing_extern_type: None,
            },
          );
        } else if let Some(index_function) = get_type_id::<FunctionType>(followed) {
          let index_function_result = first(index_function.ret_types, true);
          if let Some(index_function_result) = index_function_result {
            autocomplete_props(
              &rec_ctx,
              AutocompletePropsStep {
                ty: index_function_result,
                result,
                seen,
                containing_extern_type: None,
              },
            );
          }
        }
      }
    };

  if let Some(cls) = get_type_id::<ExternType>(ty) {
    let containing_extern_type = containing_extern_type.or(Some(cls as *const ExternType));
    fill_props(result, &cls.props);
    if let Some(parent) = cls.parent {
      autocomplete_props(
        &rec_ctx,
        AutocompletePropsStep {
          ty: parent,
          result: &mut *result,
          seen: &mut *seen,
          containing_extern_type,
        },
      );
    }
  } else if let Some(tbl) = get_type_id::<TableType>(ty) {
    fill_props(result, &tbl.props);
    if let Some(indexer) = &tbl.indexer
      && index_type == PropIndexType::Point
    {
      let indexer_ty = follow_type_id(indexer.index_type);
      if let Some(utv) = get_type_id::<UnionType>(indexer_ty) {
        for &option in &utv.options {
          maybe_fill_singleton_prop(result, option);
        }
      } else {
        maybe_fill_singleton_prop(result, indexer_ty);
      }
    }
  } else if let Some(mt) = get_type_id::<MetatableType>(ty) {
    autocomplete_props(
      &rec_ctx,
      AutocompletePropsStep {
        ty: mt.table,
        result: &mut *result,
        seen: &mut *seen,
        containing_extern_type: None,
      },
    );

    if let Some(mtable) = get_type_id::<TableType>(follow_type_id(mt.metatable)) {
      fill_metatable_props(result, seen, mtable);
    }
  } else if let Some(i) = get_type_id::<IntersectionType>(ty) {
    // Complete all properties in every variant
    for &ty in &i.parts {
      let mut inner: AutocompleteEntryMap = Default::default();
      let mut inner_seen: HashSet<TypeId> = seen.clone();

      autocomplete_props(
        &rec_ctx,
        AutocompletePropsStep {
          ty,
          result: &mut inner,
          seen: &mut inner_seen,
          containing_extern_type: None,
        },
      );

      for (k, v) in inner {
        result.entry(k).or_insert(v);
      }
    }
  } else if let Some(u) = get_type_id::<UnionType>(ty) {
    // Complete all properties common to all variants
    let options = &u.options;
    let mut idx = 0usize;

    while idx < options.len() {
      if is_nil(options[idx]) {
        idx += 1;
      } else {
        break;
      }
    }

    if idx == options.len() {
      return;
    }

    autocomplete_props(
      &rec_ctx,
      AutocompletePropsStep {
        ty: options[idx],
        result: &mut *result,
        seen: &mut *seen,
        containing_extern_type: None,
      },
    );

    idx += 1;

    while idx < options.len() {
      let mut inner: AutocompleteEntryMap = Default::default();
      let mut inner_seen: HashSet<TypeId> = Default::default();

      // If we don't do this, and we have the misfortune of receiving a
      // recursive union like:
      //
      //  t1 where t1 = t1 | ExternType
      //
      // Then we are on a one way journey to a stack overflow.
      for &ty in seen.iter() {
        if get_type_id::<UnionType>(ty).is_some() || get_type_id::<IntersectionType>(ty).is_some() {
          inner_seen.insert(ty);
        }
      }

      if is_nil(options[idx]) {
        idx += 1;
        continue;
      }

      autocomplete_props(
        &rec_ctx,
        AutocompletePropsStep {
          ty: options[idx],
          result: &mut inner,
          seen: &mut inner_seen,
          containing_extern_type: None,
        },
      );

      let mut to_remove: BTreeSet<String> = Default::default();

      for k in result.keys() {
        if !inner.contains_key(k) {
          to_remove.insert(k.clone());
        }
      }

      for k in to_remove {
        result.remove(&k);
      }

      idx += 1;
    }
  } else if let Some(pt) = get_type_id::<PrimitiveType>(ty) {
    if let Some(metatable) = pt.metatable
      && let Some(mtable) = get_type_id::<TableType>(metatable)
    {
      fill_metatable_props(result, seen, mtable);
    }
  } else if let Some(singleton) = get_type_id::<SingletonType>(ty)
    && get_singleton_type::<StringSingleton>(singleton).is_some()
  {
    autocomplete_props(
      &rec_ctx,
      AutocompletePropsStep {
        ty: builtin_types.string_type,
        result: &mut *result,
        seen: &mut *seen,
        containing_extern_type: None,
      },
    );
  }
}
