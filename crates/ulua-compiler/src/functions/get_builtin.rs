use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal,
    ast_local::AstLocal,
    ast_name::AstName,
    ast_node::AstNode,
  },
  rtti,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::global::Global,
  functions::get_global_state::get_global_state,
  records::{builtin::Builtin, variable::Variable},
};

pub fn get_builtin(
  node: *mut AstExpr,
  globals: &DenseHashMap<AstName, Global>,
  variables: &DenseHashMap<*mut AstLocal, Variable>,
) -> Builtin {
  let node_ptr = node;
  if node_ptr.is_null() {
    return Builtin::default();
  }

  let expr_local = unsafe { rtti::ast_node_as::<AstExprLocal>(node_ptr as *mut AstNode) };
  if !expr_local.is_null() {
    let expr = unsafe { &*expr_local };
    if let Some(v) = variables.find(&expr.local)
      && !v.written
      && !v.init.is_null()
    {
      return get_builtin(v.init, globals, variables);
    }
    return Builtin::default();
  }

  let expr_index = unsafe { rtti::ast_node_as::<AstExprIndexName>(node_ptr as *mut AstNode) };
  if !expr_index.is_null() {
    let expr = unsafe { &*expr_index };
    let object_local = unsafe { rtti::ast_node_as::<AstExprLocal>(expr.expr as *mut AstNode) };
    if !object_local.is_null() {
      let object = unsafe { &*object_local };
      if let Some(v) = variables.find(&object.local)
        && !v.written
        && !v.init.is_null()
      {
        let mut target_global: *mut AstExprGlobal = null_mut();

        let global_expr = unsafe { rtti::ast_node_as::<AstExprGlobal>(v.init as *mut AstNode) };
        if !global_expr.is_null() {
          target_global = global_expr;
        } else {
          let binary_expr = unsafe { rtti::ast_node_as::<AstExprBinary>(v.init as *mut AstNode) };
          if !binary_expr.is_null() {
            let cond = unsafe { &*binary_expr };
            if cond.op == AstExprBinaryOp::Or {
              target_global =
                unsafe { rtti::ast_node_as::<AstExprGlobal>(cond.left as *mut AstNode) };
            }
          }
        }

        if !target_global.is_null() {
          let global = unsafe { &*target_global };
          if get_global_state(globals, global.name) == Global::Default {
            return Builtin {
              object: global.name,
              method: expr.index,
            };
          }
        }
      }
    }

    let object_global = unsafe { rtti::ast_node_as::<AstExprGlobal>(expr.expr as *mut AstNode) };
    if !object_global.is_null() {
      let object = unsafe { &*object_global };
      if get_global_state(globals, object.name) == Global::Default {
        return Builtin {
          object: object.name,
          method: expr.index,
        };
      }
    }

    return Builtin::default();
  }

  let expr_global = unsafe { rtti::ast_node_as::<AstExprGlobal>(node_ptr as *mut AstNode) };
  if !expr_global.is_null() {
    let expr = unsafe { &*expr_global };
    if get_global_state(globals, expr.name) == Global::Default {
      return Builtin {
        object: AstName::new(),
        method: expr.name,
      };
    }
  }

  Builtin::default()
}
