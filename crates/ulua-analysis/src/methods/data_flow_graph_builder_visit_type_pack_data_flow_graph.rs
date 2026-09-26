use ulua_ast::{
  records::{
    ast_node::AstNode, ast_type::AstType, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_pack_generic::AstTypePackGeneric,
    ast_type_pack_variadic::AstTypePackVariadic,
  },
  rtti::AstNodeClass,
};
use ulua_common::LUAU_ASSERT;

use crate::{
  functions::{arena_ref::arena_ref, ast_node_downcast::ast_node_downcast as pack_downcast},
  records::data_flow_graph_builder::DataFlowGraphBuilder,
};

impl DataFlowGraphBuilder {
  /// cpp `visit(AstTypePack*)` 的分派入口。
  pub fn visit_type_pack(&mut self, p: &AstTypePack) {
    // 类索引 match 与原 `ast_node_is` 长链同一判据且互斥（rtti_indices_unique
    // 测试保证），臂序无关语义。
    let node: &AstNode = &p.base;
    match node.class_index {
      AstTypePackExplicit::CLASS_INDEX => {
        self.visit_type_pack_explicit(pack_downcast::<AstTypePackExplicit>(node));
      }
      AstTypePackVariadic::CLASS_INDEX => {
        self.visit_type_pack_variadic(pack_downcast::<AstTypePackVariadic>(node));
      }
      AstTypePackGeneric::CLASS_INDEX => {
        // ok
      }
      _ => {
        LUAU_ASSERT!(false);
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
