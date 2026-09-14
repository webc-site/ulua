use alloc::{string::String, sync::Arc, vec::Vec};
use core::ffi::CStr;
use std::collections::HashMap;

use ulua_ast::{
  records::{
    ast_expr_index_name::AstExprIndexName, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_block::AstStatBlock, ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_function::AstStatFunction, ast_stat_local_function::AstStatLocalFunction,
    ast_stat_type_alias::AstStatTypeAlias,
  },
  rtti::ast_node_as,
};

use crate::{
  enums::control_flow::ControlFlow,
  functions::{
    contains_function_call_or_return::contains_function_call_or_return,
    follow_type::follow_type_id, toposort::toposort,
  },
  records::{binding::Binding, scope::Scope, symbol::Symbol, type_checker::TypeChecker},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl TypeChecker {
  pub fn check_block_without_recursion_check(
    &mut self,
    scope: &ScopePtr,
    block: &AstStatBlock,
  ) -> ControlFlow {
    let mut sub_level: i32 = 0;

    let mut sorted: Vec<*mut AstStat> = (0..block.body.size)
      .map(|i| unsafe { *block.body.data.add(i) })
      .collect();
    toposort(&mut sorted);

    for stat in sorted.iter() {
      let stat_ptr = *stat;
      let typealias = unsafe { ast_node_as::<AstStatTypeAlias>(stat_ptr as *mut AstNode) };
      if !typealias.is_null() {
        self.prototype_scope_ptr_ast_stat_type_alias_i32(
          scope.clone(),
          unsafe { &*typealias },
          sub_level,
        );
        sub_level += 1;
      } else {
        let declared_extern_type =
          unsafe { ast_node_as::<AstStatDeclareExternType>(stat_ptr as *mut AstNode) };
        if !declared_extern_type.is_null() {
          self.prototype_scope_ptr_ast_stat_declare_extern_type(scope.clone(), unsafe {
            &*declared_extern_type
          });
        }
      }
    }

    let mut proto_iter: usize = 0;
    let mut check_iter: usize = 0;

    let mut function_decls: HashMap<*mut AstStat, (TypeId, ScopePtr)> = HashMap::new();

    let mut first_flow: Option<ControlFlow> = None;
    while proto_iter < sorted.len() {
      let proto_stat = sorted[proto_iter];

      if contains_function_call_or_return(unsafe { &*proto_stat }) {
        while check_iter != proto_iter {
          self.check_body(scope, sorted[check_iter], &function_decls);
          check_iter += 1;
        }

        // We do check the current element, so advance checkIter beyond it.
        check_iter += 1;
        let flow = self.check_scope_ptr_ast_stat(scope, unsafe { &*proto_stat });
        if flow != ControlFlow::None && first_flow.is_none() {
          first_flow = Some(flow);
        }
      } else if !unsafe { ast_node_as::<AstStatFunction>(proto_stat as *mut AstNode) }.is_null() {
        let fun = unsafe { ast_node_as::<AstStatFunction>(proto_stat as *mut AstNode) };
        let self_type: Option<TypeId> = None; // TODO clip
        let mut expected_type: Option<TypeId> = None;

        if unsafe { (*(*fun).func).self_ }.is_null() {
          let name = unsafe { ast_node_as::<AstExprIndexName>((*fun).name as *mut AstNode) };
          if !name.is_null() {
            let expr_ty = self
              .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*(*name).expr },
                None,
                false,
              )
              .r#type;
            let index_name = unsafe {
              CStr::from_ptr((*name).index.value)
                .to_string_lossy()
                .into_owned()
            };
            expected_type = self.get_index_type_from_type(
              scope.clone(),
              expr_ty,
              &index_name,
              unsafe { &(*name).index_location },
              false,
            );
          }
        }

        let pair = self.check_function_signature(
          scope,
          sub_level,
          unsafe { &*(*fun).func },
          Some(unsafe { (*(*fun).name).base.location }),
          self_type,
          expected_type,
        );
        let (fun_ty, fun_scope) = (pair.0, pair.1.clone());

        function_decls.insert(proto_stat, pair);
        sub_level += 1;

        let left_type = unsafe {
          follow_type_id(self.check_function_name(scope, &*(*fun).name, fun_scope.level))
        };

        self.unify_type_id_type_id_scope_ptr_location(fun_ty, left_type, scope, unsafe {
          &(*fun).base.base.location
        });
      } else if !unsafe { ast_node_as::<AstStatLocalFunction>(proto_stat as *mut AstNode) }
        .is_null()
      {
        let fun = unsafe { ast_node_as::<AstStatLocalFunction>(proto_stat as *mut AstNode) };
        let pair = self.check_function_signature(
          scope,
          sub_level,
          unsafe { &*(*fun).func },
          Some(unsafe { (*(*fun).name).location }),
          None,
          None,
        );
        let fun_ty = pair.0;

        function_decls.insert(proto_stat, pair);
        sub_level += 1;

        unsafe {
          let scope_mut = Arc::as_ptr(scope) as *mut Scope;
          (*scope_mut).bindings.insert(
            Symbol::from_local((*fun).name),
            Binding {
              type_id: fun_ty,
              location: (*(*fun).name).location,
              deprecated: false,
              deprecated_suggestion: String::new(),
              documentation_symbol: None,
            },
          );
        }
      } else {
        let flow = self.check_scope_ptr_ast_stat(scope, unsafe { &*proto_stat });
        if flow != ControlFlow::None && first_flow.is_none() {
          first_flow = Some(flow);
        }
      }

      proto_iter += 1;
    }

    while check_iter != sorted.len() {
      self.check_body(scope, sorted[check_iter], &function_decls);
      check_iter += 1;
    }

    self.check_block_type_aliases(scope, &mut sorted);

    first_flow.unwrap_or(ControlFlow::None)
  }

  /// C++ `checkBody` lambda inside `checkBlockWithoutRecursionCheck`.
  fn check_body(
    &mut self,
    scope: &ScopePtr,
    stat: *mut AstStat,
    function_decls: &HashMap<*mut AstStat, (TypeId, ScopePtr)>,
  ) {
    let fun = unsafe { ast_node_as::<AstStatFunction>(stat as *mut AstNode) };
    if !fun.is_null() {
      let (fun_ty, fun_scope) = function_decls
        .get(&stat)
        .map(|(t, s)| (*t, s.clone()))
        .expect("functionDecls.count(stat)");
      self.check_scope_ptr_type_id_scope_ptr_ast_stat_function(scope, fun_ty, &fun_scope, unsafe {
        &*fun
      });
      return;
    }

    let fun_local = unsafe { ast_node_as::<AstStatLocalFunction>(stat as *mut AstNode) };
    if !fun_local.is_null() {
      let (fun_ty, fun_scope) = function_decls
        .get(&stat)
        .map(|(t, s)| (*t, s.clone()))
        .expect("functionDecls.count(stat)");
      self.check_scope_ptr_type_id_scope_ptr_ast_stat_local_function(
        scope,
        fun_ty,
        &fun_scope,
        unsafe { &*fun_local },
      );
    }
  }
}
