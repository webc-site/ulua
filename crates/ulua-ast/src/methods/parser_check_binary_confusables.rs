use crate::{
  enums::type_lexer::Type,
  records::{
    ast_expr_binary::AstExprBinaryOp, binary_op_priority::BinaryOpPriority, location::Location,
    parser::Parser,
  },
};

/// 一个可混淆二元运算符组合：首词素、次词素、纠正后的运算符与提示文案。
struct BinaryConfusable {
  curr: Type,
  next: Type,
  op: AstExprBinaryOp,
  message: &'static str,
}

/// cpp `Parser::checkBinaryConfusables`（Ast/src/Parser.cpp:3607-3625）的三个
/// 同形 if 臂：逐字只差「首/次词素、运算符、文案」，单源化为编译期定表，
/// 分支骨架全仓唯一。行序即 cpp 臂序（判定互斥，顺序仅影响可读性）。
const BINARY_CONFUSABLES: [BinaryConfusable; 3] = [
  BinaryConfusable {
    curr: Type::AMPERSAND,
    next: Type::AMPERSAND,
    op: AstExprBinaryOp::And,
    message: "Unexpected '&&'; did you mean 'and'?",
  },
  BinaryConfusable {
    curr: Type::PIPE,
    next: Type::PIPE,
    op: AstExprBinaryOp::Or,
    message: "Unexpected '||'; did you mean 'or'?",
  },
  BinaryConfusable {
    curr: Type::BANG,
    next: Type::EQUAL_SIGN,
    op: AstExprBinaryOp::CompareNe,
    message: "Unexpected '!='; did you mean '~='?",
  },
];

/// 编译期自检：首词素两两互异——cpp 早退分支以 `!=` 三连枚举首集，表内首词素
/// 若有重复，早退集与查表行为将单边漂移（重复行永远不可达）。
const _: () = {
  let confusables = &BINARY_CONFUSABLES;
  assert!(confusables[0].curr.0 != confusables[1].curr.0);
  assert!(confusables[0].curr.0 != confusables[2].curr.0);
  assert!(confusables[1].curr.0 != confusables[2].curr.0);
};

impl Parser {
  pub(crate) fn check_binary_confusables(
    &mut self,
    binary_priority: &[BinaryOpPriority],
    limit: u32,
  ) -> Option<AstExprBinaryOp> {
    let curr = *self.lexer.current();

    // early-out: need to check if this is a possible confusable quickly
    // （cpp 3607 的三连 `!=` 收口为查表：首词素集即表的内禀属性，无第二份名单）
    if !BINARY_CONFUSABLES.iter().any(|c| c.curr == curr.r#type) {
      return None;
    }

    // slow path: possible confusable
    let start = curr.location;
    let next = self.lexer.lookahead();

    for confusable in &BINARY_CONFUSABLES {
      if confusable.curr == curr.r#type
        && confusable.next == next.r#type
        && curr.location.end == next.location.begin
        && binary_priority[confusable.op as usize].left as u32 > limit
      {
        self.next_lexeme();
        self.report(
          Location {
            begin: start.begin,
            end: next.location.end,
          },
          format_args!("{}", confusable.message),
        );
        return Some(confusable.op);
      }
    }

    None
  }
}
