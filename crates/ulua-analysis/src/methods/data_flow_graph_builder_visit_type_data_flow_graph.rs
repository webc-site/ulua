use ulua_ast::records::{
  ast_node::AstNode, ast_type::AstType, ast_type_error::AstTypeError,
  ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
  ast_type_intersection::AstTypeIntersection, ast_type_optional::AstTypeOptional,
  ast_type_reference::AstTypeReference, ast_type_singleton_bool::AstTypeSingletonBool,
  ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
  ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion,
};
use ulua_common::LUAU_ASSERT;

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
impl DataFlowGraphBuilder {
  pub fn visit_type_ast_type(&mut self, t: *mut AstType) {
    unsafe {
      let node = t as *mut AstNode;
      if (*node).is::<AstTypeReference>() {
        self.visit_type_ast_type_reference(node as *mut AstTypeReference);
      } else if (*node).is::<AstTypeTable>() {
        self.visit_type_ast_type_table(node as *mut AstTypeTable);
      } else if (*node).is::<AstTypeFunction>() {
        self.visit_type_ast_type_function(node as *mut AstTypeFunction);
      } else if (*node).is::<AstTypeTypeof>() {
        self.visit_type_ast_type_typeof(node as *mut AstTypeTypeof);
      } else if (*node).is::<AstTypeOptional>() {
      } else if (*node).is::<AstTypeUnion>() {
        self.visit_type_ast_type_union(node as *mut AstTypeUnion);
      } else if (*node).is::<AstTypeIntersection>() {
        self.visit_type_ast_type_intersection(node as *mut AstTypeIntersection);
      } else if (*node).is::<AstTypeError>() {
        self.visit_type_ast_type_error(node as *mut AstTypeError);
      } else if (*node).is::<AstTypeSingletonBool>() || (*node).is::<AstTypeSingletonString>() {
      } else if (*node).is::<AstTypeGroup>() {
        let group = node as *mut AstTypeGroup;
        self.visit_type_ast_type((*group).type_);
      } else {
        LUAU_ASSERT!(false);
      }
    }
  }
}
