use alloc::string::String;
use core::mem::swap;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, ast_expr_call::AstExprCall,
    ast_expr_constant_string::AstExprConstantString, ast_expr_global::AstExprGlobal,
  },
  rtti::ast_node_try_as_ptr,
};

use crate::records::type_guard::TypeGuard;
pub fn match_type_guard(op: i32, left: *mut AstExpr, right: *mut AstExpr) -> Option<TypeGuard> {
  if op != AstExprBinaryOp::CompareEq as i32 && op != AstExprBinaryOp::CompareNe as i32 {
    return None;
  }

  let mut left = left;
  let mut right = right;

  // Safety: try_as_ptr 自带判空（null ⇒ None，从不解引用），left/right 为 binary
  // 比较运算数指针，非空时指向 parse arena 存活节点（Module 活过整个 lint 期，地址
  // 不移动）；命中即 repr(C) 基址重合的真实派生类型只读借用，未命中返回 None。
  // 全程单线程只读，无别名冲突。
  unsafe {
    if ast_node_try_as_ptr::<AstExprCall>(right).is_some() {
      swap(&mut left, &mut right);
    }

    let call = ast_node_try_as_ptr::<AstExprCall>(left)?;
    let string = ast_node_try_as_ptr::<AstExprConstantString>(right)?;
    let callee = ast_node_try_as_ptr::<AstExprGlobal>(call.func)?;

    // AstName::as_bytes 容忍 null（空名 → 空切片，两个分支均不命中）。
    let name_bytes = callee.name.as_bytes();
    let is_typeof = if name_bytes == b"typeof" {
      true
    } else if name_bytes == b"type" {
      false
    } else {
      return None;
    };

    if call.args.len() != 1 {
      return None;
    }

    let type_str = String::from_utf8_lossy(string.value.as_bytes()).into_owned();

    Some(TypeGuard {
      is_typeof,
      target: call.args.as_slice()[0],
      r#type: type_str,
    })
  }
}
