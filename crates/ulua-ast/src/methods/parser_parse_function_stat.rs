use ulua_common::FFlag::LuauExportValueSyntax;

use crate::{
  enums::type_lexer::Type,
  functions::is_expr_l_value::is_expr_l_value,
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_expr::AstExpr, ast_name::AstName,
    ast_node::AstNode, ast_stat_function::AstStatFunction, cst_node::CstNode,
    cst_stat_function::CstStatFunction, location::Location, parser::Parser,
  },
};

impl Parser {
  pub fn parse_function_stat(
    &mut self,
    attributes: &AstArray<*mut AstAttr>,
  ) -> *mut AstStatFunction {
    let start = if attributes.size > 0 {
      unsafe { (**attributes.data).base.location }
    } else {
      self.lexer.current().location
    };

    let match_function = *self.lexer.current();
    self.next_lexeme();

    let mut hasself = false;
    let mut debugname = AstName::new();
    let mut expr = self.parse_function_name(&mut hasself, &mut debugname);

    // C++ 将 expr 重赋值为错误节点后继续走 parse_function_body，不提前返回。
    if !is_expr_l_value(expr) {
      expr = if LuauExportValueSyntax.get() {
        self.report_l_value_error(expr) as *mut AstExpr
      } else {
        let expressions = self.copy_initializer_list_t(&[expr]);
        self.report_expr_error(
          unsafe { (*expr).base.location },
          expressions,
          format_args!("Assigned expression must be a variable or a field"),
        ) as *mut AstExpr
      };
    }

    self.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] += 1;

    let (body, _) = self.parse_function_body(
      hasself,
      &match_function,
      &debugname,
      None,
      attributes,
      false,
    );

    self.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] -= 1;

    let node = unsafe {
      (*self.allocator).alloc(AstStatFunction::new(
        Location::new(start.begin, (*body).base.base.location.end),
        expr,
        body,
      ))
    };

    if self.options.store_cst_data {
      let cst_node =
        unsafe { (*self.allocator).alloc(CstStatFunction::new(match_function.location.begin)) };
      self
        .cst_node_map
        .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
    }

    node
  }
}
