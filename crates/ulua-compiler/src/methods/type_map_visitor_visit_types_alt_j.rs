use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
    ast_node::AstNode, ast_type::AstType, ast_type_table::AstTypeTable,
  },
  rtti::ast_node_as,
  visit::ast_expr_visit,
};
use ulua_common::enums::luau_bytecode_type::{
  LBC_TYPE_ANY, LBC_TYPE_BOOLEAN, LBC_TYPE_INTEGER, LBC_TYPE_NUMBER, LBC_TYPE_STRING,
  LBC_TYPE_VECTOR, LuauBytecodeType,
};

use crate::{
  functions::is_matching_global_member::is_matching_global_member,
  records::type_map_visitor::TypeMapVisitor,
};

impl TypeMapVisitor<'_> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_expr_index_name(&mut self, node: *mut AstExprIndexName) -> bool {
    unsafe {
      if node.is_null() {
        return false;
      }

      let node_ref = &*node;

      ast_expr_visit(node_ref.expr, self);

      if let Some(&type_ptr) = self.resolved_exprs.find(&node_ref.expr) {
        let table_ty_ptr = ast_node_as::<AstTypeTable>(type_ptr as *mut AstNode);

        if !table_ty_ptr.is_null() {
          let table_ty = &*table_ty_ptr;
          for prop in table_ty.props.iter() {
            if prop.name.value == node_ref.index.value {
              self.record_resolved_type_ast_expr_ast_type(node as *mut _, prop.r#type);
              return false;
            }
          }
        }
      }

      if let Some(&type_bc) = self.expr_types.find(&node_ref.expr)
        && type_bc == LBC_TYPE_VECTOR
      {
        let is_xyz = matches!(
          node_ref.index.as_bytes(),
          b"X" | b"Y" | b"Z" | b"x" | b"y" | b"z"
        );

        if is_xyz {
          self.record_resolved_type_ast_expr_ast_type(
            node as *mut _,
            &self.builtin_types.number_type as *const _ as *const AstType,
          );
          return false;
        }
      }

      if is_matching_global_member(self.globals, node, "vector", "zero")
        || is_matching_global_member(self.globals, node, "vector", "one")
      {
        self.record_resolved_type_ast_expr_ast_type(
          node as *mut _,
          &self.builtin_types.vector_type as *const _ as *const AstType,
        );
        return false;
      }

      if let Some(library_member_type_cb) = self.library_member_type_cb {
        let object_ptr = ast_node_as::<AstExprGlobal>(node_ref.expr as *mut AstNode);
        if !object_ptr.is_null() {
          let object = &*object_ptr;
          let raw_ty = library_member_type_cb(object.name.value, node_ref.index.value);
          let ty = LuauBytecodeType(raw_ty as u16);

          if ty != LBC_TYPE_ANY {
            match ty {
              LBC_TYPE_BOOLEAN => {
                self.resolved_exprs.try_insert(
                  node as *mut AstExpr,
                  &self.builtin_types.boolean_type as *const _ as *const AstType,
                );
              }
              LBC_TYPE_NUMBER => {
                self.resolved_exprs.try_insert(
                  node as *mut AstExpr,
                  &self.builtin_types.number_type as *const _ as *const AstType,
                );
              }
              LBC_TYPE_INTEGER => {
                self.resolved_exprs.try_insert(
                  node as *mut AstExpr,
                  &self.builtin_types.integer_type as *const _ as *const AstType,
                );
              }
              LBC_TYPE_STRING => {
                self.resolved_exprs.try_insert(
                  node as *mut AstExpr,
                  &self.builtin_types.string_type as *const _ as *const AstType,
                );
              }
              LBC_TYPE_VECTOR => {
                self.resolved_exprs.try_insert(
                  node as *mut AstExpr,
                  &self.builtin_types.vector_type as *const _ as *const AstType,
                );
              }
              _ => {}
            }

            self.expr_types.try_insert(node as *mut AstExpr, ty);
            return false;
          }
        }
      }

      false
    }
  }
}
