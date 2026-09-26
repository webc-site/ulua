use alloc::{string::String, sync::Arc};

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
    // expr 已句柄化恒非空（cpp `expr = group->expr` 亦无判空），is_null 守卫随类型消失。
    // Safety: as_ptr 为既有 arena 裸指针桥；节点由 parse arena 存活契约保证，
    // 循环重绑定不能沿用 group 借用半径，引用外扩与 cpp 直传 `group->expr` 同款。
    expr = unsafe { &*group.expr.as_ptr() };
  }

  if let Some(local) = ast_node_try_as::<AstExprLocal>(&expr.base) {
    // local 槽已句柄化恒非空；Symbol::from_local 为既有裸指针 API，经 as_ptr 桥接。
    return Some(LValue::Symbol(Symbol::from_local(local.local.as_ptr())));
  }

  if let Some(global) = ast_node_try_as::<AstExprGlobal>(&expr.base) {
    return Some(LValue::Symbol(Symbol::from_global(global.name)));
  }

  if let Some(indexname) = ast_node_try_as::<AstExprIndexName>(&expr.base) {
    // expr 已句柄化恒非空（cpp 亦无判空），is_null 死守卫随类型消失；
    // get() 只读借用出自存活 &AstExprIndexExpr（AST arena 节点）。
    let lvalue = try_get_l_value(indexname.expr.get())?;
    let key = indexname.index.as_str_or_empty().to_string();
    return Some(LValue::Field(Field {
      parent: Some(Arc::new(lvalue)),
      key,
    }));
  }

  if let Some(indexexpr) = ast_node_try_as::<AstExprIndexExpr>(&expr.base) {
    // expr/index 已句柄化恒非空（cpp 亦无判空），is_null 守卫随类型消失；
    // get() 只读借用出自存活 &AstExprIndexExpr（AST arena 节点）。
    let lvalue = try_get_l_value(indexexpr.expr.get())?;
    let string_node = ast_node_try_as::<AstExprConstantString>(&indexexpr.index.get().base)?;
    let key = String::from_utf8_lossy(string_node.value.as_bytes()).into_owned();
    return Some(LValue::Field(Field {
      parent: Some(Arc::new(lvalue)),
      key,
    }));
  }

  None
}
