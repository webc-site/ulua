use alloc::{sync::Arc, vec, vec::Vec};
use core::ptr::null_mut;

use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_local::AstExprLocal, ast_node::AstNode},
  rtti::ast_node_as,
};

use crate::{
  functions::{
    finite::finite, follow_type::follow_type_id, get_type_alt_j::get_type_id,
    is_table_intersection::is_table_intersection, is_table_union::is_table_union,
    size_type_pack::size,
  },
  records::{
    any_type::AnyType,
    binding::Binding,
    cannot_extend_table::{CannotExtendTable, CannotExtendTable_Context},
    generic_error::GenericError,
    metatable_type::MetatableType,
    module::Module,
    scope::Scope,
    symbol::Symbol,
    table_type::TableType,
    type_checker::TypeChecker,
    type_error::TypeError,
    type_pack::TypePack,
    union_type::UnionType,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    error_type::ErrorType, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};
pub fn magic_set_metatable_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let param_pack = with_predicate.r#type;

  if unsafe { size(param_pack, null_mut()) } < 2 && unsafe { finite(param_pack, null_mut()) } {
    return None;
  }

  let module = typechecker.current_module.as_ref()?.clone();
  let arena = unsafe { &mut (*(Arc::as_ptr(&module) as *mut Module)).internal_types };

  let expected_args = typechecker.un_type_pack(scope, param_pack, 2, &expr.base.base.location);
  let target = follow_type_id(expected_args[0]);
  let mt = follow_type_id(expected_args[1]);

  typechecker.tablify(target);
  typechecker.tablify(mt);

  if let Some(tab_ref) = get_type_id::<TableType>(target) {
    if unsafe { (*target).persistent } {
      typechecker.report_error_type_error(&TypeError::type_error_location_type_error_data(
        expr.base.base.location,
        TypeErrorData::CannotExtendTable(CannotExtendTable {
          table_type: target,
          context: CannotExtendTable_Context::Metatable,
          prop: String::new(),
        }),
      ));
    } else {
      let mt_ttv = get_type_id::<TableType>(mt);
      let mut mtv = MetatableType {
        table: target,
        metatable: mt,
        synthetic_name: None,
      };

      if (tab_ref.name.is_some() || tab_ref.synthetic_name.is_some())
        && mt_ttv.is_some_and(|mt_ttv| mt_ttv.name.is_some() || mt_ttv.synthetic_name.is_some())
      {
        let table_name = tab_ref
          .name
          .as_ref()
          .or(tab_ref.synthetic_name.as_ref())
          .unwrap();
        let metatable_name = mt_ttv
          .and_then(|mt_ttv| mt_ttv.name.as_ref().or(mt_ttv.synthetic_name.as_ref()))
          .unwrap();

        if table_name == metatable_name {
          mtv.synthetic_name = Some(table_name.clone());
        }
      }

      let mt_ty = arena.add_type(mtv);

      if expr.args.size < 1 {
        return None;
      }

      if !expr.self_ {
        let target_expr = unsafe { *expr.args.data.add(0) };
        let target_local = unsafe { ast_node_as::<AstExprLocal>(target_expr as *mut AstNode) };
        if !target_local.is_null() {
          let scope_ptr = Arc::as_ptr(scope) as *mut Scope;
          unsafe {
            (*scope_ptr).bindings.insert(
              Symbol::from_local((*target_local).local),
              Binding {
                type_id: mt_ty,
                location: expr.base.base.location,
                deprecated: false,
                deprecated_suggestion: String::new(),
                documentation_symbol: None,
              },
            );
          }
        }
      }

      return Some(WithPredicate::with_predicate_t(arena.add_type_pack_t(
        TypePack {
          head: vec![mt_ty],
          tail: None,
        },
      )));
    }
  } else if get_type_id::<AnyType>(target).is_some()
    || get_type_id::<ErrorType>(target).is_some()
    || is_table_intersection(target)
  {
  } else if is_table_union(target) {
    // is_table_union 已判定为 UnionType，get 必命中；None（不可达）落回尾部兜底。
    if let Some(ut_ref) = get_type_id::<UnionType>(target) {
      let mut result_parts: Vec<TypeId> = Vec::new();
      for &ty in &ut_ref.options {
        result_parts.push(arena.add_type(MetatableType {
          table: ty,
          metatable: mt,
          synthetic_name: None,
        }));
      }

      let result_union = arena.add_type(UnionType {
        options: result_parts,
      });
      return Some(WithPredicate::with_predicate_t(arena.add_type_pack_t(
        TypePack {
          head: vec![result_union],
          tail: None,
        },
      )));
    }
  } else {
    typechecker.report_error_type_error(&TypeError::type_error_location_type_error_data(
      expr.base.base.location,
      TypeErrorData::GenericError(GenericError::new(
        "setmetatable should take a table".to_string(),
      )),
    ));
  }

  Some(WithPredicate::with_predicate_t(arena.add_type_pack_t(
    TypePack {
      head: vec![target],
      tail: None,
    },
  )))
}
