//! Source: `Ast/src/Parser.cpp:2004`
//!
//! Faithful port of `Parser::parseAssignment` — `var {, var} = expr {, expr}`.
//! Each assignment target is validated as an l-value (a non-l-value is replaced
//! by an error node, gated on the export-value flag pair like the C++). The
//! target and value lists are gathered in scratch arenas (vars/values share the
//! comma-position arena, stack-disciplined) and copied into the node.

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::slot_ref,
  records::{
    ast_expr::AstExpr, ast_stat::AstStat, ast_stat_assign::AstStatAssign,
    cst_stat_assign::CstStatAssign, location::Location, node_handle::Node, parser::Parser,
    temp_vector::TempVector,
  },
};

impl Parser {
  pub(crate) fn parse_assignment(&mut self, mut initial: *mut AstExpr) -> *mut AstStat {
    if !self.is_expr_l_value(initial) {
      initial = self.report_l_value_error(initial);
    }

    let mut vars = TempVector::new(&mut self.scratch_expr);
    let mut vars_comma_positions = TempVector::new(&mut self.scratch_position);
    // 入 scratch 即折算为 arena 句柄：initial/expr 出自 parse_primary_expr 或
    // report_l_value_error，两者都是 arena 分配产物（失败即中止），恒非空。
    vars.push_back(Node::from_raw(initial));

    while self.lexer.current().r#type == Type::COMMA {
      if self.options.store_cst_data {
        vars_comma_positions.push_back(self.lexer.current().location.begin);
      }
      self.next_lexeme();

      let mut expr = self.parse_primary_expr(true);

      if !self.is_expr_l_value(expr) {
        expr = self.report_l_value_error(expr);
      }

      vars.push_back(Node::from_raw(expr));
    }

    let equals_position = self.expect_and_consume_char_position('=', "assignment");

    let mut values = TempVector::new(&mut self.scratch_expr_aux);
    let mut values_comma_positions = TempVector::new(&mut self.scratch_position);
    self.parse_expr_list(
      &mut values,
      if self.options.store_cst_data {
        Some(&mut values_comma_positions)
      } else {
        None
      },
    );

    let vars_array = self.copy_temp_vector_ptrs(&vars);
    let values_array = self.copy_temp_vector_ptrs(&values);

    let node = self.alloc_stat(AstStatAssign::new(
      Location::new(
        // initial 是调用方 parse_primary_expr 刚 arena 分配的存活 AstExpr（或经
        // report_l_value_error 替换的存活错误节点）；slot_ref 把槽位读成共享
        // 引用后拷贝基类前缀字段，全程 safe。
        slot_ref(initial).base.location.begin,
        // values 由 parse_expr_list 至少推入一条表达式（其首元素在循环前 push），
        // 元素均为 arena 句柄，last() 经 Deref 读末元素基类 location.end。
        values.last().expect("values 非空").base.location.end,
      ),
      vars_array,
      values_array,
    ));

    if self.options.store_cst_data {
      let vars_comma = self.copy_temp_vector_t(&vars_comma_positions);
      let values_comma = self.copy_temp_vector_t(&values_comma_positions);
      self.attach_cst(node, |alloc| {
        alloc.alloc(CstStatAssign::new(
          vars_comma,
          equals_position,
          values_comma,
        ))
      });
    }

    node
  }
}
