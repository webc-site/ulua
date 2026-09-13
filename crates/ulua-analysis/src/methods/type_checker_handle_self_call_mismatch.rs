use alloc::{sync::Arc, vec::Vec};
use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
    location::Location,
  },
  rtti::ast_node_as,
};

use crate::{
  records::{
    function_does_not_take_self::FunctionDoesNotTakeSelf,
    function_requires_self::FunctionRequiresSelf, module::Module,
    overload_error_entry::OverloadErrorEntry, type_checker::TypeChecker, type_error::TypeError,
    type_pack::TypePack,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_error_data::TypeErrorData},
};
impl TypeChecker {
  pub fn handle_self_call_mismatch(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprCall,
    args: &mut TypePack,
    arg_locations: &[Location],
    errors: &Vec<OverloadErrorEntry>,
  ) -> bool {
    // No overloads succeeded: scan for one that would have worked had the
    // user used `a.b()` rather than `a:b()` or vice versa.
    for e in errors {
      if expr.self_ {
        let edited_arg_locations = if arg_locations.len() > 1 {
          arg_locations[1..].to_vec()
        } else {
          Vec::new()
        };

        let edited_param_list = if args.head.len() > 1 {
          args.head[1..].to_vec()
        } else {
          Vec::new()
        };
        let edited_arg_pack = self.add_type_pack_type_pack(TypePack {
          head: edited_param_list,
          tail: args.tail,
        });

        let mut edited_state = self.mk_unifier(scope, &expr.base.base.location);
        let module_ptr = self
          .current_module
          .as_ref()
          .map(|module| Arc::as_ptr(module) as *mut Module)
          .unwrap_or(null_mut());
        let error_checkpoint = if module_ptr.is_null() {
          0
        } else {
          unsafe { (*module_ptr).errors.len() }
        };

        self.check_argument_list(
          scope,
          unsafe { &*expr.func },
          &mut edited_state,
          edited_arg_pack,
          unsafe { (*e.fn_ty).arg_types },
          &edited_arg_locations,
        );
        if !module_ptr.is_null() {
          unsafe { (*module_ptr).errors.truncate(error_checkpoint) };
        }

        if edited_state.errors.is_empty() {
          edited_state.log.commit();
          self.report_error_type_error(&TypeError::type_error_location_type_error_data(
            expr.base.base.location,
            FunctionDoesNotTakeSelf::default().into(),
          ));
          return true;
        }
      } else if unsafe { (*e.fn_ty).has_self } {
        let index_name =
          unsafe { ast_node_as::<AstExprIndexName>(expr.func as *mut AstNode).as_ref() };
        if let Some(index_name) = index_name {
          let mut edited_arg_locations = Vec::with_capacity(arg_locations.len() + 1);
          edited_arg_locations.push(unsafe { (*index_name.expr).base.location });
          edited_arg_locations.extend(arg_locations.iter().copied());

          let mut edited_arg_list = args.head.clone();
          let module_ptr = self
            .current_module
            .as_ref()
            .map(|module| Arc::as_ptr(module) as *mut Module)
            .unwrap_or(null_mut());
          let error_checkpoint = if module_ptr.is_null() {
            0
          } else {
            unsafe { (*module_ptr).errors.len() }
          };

          let receiver_type = self
            .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
              scope,
              unsafe { &*index_name.expr },
              None,
              false,
            )
            .r#type;
          if !module_ptr.is_null() {
            unsafe { (*module_ptr).errors.truncate(error_checkpoint) };
          }

          edited_arg_list.insert(0, receiver_type);
          let edited_arg_pack = self.add_type_pack_type_pack(TypePack {
            head: edited_arg_list,
            tail: args.tail,
          });

          let mut edited_state = self.mk_unifier(scope, &expr.base.base.location);
          let error_checkpoint = if module_ptr.is_null() {
            0
          } else {
            unsafe { (*module_ptr).errors.len() }
          };

          self.check_argument_list(
            scope,
            unsafe { &*expr.func },
            &mut edited_state,
            edited_arg_pack,
            unsafe { (*e.fn_ty).arg_types },
            &edited_arg_locations,
          );
          if !module_ptr.is_null() {
            unsafe { (*module_ptr).errors.truncate(error_checkpoint) };
          }

          let only_receiver_mismatch = edited_state.errors.len() == 1
            && matches!(edited_state.errors[0].data, TypeErrorData::TypeMismatch(_))
            && edited_state.errors[0].location == unsafe { (*index_name.expr).base.location };

          if edited_state.errors.is_empty() || (only_receiver_mismatch && !args.head.is_empty()) {
            edited_state.log.commit();
            self.report_error_type_error(&TypeError::type_error_location_type_error_data(
              expr.base.base.location,
              FunctionRequiresSelf::default().into(),
            ));
            return true;
          }
        }
      }
    }

    false
  }
}
