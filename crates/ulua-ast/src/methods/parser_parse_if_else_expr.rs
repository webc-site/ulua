use crate::{
  enums::type_lexer::Type,
  functions::optional_node::slot_ref,
  records::{
    ast_expr::AstExpr,
    ast_expr_if_else::AstExprIfElse,
    cst_expr_if_else::CstExprIfElse,
    location::Location,
    node_handle::{Node, OptNode},
    parser::Parser,
    position::Position,
  },
};

impl Parser {
  pub(crate) fn parse_if_else_expr(&mut self) -> *mut AstExpr {
    let start = self.lexer.current().location;

    self.next_lexeme();

    let condition = self.parse_expr(0);

    let has_then = self.expect_and_consume_type(Type::RESERVED_THEN, "if then else expression");
    let then_position = if has_then {
      self.lexer.previous_location().begin
    } else {
      Position::missing()
    };

    let true_expr = self.parse_expr(0);

    let else_position = self.lexer.current().location.begin;

    let (has_else, false_expr, is_else_if) = if self.lexer.current().r#type == Type::RESERVED_ELSEIF
    {
      let old_recursion_count = self.recursion_counter;
      self.increment_recursion_counter("expression");
      let false_expr = self.parse_if_else_expr();
      self.recursion_counter = old_recursion_count;
      (true, false_expr, true)
    } else {
      let has_else = self.expect_and_consume_type(Type::RESERVED_ELSE, "if then else expression");
      let false_expr = self.parse_expr(0);
      (has_else, false_expr, false)
    };

    // false_expr 取自上方 parse_expr 结果（alloc_expr 恒非空，失败中止），只读拷贝
    // 其基类 location。
    let end = slot_ref(false_expr).base.location;

    let node = self.alloc_expr(AstExprIfElse::new(
      Location::new(start.begin, end.end),
      Node::from_raw(condition),
      has_then,
      Node::from_raw(true_expr),
      has_else,
      Node::from_raw(false_expr),
      // cpp 基础 ctor 默认 `conditionLocal = nullptr`（Ast.h:690）：`if local`/
      // `if const` 语法受上游 DebugLuauIfLocalSyntax 门控、parser 未接入，此处
      // 即 cpp 空槽形态，随字段同步落地。
      OptNode::default(),
    ));

    self.attach_cst(node, |alloc| {
      alloc.alloc(CstExprIfElse::new(then_position, else_position, is_else_if))
    });

    node
  }
}
