use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr_call::AstExprCall};

pub fn match_table_freeze(call: &AstExprCall) -> bool {
  if call.args.is_empty() {
    return false;
  }

  // Safety: `call.func` 是 parser 写入、arena 存活的 `AstExpr` 节点（或 null）；
  // as_ref 先判空，as_expr_ref 基于 repr(C) 基类 class_index 安全模式匹配具体枚举。全程只读、单线程。
  let Some(func) = (unsafe { call.func.as_ref() }) else {
    return false;
  };

  if let AstExprRef::IndexName(index) = func.as_expr_ref()
    && index.index.as_bytes() == b"freeze"
    && let AstExprRef::Global(global) = index.expr.as_expr_ref()
  {
    return global.name.as_bytes() == b"table";
  }

  false
}
