use alloc::string::String;
use core::mem::swap;

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_call::AstExprCall,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_global::AstExprGlobal,
  },
  rtti::ast_node_try_as,
};

use crate::{
  functions::try_get_l_value::try_get_l_value,
  records::{not_predicate::NotPredicate, type_guard_predicate::TypeGuardPredicate},
  type_aliases::{predicate::Predicate, predicate_vec::PredicateVec},
};
pub fn try_get_type_guard_predicate(expr: &AstExprBinary) -> Option<Predicate> {
  if expr.op != AstExprBinaryOp::CompareEq && expr.op != AstExprBinaryOp::CompareNe {
    return None;
  }

  // left/right 已句柄化恒非空（cpp 亦无判空），is_null 守卫随类型消失；
  // 行走链沿用既有裸指针 API，经 as_ptr 桥接。
  let mut left: *mut AstExpr = expr.left.as_ptr();
  let mut right: *mut AstExpr = expr.right.as_ptr();

  // SAFETY: left 指向 AST arena 节点。
  if ast_node_try_as::<AstExprConstantString>(unsafe { &(*left).base }).is_some() {
    swap(&mut left, &mut right);
  }

  // SAFETY: right 与 left 非 null，指向 AST arena 节点。
  let str_node = ast_node_try_as::<AstExprConstantString>(unsafe { &(*right).base })?;
  // Safety: left 经第 27-29 行判空后无论是否被第 32-34 行 swap 交换，仍是
  // 两个非空 arena 节点之一（parser 保证 AstExprBinary.left/right 非空且
  // AST 存活性覆盖本函数）；&base 仅读取 repr(C) 头部供 RTTI 判别。
  let call = ast_node_try_as::<AstExprCall>(unsafe { &(*left).base })?;
  if call.func.is_null() {
    return None;
  }
  // Safety: call.func 刚判空返回，指向 AST arena 存活表达式节点（parser 对
  // AstExprCall.func 的非空保证覆盖此字段）；&base 是 repr(C) 首字段基址重合
  // 视图，仅用于 ast_node_try_as 的 RTTI 判别。
  let callee = ast_node_try_as::<AstExprGlobal>(unsafe { &(*call.func).base })?;

  // C++ `AstName::operator==(const char*)` strcmps the interned value (false when null).
  // AstName::as_bytes 容忍 null（空名 → 空切片，两个分支均不命中）。
  let callee_name = callee.name.as_bytes();
  if callee_name != b"type" && callee_name != b"typeof" {
    return None;
  }

  let args_slice = call.args.as_slice();
  if args_slice.len() != 1 {
    return None;
  }

  // If ssval is not a valid constant string, we'll find out later when resolving predicate.
  let ssval: String = String::from_utf8_lossy(str_node.value.as_bytes()).into_owned();
  let is_typeof = callee_name == b"typeof";

  let &arg0 = args_slice.first()?;
  if arg0.is_null() {
    return None;
  }
  // SAFETY: arg0 非 null，指向 AST arena 节点。
  let lvalue = try_get_l_value(unsafe { &*arg0 })?;

  let predicate = Predicate::TypeGuard(TypeGuardPredicate {
    lvalue,
    location: expr.base.base.location,
    kind: ssval,
    is_typeof,
  });

  if expr.op == AstExprBinaryOp::CompareNe {
    return Some(Predicate::Not(NotPredicate {
      predicates: PredicateVec::from(alloc::vec![predicate]),
    }));
  }

  Some(predicate)
}
