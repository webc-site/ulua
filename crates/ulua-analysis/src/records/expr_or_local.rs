use core::ptr::null_mut;

use ulua_ast::records::{ast_expr::AstExpr, ast_local::AstLocal};
#[derive(Debug, Clone, Copy)]
pub struct ExprOrLocal {
  pub(crate) expr: *mut AstExpr,
  pub(crate) local: *mut AstLocal,
}

// 保留 `Default` 的 null 哨兵（review.md §2(b) 型「定义处收口的规范空值」，
// 非可改 `Option` 的可选依赖）：本结构是「expr 或 local 二选一」的联合槽，
// cpp `ExprOrLocal` 的 `AstExpr* expr = nullptr; AstLocal* local = nullptr;`
// 默认成员初始化即「两臂皆空 = 未命中」，这是唯一合法的空态。读取臂的判空
// （`get_location`/`get_name`/`get_expr`/`get_local`）与互斥写入
// （`set_expr`/`set_local`，写一臂即清另一臂为 null）都定义在
// `methods/expr_or_local.rs`，把 `expr`/`local` 改成 `Option<&T>` 需要连带
// 改这些 `methods/` 内的解引用与清空点，属越界。故本处维持「空 = null」的
// 直存可空句柄形态，并作为全 crate 该联合槽空值的唯一定义处
// （`find_expr_or_local.rs::new` 亦统一经 `ExprOrLocal::default()` 取空，
// 业务侧不再散落 null 字面量）。
impl Default for ExprOrLocal {
  fn default() -> Self {
    Self {
      // 既有约定（review.md §2）：两臂皆空 = 未命中，为该联合槽唯一合法空态（详见上方块注释）。
      expr: null_mut(),
      local: null_mut(),
    }
  }
}
