use ulua_ast::records::{ast_expr::AstExpr, location::Location};

use crate::{
  enums::value_context::ValueContext, records::type_checker_2::TypeChecker2,
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  /// 对应 C++ `TypeChecker2` 的 visitExprName 助手段（`cpp/Analysis/src/TypeChecker2.cpp:2267`）：
  /// 先查 expr 推断类型、剥 nil 再核对索引属性。
  ///
  /// 降 safe 说明：`expr` 原为 `*mut AstExpr`，函数体只把它当存活节点做只读遍历
  /// （`visit_expr`/`lookup_type` 均收引用），故形参直接收窄为 `&AstExpr`，
  /// 裸指针派生留在调用点既有 unsafe 处。
  pub fn visit_expr_name(
    &mut self,
    expr: &AstExpr,
    location: Location,
    prop_name: &str,
    context: ValueContext,
    ast_index_expr_ty: TypeId,
  ) {
    self.visit_expr(expr, ValueContext::RValue);
    let inferred = self.lookup_type(expr);
    let left_type = self.strip_from_nil_and_report(inferred, &location);
    self.check_index_type_from_type(left_type, prop_name, context, location, ast_index_expr_ty);
  }
}
