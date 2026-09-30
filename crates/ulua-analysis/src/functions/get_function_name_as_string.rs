use alloc::string::{String, ToString};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_group::AstExprGroup,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal,
  },
  rtti::ast_node_try_as_ptr,
};
pub fn get_function_name_as_string(expr: &AstExpr) -> Option<String> {
  let mut curr = expr as *const AstExpr;
  let mut s = String::new();

  // Safety: `curr` 初值取自存活的 `&AstExpr`（非空、对齐），循环沿 AST 子节点指针下行；
  // `ast_node_try_as_ptr` 按 RTTI class-index 分派，null/未命中返回 None 且从不解引用，
  // 命中即按 repr(C) 基址重合借出类型正确的只读引用。下行取的 `.expr` 子指针由 parser
  // 保证非空，且 AST 节点存活于 `expr` 借用覆盖的 arena（bump 分配、地址不移动）。
  // 全程只读、单线程遍历。
  unsafe {
    loop {
      if let Some(local) = ast_node_try_as_ptr::<AstExprLocal>(curr.cast_mut()) {
        let mut name = local.local.name.as_str_or_empty().to_string();
        name.push_str(&s);
        return Some(name);
      }

      if let Some(global) = ast_node_try_as_ptr::<AstExprGlobal>(curr.cast_mut()) {
        let mut name = global.name.as_str_or_empty().to_string();
        name.push_str(&s);
        return Some(name);
      }

      if let Some(indexname) = ast_node_try_as_ptr::<AstExprIndexName>(curr.cast_mut()) {
        // expr 已句柄化恒非空；行走链为既有裸指针 API，经 as_ptr 桥接。
        curr = indexname.expr.as_ptr();

        let index_str = indexname.index.as_str_or_empty().to_string();

        let mut new_s = String::new();
        new_s.push('.');
        new_s.push_str(&index_str);
        new_s.push_str(&s);
        s = new_s;

        continue;
      }

      if let Some(group) = ast_node_try_as_ptr::<AstExprGroup>(curr.cast_mut()) {
        // expr 已句柄化恒非空；curr 为既有裸指针行走链，经 as_ptr 桥接。
        curr = group.expr.as_ptr();
        continue;
      }

      return None;
    }
  }
}
