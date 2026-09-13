use alloc::{string::String, sync::Arc};
use core::ffi::CStr;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
    ast_expr_global::AstExprGlobal, ast_expr_group::AstExprGroup,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal,
  },
  rtti::ast_node_try_as,
};

use crate::{
  records::{field::Field, symbol::Symbol},
  type_aliases::l_value::LValue,
};
pub fn try_get_l_value(node: &AstExpr) -> Option<LValue> {
  let mut expr = node;

  while let Some(group) = ast_node_try_as::<AstExprGroup>(&expr.base) {
    if group.expr.is_null() {
      return None;
    }
    // SAFETY: group.expr 非 null，指向 AST arena 节点。
    expr = unsafe { &*group.expr };
  }

  if let Some(local) = ast_node_try_as::<AstExprLocal>(&expr.base) {
    return Some(LValue::Symbol(Symbol::symbol_ast_local(local.local)));
  }

  if let Some(global) = ast_node_try_as::<AstExprGlobal>(&expr.base) {
    return Some(LValue::Symbol(Symbol::symbol_ast_name(global.name)));
  }

  if let Some(indexname) = ast_node_try_as::<AstExprIndexName>(&expr.base) {
    if indexname.expr.is_null() {
      return None;
    }
    // SAFETY: indexname.expr 非 null，指向 AST arena 节点。
    let lvalue = try_get_l_value(unsafe { &*indexname.expr })?;
    let key = if indexname.index.value.is_null() {
      String::new()
    } else {
      // SAFETY: index.value 非 null，为 NUL 结尾 C 字符串。
      unsafe { CStr::from_ptr(indexname.index.value) }
        .to_string_lossy()
        .into_owned()
    };
    return Some(LValue::Field(Field {
      parent: Some(Arc::new(lvalue)),
      key,
    }));
  }

  if let Some(indexexpr) = ast_node_try_as::<AstExprIndexExpr>(&expr.base) {
    if indexexpr.expr.is_null() || indexexpr.index.is_null() {
      return None;
    }
    // SAFETY: indexexpr.expr 非 null，指向 AST arena 节点。
    let lvalue = try_get_l_value(unsafe { &*indexexpr.expr })?;
    // SAFETY: indexexpr.index 非 null，指向 AST arena 节点。
    let string_node =
      ast_node_try_as::<AstExprConstantString>(unsafe { &(*indexexpr.index).base })?;
    let key = String::from_utf8_lossy(string_node.value.as_bytes()).into_owned();
    return Some(LValue::Field(Field {
      parent: Some(Arc::new(lvalue)),
      key,
    }));
  }

  None
}
