use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_varargs::AstExprVarargs,
    temp_vector::TempVector,
  },
  rtti::ast_node_is,
};

/// cpp `isEnoughValues(TempVector<AstExpr*>& values, size_t expected)`
/// （`Ast/src/Parser.cpp:1302`）：末元素是调用/变参时视为「足够」，否则比较数量。
/// 只读遍历，故借用取 `&`（cpp 的非 const 引用只是历史签名）。
pub fn is_enough_values(values: &TempVector<'_, *mut AstExpr>, expected: usize) -> bool {
  if !values.empty() {
    let last = *values.back();
    // SAFETY: 词法/语法已保证 values 元素指向 arena 存活节点（cpp 同款前提）。
    unsafe {
      if ast_node_is::<AstExprCall>(&*last) || ast_node_is::<AstExprVarargs>(&*last) {
        return true;
      }
    }
  }
  values.size() == expected
}
