use crate::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{ast_expr_function::AstExprFunction, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_pack_visit_ref, ast_type_visit},
};

// cpp `AstExprFunction::visit(AstVisitor*)`（`Ast/src/Ast.cpp:336`）：`this`
// 是非 const 指针，visitor（如 Analysis 的 TypeAttacher::attachTypes）会在
// dispatch 期间写穿本节点回填 `returnAnnotation`。本 trait 因此取
// `&mut self`（见 `visit.rs` 的总说明），交出的 `&mut Self` 由独占借用
// 派生，写穿合法——无需再以 `black_box` 遮掩 `&self` 的 readonly 属性。
impl_visitable!(AstExprFunction, ExprFunction, |this, visitor| {
  for arg in this.args.iter() {
    // AstLocal.annotation 尚未句柄化（wave-2 字段）：null 短路仍由
    // `ast_type_visit` 的指针门面承担。
    // Safety: annotation 为 null 或存活 AstType 节点（parser arena 接线契约）。
    unsafe { ast_type_visit(arg.annotation, visitor) };
  }

  if let Some(pack) = this.vararg_annotation.get_mut() {
    ast_type_pack_visit_ref(pack, visitor);
  }

  if let Some(pack) = this.return_annotation.get_mut() {
    ast_type_pack_visit_ref(pack, visitor);
  }

  ast_stat_block_visit(this.body.get_mut(), visitor);
});
