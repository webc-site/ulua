use ulua_ast::{
  enums::ast_type_pack_ref::AstTypePackRef,
  records::{
    ast_type::AstType, ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_variadic::AstTypePackVariadic,
  },
};

use crate::{
  functions::arena_ref::arena_ref, records::data_flow_graph_builder::DataFlowGraphBuilder,
};

impl DataFlowGraphBuilder {
  /// cpp `visit(AstTypePack*)` 的分派入口。
  pub fn visit_type_pack(&mut self, p: &AstTypePack) {
    match p.as_pack_ref() {
      AstTypePackRef::Explicit(e) => {
        self.visit_type_pack_explicit(e);
      }
      AstTypePackRef::Variadic(v) => {
        self.visit_type_pack_variadic(v);
      }
      AstTypePackRef::Generic(_) => {
        // ok
      }
    }
  }

  /// cpp `visit(AstTypePackExplicit*)`：展开显式类型包。
  pub fn visit_type_pack_explicit(&mut self, e: &AstTypePackExplicit) {
    self.visit_type_list(e.type_list);
  }

  /// cpp `visit(AstTypePackVariadic*)`：登记 `T...` 的元素类型。
  pub fn visit_type_pack_variadic(&mut self, v: &AstTypePackVariadic) {
    // SAFETY: variadic_type 由 parser 契约保证非空（cpp 直接解引用）。
    let variadic_type = arena_ref::<AstType>(v.variadic_type, "AstTypePackVariadic.variadic_type");
    self.visit_type(variadic_type);
  }
}
