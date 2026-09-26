use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  functions::follow_type, records::non_strict_type_checker::NonStrictTypeChecker,
  type_aliases::type_id::TypeId,
};

impl NonStrictTypeChecker {
  /// # Safety
  /// `{expr}` 须指向本次遍历期间存活的 parse-arena 节点：非空、对齐，地址在该 arena 释放前不
  /// 移动；调用方（AstVisitor 遍历驱动）单线程串行访问，函数体内不产生与之重叠的可变借用。
  /// 对应 C++ `TypeId NonStrictTypeChecker::lookupType(AstExpr* expr)` (`cpp/Analysis/src/NonStrictTypeChecker.cpp:264`)。
  pub unsafe fn lookup_type(&mut self, expr: *mut AstExpr) -> TypeId {
    let module = self.module_ref();

    if let Some(ty) = module.ast_types.find(&(expr as *const AstExpr)) {
      // Safety: 进入本分支说明 `expr as *const AstExpr` 是 module.ast_types 已登记的键，
      // 即由约束生成写入的存活 parse-arena AstExpr 节点（非空、对齐）；仅读其 AstNode 首
      // 字段 base.location，全程只读。
      let location = unsafe { (*expr).base.location };
      self.check_for_type_function_inhabitance(follow_type::follow(*ty), location)
    } else if let Some(tp) = module.ast_type_packs.find(&(expr as *const AstExpr)) {
      // Safety: 同上——expr 命中 module.ast_type_packs 键，指向存活对齐的 Arena AstExpr，
      // 仅只读 base.location。
      let location = unsafe { (*expr).base.location };
      let flattened = self.flatten_pack(*tp);
      self.check_for_type_function_inhabitance(flattened, location)
    } else {
      self.builtin_types_ref().any_type
    }
  }
}
