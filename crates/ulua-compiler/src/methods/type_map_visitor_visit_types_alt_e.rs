use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_node::AstNode,
    ast_stat_local_function::AstStatLocalFunction, ast_type_pack_explicit::AstTypePackExplicit,
  },
  rtti::ast_node_as,
};

use crate::{
  functions::get_function_type::get_function_type, records::type_map_visitor::TypeMapVisitor,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) fn visit_ast_stat_local_function(
  this: &mut TypeMapVisitor<'_>,
  node: *mut AstStatLocalFunction,
) -> bool {
  unsafe {
    if node.is_null() {
      return true;
    }

    let n = &*node;
    if !n.func.is_null() && !(*n.func).return_annotation.is_null() {
      let return_annotation = (*n.func).return_annotation;
      let type_pack = ast_node_as::<AstTypePackExplicit>(return_annotation as *mut AstNode);

      if !type_pack.is_null() {
        let type_list = &(*type_pack).type_list;
        let types = type_list.types.as_slice();
        if !types.is_empty() {
          let first_type = types[0];
          this.function_return_types.try_insert(n.name, first_type);
        }
      }
    }
  }

  true // Let generic visitor step into all expressions
}

impl TypeMapVisitor<'_> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_stat_local_function(&mut self, node: *mut AstStatLocalFunction) -> bool {
    visit_ast_stat_local_function(self, node)
  }

  pub fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
    let type_str = get_function_type(
      node,
      &self.type_aliases,
      self.host_vector_type,
      self.userdata_types,
      &mut *self.bytecode,
    );
    if !type_str.is_empty() {
      *self.function_types.get_or_insert(node) = type_str;
    }
    true
  }
}
