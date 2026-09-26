use ulua_ast::records::ast_expr::AstExpr;

use crate::{records::type_checker_2::TypeChecker2, type_aliases::type_id::TypeId};

impl TypeChecker2 {
  /// # Safety
  /// `expr` 须指向本次检查存活于模块 AST arena 的表达式节点（C++
  /// `AstExpr*` 入参契约，由 visit_* 调用侧以 arena 节点指针成立）。
  pub unsafe fn test_literal_or_ast_type_is_subtype(
    &mut self,
    expr: *mut AstExpr,
    expected_type: TypeId,
  ) -> bool {
    // Safety: expr 满足本 fn 的 # Safety 契约——指向 parse arena 存活节点，检查
    // 期 AST 不可变；函数头一次取得共享借用，替代原三处同址 `&*expr` 解引用
    // （location 读取、lookup_type、test_potential_literal 传参，均只读，
    // 观察行为一致）。
    let expr_ref: &AstExpr = unsafe { &*expr };
    let scope = self.find_innermost_scope(expr_ref.base.location);
    let expr_ty = self.lookup_type(expr_ref);

    // self.subtyping 已句柄化（构造期接线指向自有 _subtyping 字段的句柄，
    // C++ 引用成员直译），借用经 `subtyping_mut` 止于本语句，
    // 此刻由本 fn 对 self 的 &mut 独占，单线程下无并存别名。
    let r = self
      .subtyping_mut()
      .is_subtype_type_id_type_id_not_null_scope(expr_ty, expected_type, scope);

    if r.is_subtype {
      return true;
    }

    self.test_potential_literal_is_subtype(expr_ref, expected_type)
  }
}
