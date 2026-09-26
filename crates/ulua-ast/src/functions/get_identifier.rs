use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal,
    ast_name::AstName,
  },
  rtti::ast_node_try_as,
};

/// 节点的标识符名：Global 取其名，Local 取绑定 local 的名；其余（含 `None`）为空名。
///
/// 入参 `Option<&AstExpr>` 即 cpp 可空 `AstExpr*` 的借用形态：`None` 对应空指针，
/// 引用本身即「存活且只读」证明，本函数无前置条件。
pub fn get_identifier(node: Option<&AstExpr>) -> AstName {
  if let Some(node) = node {
    // Safety: `ast_node_try_as` 先按 class_index 判别，命中 `AstExprGlobal` 才返回 Some，
    // 其动态类型正确且落在入参借用覆盖的节点内，读 `global.name` 合法。
    if let Some(global) = ast_node_try_as::<AstExprGlobal>(&node.base) {
      return global.name;
    }

    // `local.local` 已句柄化恒非空（parser 构造端 `if (value && *value)` 守卫）：
    // .get() 安全借用即存活 AstLocal，null 折叠分支随类型消失。
    if let Some(local) = ast_node_try_as::<AstExprLocal>(&node.base) {
      return local.local.get().name;
    }
  }

  // 空名走 `AstName::new()` 单源，本 crate 不再手写 `value: null()` 字面量。
  AstName::new()
}
