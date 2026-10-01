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
  records::{
    arena_handle::alias_ref, not_predicate::NotPredicate, type_guard_predicate::TypeGuardPredicate,
  },
  type_aliases::{predicate::Predicate, predicate_vec::PredicateVec},
};
pub fn try_get_type_guard_predicate(expr: &AstExprBinary) -> Option<Predicate> {
  if !expr.op.is_equality() {
    return None;
  }

  // left/right 已句柄化恒非空（cpp 亦无判空），is_null 守卫随类型消失；
  // 行走链沿用既有裸指针 API，经 as_ptr 桥接。
  let mut left: *mut AstExpr = expr.left.as_ptr();
  let mut right: *mut AstExpr = expr.right.as_ptr();

  if ast_node_try_as::<AstExprConstantString>(&alias_ref(left).base).is_some() {
    swap(&mut left, &mut right);
  }

  let str_node = ast_node_try_as::<AstExprConstantString>(&alias_ref(right).base)?;
  let call = ast_node_try_as::<AstExprCall>(&alias_ref(left).base)?;
  if call.func.is_null() {
    return None;
  }
  let callee = ast_node_try_as::<AstExprGlobal>(&alias_ref(call.func).base)?;

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
  let lvalue = try_get_l_value(alias_ref(arg0))?;

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

// r7-tlossy1 让位台账（本文件票面 1 枚：让 1）——:62 `String::from_utf8_lossy(...).into_owned()`
// 源为词法字节流（`str_node.value` 可携非 UTF-8 字节，cpp `std::string ssval` 不
// 校验，Rust String 面 lossy 不可免）；汇为 `TypeGuardPredicate.kind: String`，谓词
// 随 PredicateVec 长存转移，owned 下限恒 1 malloc。callee 名判别已走 `as_bytes()`
// 字节比较零堆（:51 同款前票已收），本枚仅存下限形态。
