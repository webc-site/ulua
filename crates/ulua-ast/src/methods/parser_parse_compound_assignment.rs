use crate::{
  functions::optional_node::slot_ref,
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, ast_stat::AstStat,
    ast_stat_compound_assign::AstStatCompoundAssign,
    cst_stat_compound_assign::CstStatCompoundAssign, location::Location, node_handle::Node,
    parser::Parser,
  },
};

impl Parser {
  pub(crate) fn parse_compound_assignment(
    &mut self,
    mut initial: *mut AstExpr,
    op: AstExprBinaryOp,
  ) -> *mut AstStat {
    // cpp Parser.cpp:2236-2242：非 l-value 时把 initial 替换为报错节点后继续
    // 消费运算符并解析右侧（不提前 return），否则调用方的语句循环会原地死循环。
    if !self.is_expr_l_value(initial) {
      initial = self.report_l_value_error(initial);
    }

    let op_position = self.lexer.current().location.begin;
    self.next_lexeme();

    let value = self.parse_expr(0);

    // `initial` 若非 l-value 已被 `report_l_value_error` 归一为报错节点、否则为 parse_expr 产出的
    // 存活 arena 节点，均非空；只读基类 location。
    let begin = slot_ref(initial).base.location.begin;
    let end = slot_ref(value).base.location.end;

    let node = self.alloc_stat(AstStatCompoundAssign::new(
      Location::new(begin, end),
      op,
      Node::from_raw(initial),
      Node::from_raw(value),
    ));

    self.attach_cst(node, |alloc| {
      alloc.alloc(CstStatCompoundAssign::new(op_position))
    });

    node
  }
}
