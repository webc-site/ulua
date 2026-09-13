use alloc::string::String;
use core::{ffi::CStr, mem::swap};

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

  let mut left: *mut AstExpr = expr.left;
  let mut right: *mut AstExpr = expr.right;
  if left.is_null() || right.is_null() {
    return None;
  }

  // SAFETY: left 指向 AST arena 节点。
  if ast_node_try_as::<AstExprConstantString>(unsafe { &(*left).base }).is_some() {
    swap(&mut left, &mut right);
  }

  // SAFETY: right 与 left 非 null，指向 AST arena 节点。
  let str_node = ast_node_try_as::<AstExprConstantString>(unsafe { &(*right).base })?;
  let call = ast_node_try_as::<AstExprCall>(unsafe { &(*left).base })?;
  if call.func.is_null() {
    return None;
  }
  let callee = ast_node_try_as::<AstExprGlobal>(unsafe { &(*call.func).base })?;

  // C++ `AstName::operator==(const char*)` strcmps the interned value (false when null).
  let name_ptr = callee.name.value;
  if name_ptr.is_null() {
    return None;
  }
  // SAFETY: name_ptr 非 null，为 NUL 结尾 C 字符串。
  let callee_name = unsafe { CStr::from_ptr(name_ptr) }.to_bytes();
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
