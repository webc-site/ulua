use alloc::string::String;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp},
};

use crate::records::{arena_handle::alias_opt, type_guard::TypeGuard};

pub(crate) fn match_type_guard(
  op: AstExprBinaryOp,
  left: *mut AstExpr,
  right: *mut AstExpr,
) -> Option<TypeGuard> {
  if !op.is_equality() {
    return None;
  }

  let left_ref = alias_opt(left)?.as_expr_ref();
  let right_ref = alias_opt(right)?.as_expr_ref();

  let (call, string) = match (left_ref, right_ref) {
    (AstExprRef::Call(call), AstExprRef::ConstantString(string)) => (call, string),
    (AstExprRef::ConstantString(string), AstExprRef::Call(call)) => (call, string),
    _ => return None,
  };

  if call.args.len() != 1 {
    return None;
  }

  let callee = alias_opt(call.func)?;
  let is_typeof = match callee.as_expr_ref() {
    AstExprRef::Global(global) => match global.name.as_bytes() {
      b"typeof" => true,
      b"type" => false,
      _ => return None,
    },
    _ => return None,
  };

  let type_str = String::from_utf8_lossy(string.value.as_bytes()).into_owned();

  Some(TypeGuard {
    is_typeof,
    target: *call.args.first()?,
    r#type: type_str,
  })
}

// r7-tlossy1 让位台账（本文件票面 1 枚：让 1）——:45 `String::from_utf8_lossy(...).into_owned()`
// 源为词法字节流（`string.value` 可携非 UTF-8，lossy 系 Rust String 表示面）；汇为
// `TypeGuard.r#type: String` 字段，guard 随消费面（cfg_builder/constraint_generator
// 的 `typeguard.r#type()` 读取）跨函数转移长存，owned 下限恒 1 malloc。免分配需
// Cow<'arena> 贯通 struct 字段+访问器，外溢候裁。callee 名比较已走 `as_bytes()`
// 字节零堆形（:37），不在此枚账内。
