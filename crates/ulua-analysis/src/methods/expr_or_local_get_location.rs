use ulua_ast::records::location::Location;

use crate::records::expr_or_local::ExprOrLocal;

impl ExprOrLocal {
  pub fn get_location(&self) -> Option<Location> {
    let expr = self.get_expr();
    if !expr.is_null() {
      // Safety: 先判空再解引用。`expr` 是 ExprOrLocal 记录的 AST 节点裸指针，只可能由
      // parser 产出的 `*mut AstExpr` 填入（`Default` 为 null，表示「本变体未使用」），
      // 节点位于 AST arena 的 bump 块内、地址不移动，且 `&self` 期间该 arena 存活。
      // 这里只一次性拷贝 `base.location`（`#[repr(C)]` 首字段，与派生体基址重合的
      // `AstNode::location`），不构造长期引用，故无别名与悬垂风险。
      return Some(unsafe { (*expr).base.location });
    }

    let local = self.get_local();
    if !local.is_null() {
      // Safety: 同上——`local` 由 parser 建立的 `AstLocal` 节点地址填入（未使用时为 null，
      // 已由判空排除），AST arena 保活且块地址不移动；仅拷贝 `location` 这个 `Copy` 字段。
      return Some(unsafe { (*local).location });
    }

    None
  }
}
