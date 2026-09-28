use ulua_common::fflag::LuauExportValueSyntax;

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::slot_ref,
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_name::AstName, ast_stat_function::AstStatFunction,
    cst_stat_function::CstStatFunction, location::Location, node_handle::Node, parser::Parser,
  },
};

impl Parser {
  pub fn parse_function_stat(
    &mut self,
    attributes: &AstArray<*mut AstAttr>,
  ) -> *mut AstStatFunction {
    let start = self.first_attr_location(attributes, self.lexer.current().location);

    let match_function = *self.lexer.current();
    self.next_lexeme();

    let mut hasself = false;
    let mut debugname = AstName::new();
    let mut expr = self.parse_function_name(&mut hasself, &mut debugname);

    // C++ 将 expr 重赋值为错误节点后继续走 parse_function_body，不提前返回。
    if !self.is_expr_l_value(expr) {
      expr = if LuauExportValueSyntax.get() {
        self.report_l_value_error(expr)
      } else {
        let expressions = self.copy_initializer_list_t(&[expr]);
        self.report_expr_error(
          slot_ref(expr).base.location,
          expressions,
          format_args!("Assigned expression must be a variable or a field"),
        )
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

    let node = self.alloc(AstStatFunction::new(
      Location::new(start.begin, slot_ref(body).base.base.location.end),
      // parseNameExpr/report_*_error 与 parse_function_body 恒交回 arena 非空
      // 节点（论证见 records::ast_stat_function 字段注释），from_raw 建句柄。
      Node::from_raw(expr),
      Node::from_raw(body),
    ));

    if self.options.store_cst_data {
      self.attach_cst(node, |alloc| {
        alloc.alloc(CstStatFunction::new(match_function.location.begin))
      });
    }

    node
  }
}
