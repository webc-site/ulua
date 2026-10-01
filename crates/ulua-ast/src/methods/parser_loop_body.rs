//! while/repeat/for 循环体解析的公共骨架：cpp `Parser.cpp` 的 `parseWhile` /
//! `parseRepeat` / `parseFor` 两分支各自重复的 `functionStack.back().loopDepth`
//! 升降段与 `end` 收尾段（取语句终点坐标、消费 `end`、回填 `body.hasEnd`）。

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_stat_block::AstStatBlock, location::Location, match_lexeme::MatchLexeme, node_handle::Node,
    parser::Parser,
  },
};

impl Parser {
  /// 循环体公共骨架：解析体前把当前函数帧 `loop_depth` +1、解析后 -1。
  /// `f` 内完成循环体解析（for 的控制变量入栈也在其中）。
  pub(crate) fn with_loop_depth<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
    self
      .function_stack
      .last_mut()
      .expect("Parser::new 构造期压入的顶层 chunk 底帧永不出栈，循环访问此处栈必非空")
      .loop_depth += 1;

    let result = f(self);

    self
      .function_stack
      .last_mut()
      .expect("Parser::new 构造期压入的顶层 chunk 底帧永不出栈，循环访问此处栈必非空")
      .loop_depth -= 1;
    result
  }

  /// while/for 共享收尾：取当前词素位置为语句 end 坐标、消费 `end`（错配恢复见
  /// [`Parser::expect_match_end_and_consume`]）、把结果回填 `body.has_end`。
  ///
  /// `body` 以句柄传入（`Node` 为 `Copy`，调用点经 [`Node::from_raw`] 一次建槽、
  /// 副本传参）：非空性由 `from_raw` 的构造端拦截证明，写穿 `has_end` 走
  /// `DerefMut`，全程无 `unsafe`、无裸指针契约。
  pub(crate) fn consume_loop_end(
    &mut self,
    begin: &MatchLexeme,
    mut body: Node<AstStatBlock>,
  ) -> Location {
    let end = self.lexer.current().location;
    let has_end = self.expect_match_end_and_consume(Type::RESERVED_END, begin);
    body.has_end = has_end;
    end
  }
}
