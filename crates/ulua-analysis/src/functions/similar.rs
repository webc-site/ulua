use core::ptr::eq;

use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr::AstExpr};

/// 结构等价性比较（只读引用安全门面）。
/// 对应 C++ `static bool similar(AstExpr* lhs, AstExpr* rhs)` (`cpp/Analysis/src/Linter.cpp:110`)。
pub fn similar_ref(lhs: &AstExpr, rhs: &AstExpr) -> bool {
  match (lhs.as_expr_ref(), rhs.as_expr_ref()) {
    (AstExprRef::Group(l), AstExprRef::Group(r)) => similar_ref(&l.expr, &r.expr),
    (AstExprRef::ConstantNil(_), AstExprRef::ConstantNil(_)) => true,
    (AstExprRef::ConstantBool(l), AstExprRef::ConstantBool(r)) => l.value == r.value,
    (AstExprRef::ConstantNumber(l), AstExprRef::ConstantNumber(r)) => l.value == r.value,
    (AstExprRef::ConstantInteger(l), AstExprRef::ConstantInteger(r)) => l.value == r.value,
    (AstExprRef::ConstantString(l), AstExprRef::ConstantString(r)) => {
      l.value.as_bytes() == r.value.as_bytes()
    }
    (AstExprRef::Local(l), AstExprRef::Local(r)) => eq(l.local.as_ptr(), r.local.as_ptr()),
    (AstExprRef::Global(l), AstExprRef::Global(r)) => l.name.as_bytes() == r.name.as_bytes(),
    (AstExprRef::Varargs(_), AstExprRef::Varargs(_)) => true,
    (AstExprRef::IndexName(l), AstExprRef::IndexName(r)) => {
      l.index.as_bytes() == r.index.as_bytes() && l.op == r.op && similar_ref(&l.expr, &r.expr)
    }
    (AstExprRef::IndexExpr(l), AstExprRef::IndexExpr(r)) => {
      similar_ref(&l.expr, &r.expr) && similar_ref(&l.index, &r.index)
    }
    (AstExprRef::Function(_), AstExprRef::Function(_)) => false,
    (AstExprRef::Unary(l), AstExprRef::Unary(r)) => l.op == r.op && similar_ref(&l.expr, &r.expr),
    (AstExprRef::Binary(l), AstExprRef::Binary(r)) => {
      l.op == r.op && similar_ref(&l.left, &r.left) && similar_ref(&l.right, &r.right)
    }
    (AstExprRef::TypeAssertion(l), AstExprRef::TypeAssertion(r)) => l.expr == r.expr,
    (AstExprRef::Error(_), AstExprRef::Error(_)) => false,
    (AstExprRef::Call(l), AstExprRef::Call(r)) => {
      l.self_ == r.self_
        && unsafe { similar(l.func, r.func) }
        && l.args.len() == r.args.len()
        && l
          .args
          .iter()
          .zip(r.args.iter())
          .all(|(&a, &b)| unsafe { similar(a, b) })
    }
    (AstExprRef::Table(l), AstExprRef::Table(r)) => {
      if l.items.len() != r.items.len() {
        return false;
      }
      for (li, ri) in l.items.iter().zip(r.items.iter()) {
        if li.kind != ri.kind {
          return false;
        }
        if li.key.is_null() != ri.key.is_null() {
          return false;
        }
        if !li.key.is_null() && !unsafe { similar(li.key, ri.key) } {
          return false;
        }
        if !unsafe { similar(li.value, ri.value) } {
          return false;
        }
      }
      true
    }
    (AstExprRef::IfElse(l), AstExprRef::IfElse(r)) => {
      similar_ref(&l.condition, &r.condition)
        && similar_ref(&l.true_expr, &r.true_expr)
        && similar_ref(&l.false_expr, &r.false_expr)
    }
    (AstExprRef::InterpString(l), AstExprRef::InterpString(r)) => {
      if l.strings.len() != r.strings.len() || l.expressions.len() != r.expressions.len() {
        return false;
      }
      for (ls, rs) in l.strings.iter().zip(r.strings.iter()) {
        if ls.as_bytes() != rs.as_bytes() {
          return false;
        }
      }
      for (&le_expr, &re_expr) in l.expressions.iter().zip(r.expressions.iter()) {
        if !unsafe { similar(le_expr, re_expr) } {
          return false;
        }
      }
      true
    }
    (AstExprRef::Instantiate(l), AstExprRef::Instantiate(r)) => unsafe { similar(l.expr, r.expr) },
    _ => false,
  }
}

/// # Safety
/// `lhs`、`rhs` 须各指向本次 lint 比较期间存活的 parse-arena `AstExpr`（非空、对齐、地址在 arena
/// 释放前不移动）；两指针互不重叠，全程只读，单线程。对应 C++ `static bool similar(AstExpr* lhs, AstExpr* rhs)`
/// (`cpp/Analysis/src/Linter.cpp:110`)。
pub unsafe fn similar(lhs: *mut AstExpr, rhs: *mut AstExpr) -> bool {
  match (unsafe { lhs.as_ref() }, unsafe { rhs.as_ref() }) {
    (Some(l), Some(r)) => similar_ref(l, r),
    _ => false,
  }
}
