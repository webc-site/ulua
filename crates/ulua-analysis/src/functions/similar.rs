use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup, ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate, ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal, ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion, ast_expr_unary::AstExprUnary,
    ast_expr_varargs::AstExprVarargs,
  },
  rtti::AstNodeClass,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

/// # Safety
/// 调用方须保证 `lhs、`rhs` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn similar(lhs: *mut AstExpr, rhs: *mut AstExpr) -> bool {
  if lhs.is_null() || rhs.is_null() {
    return false;
  }

  // SAFETY: lhs 与 rhs 非 null，指向 AST arena 节点。
  let lhs_ref = unsafe { &*lhs };
  let rhs_ref = unsafe { &*rhs };

  if lhs_ref.base.class_index != rhs_ref.base.class_index {
    return false;
  }

  // SAFETY: lhs/rhs 均非 null 且 class_index 一致，各臂 cast 与递归均安全。
  unsafe {
    match lhs_ref.base.class_index {
      <AstExprGroup as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprGroup>();
        let re = &*rhs.cast::<AstExprGroup>();
        similar(le.expr, re.expr)
      }
      <AstExprConstantNil as AstNodeClass>::CLASS_INDEX => true,
      <AstExprConstantBool as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprConstantBool>();
        let re = &*rhs.cast::<AstExprConstantBool>();
        le.value == re.value
      }
      <AstExprConstantNumber as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprConstantNumber>();
        let re = &*rhs.cast::<AstExprConstantNumber>();
        le.value == re.value
      }
      <AstExprConstantInteger as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprConstantInteger>();
        let re = &*rhs.cast::<AstExprConstantInteger>();
        le.value == re.value
      }
      <AstExprConstantString as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprConstantString>();
        let re = &*rhs.cast::<AstExprConstantString>();
        le.value.as_bytes() == re.value.as_bytes()
      }
      <AstExprLocal as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprLocal>();
        let re = &*rhs.cast::<AstExprLocal>();
        le.local == re.local
      }
      <AstExprGlobal as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprGlobal>();
        let re = &*rhs.cast::<AstExprGlobal>();
        le.name.value == re.name.value
      }
      <AstExprVarargs as AstNodeClass>::CLASS_INDEX => true,
      <AstExprIndexName as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprIndexName>();
        let re = &*rhs.cast::<AstExprIndexName>();
        le.index.value == re.index.value && similar(le.expr, re.expr)
      }
      <AstExprIndexExpr as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprIndexExpr>();
        let re = &*rhs.cast::<AstExprIndexExpr>();
        similar(le.expr, re.expr) && similar(le.index, re.index)
      }
      <AstExprFunction as AstNodeClass>::CLASS_INDEX => false,
      <AstExprUnary as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprUnary>();
        let re = &*rhs.cast::<AstExprUnary>();
        le.op == re.op && similar(le.expr, re.expr)
      }
      <AstExprBinary as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprBinary>();
        let re = &*rhs.cast::<AstExprBinary>();
        le.op == re.op && similar(le.left, re.left) && similar(le.right, re.right)
      }
      <AstExprTypeAssertion as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprTypeAssertion>();
        let re = &*rhs.cast::<AstExprTypeAssertion>();
        le.expr == re.expr
      }
      <AstExprError as AstNodeClass>::CLASS_INDEX => false,
      <AstExprCall as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprCall>();
        let re = &*rhs.cast::<AstExprCall>();
        if le.args.size != re.args.size || le.self_ != re.self_ {
          return false;
        }
        if !similar(le.func, re.func) {
          return false;
        }
        for (&la, &ra) in le.args.as_slice().iter().zip(re.args.as_slice()) {
          if !similar(la, ra) {
            return false;
          }
        }
        true
      }
      <AstExprTable as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprTable>();
        let re = &*rhs.cast::<AstExprTable>();
        if le.items.size != re.items.size {
          return false;
        }
        for (li, ri) in le.items.as_slice().iter().zip(re.items.as_slice()) {
          if li.kind != ri.kind {
            return false;
          }
          if li.key.is_null() != ri.key.is_null() {
            return false;
          }
          if !li.key.is_null() && !similar(li.key, ri.key) {
            return false;
          }
          if !similar(li.value, ri.value) {
            return false;
          }
        }
        true
      }
      <AstExprIfElse as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprIfElse>();
        let re = &*rhs.cast::<AstExprIfElse>();
        similar(le.condition, re.condition)
          && similar(le.true_expr, re.true_expr)
          && similar(le.false_expr, re.false_expr)
      }
      <AstExprInterpString as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprInterpString>();
        let re = &*rhs.cast::<AstExprInterpString>();
        if le.strings.size != re.strings.size || le.expressions.size != re.expressions.size {
          return false;
        }
        for (ls, rs) in le.strings.as_slice().iter().zip(re.strings.as_slice()) {
          if ls.as_bytes() != rs.as_bytes() {
            return false;
          }
        }
        for (&le_expr, &re_expr) in le
          .expressions
          .as_slice()
          .iter()
          .zip(re.expressions.as_slice())
        {
          if !similar(le_expr, re_expr) {
            return false;
          }
        }
        true
      }
      <AstExprInstantiate as AstNodeClass>::CLASS_INDEX => {
        let le = &*lhs.cast::<AstExprInstantiate>();
        let re = &*rhs.cast::<AstExprInstantiate>();
        similar(le.expr, re.expr)
      }
      _ => {
        LUAU_ASSERT!(false);
        false
      }
    }
  }
}
