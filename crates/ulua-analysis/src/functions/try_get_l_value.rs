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
  records::{arena_handle::alias_ref, field::Field, symbol::Symbol},
  type_aliases::l_value::LValue,
};
pub fn try_get_l_value(node: &AstExpr) -> Option<LValue> {
  let mut expr = node;

  while let Some(group) = ast_node_try_as::<AstExprGroup>(&expr.base) {
    // expr 已句柄化恒非空（cpp `expr = group->expr` 亦无判空），is_null 守卫随类型消失。
    expr = alias_ref(group.expr.as_ptr());
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

// r7-tlossy1 让位台账（本文件票面 1 枚：让 1）——:52 `String::from_utf8_lossy(...).into_owned()`
// 源为词法字节流（IndexExpr 常量串下标可携非 UTF-8，cpp `LValue::Field.key` 即
// std::string 原字节，Rust String 面 lossy 系表示形式）；汇为 `Field.key: String`，
// LValue 经 `Arc` 长存并跨 resolve 面转移，owned 下限恒 1 malloc。同函数 :41
// IndexName 臂 `as_str_or_empty().to_string()` 同为下限形（AstName 侧无免分配口）。
