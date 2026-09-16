use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
  },
  rtti::ast_node_try_as,
};

use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    lookup_extern_type_prop::lookup_extern_type_prop,
  },
  records::{
    extern_type::ExternType, function_type::FunctionType, lint_deprecated_api::LintDeprecatedApi,
    table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};

impl LintDeprecatedApi {
  pub fn check_ast_expr_index_name_type_id(&mut self, node: &AstExprIndexName, ty: TypeId) {
    let ty = follow_type_id(ty);
    let index = node.index.as_str().unwrap_or("");

    if let Some(extern_type) = get_type_id::<ExternType>(ty) {
      if let Some(prop) = lookup_extern_type_prop(extern_type, index) {
        if prop.deprecated {
          self.report_property(
            &node.base.base.location,
            prop,
            Some(extern_type.name.as_str()),
            index,
          );
        } else if let Some(read_ty) = prop.read_ty {
          self.report_deprecated_function_member(node, read_ty, index);
        }
      }

      return;
    }

    let Some(table) = get_type_id::<TableType>(ty) else {
      return;
    };

    let Some(prop) = table.props.get(index) else {
      return;
    };

    if prop.deprecated {
      let container_name = table.name.as_ref().map(|name| {
        if name.starts_with("typeof(") && name.ends_with(')') {
          &name[7..name.len() - 1]
        } else {
          name.as_str()
        }
      });

      self.report_property(&node.base.base.location, prop, container_name, index);
    } else if let Some(read_ty) = prop.read_ty {
      self.report_deprecated_function_member(node, read_ty, index);
    }
  }

  /// 两个分支共用：`prop.read_ty` 指向弃用函数时的成员上报
  /// （C++ `node->expr->as<AstExprGlobal>()` 取容器名）。
  fn report_deprecated_function_member(
    &mut self,
    node: &AstExprIndexName,
    read_ty: TypeId,
    index: &str,
  ) {
    if let Some(fty) = get_type_id::<FunctionType>(follow_type_id(read_ty))
      && fty.is_deprecated_function
      && !self.in_scope(fty)
    {
      // SAFETY: node.expr 由 AST 父子关系保证存活。
      let container = ast_node_try_as::<AstExprGlobal>(unsafe { &*(node.expr as *const AstNode) })
        .map(|global| global.name.as_str().unwrap_or(""));
      if let Some(info) = fty.deprecated_info.as_deref() {
        self.report_member_info(&node.base.base.location, container, index, info);
      } else {
        self.report_member(&node.base.base.location, container, index);
      }
    }
  }
}
