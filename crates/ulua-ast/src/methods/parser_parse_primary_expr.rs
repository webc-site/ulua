use crate::{
  enums::type_lexer::Type,
  records::{
    ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName, location::Location,
    node_handle::Node, parser::Parser,
  },
};

impl Parser {
  pub fn parse_primary_expr(&mut self, as_statement: bool) -> *mut AstExpr {
    let start = self.lexer.current().location.begin;

    let mut expr = self.parse_prefix_expr();

    let old_recursion_count = self.recursion_counter;

    loop {
      // 单次求值当前 token 型，供 match 派发（原 if-else 链每段重复取值）。
      let ty = self.lexer.current().r#type;

      match ty {
        Type::DOT => {
          let op_position = self.lexer.current().location.begin;
          self.next_lexeme();

          // C++ 此处传 nullptr（其余调用点传 "field name"/"method name" 等），
          // 报错文本为 "Expected identifier, got %s"，不带 " when parsing …"。
          let index = self.parse_index_name("", &op_position);

          // expr 出自 parse_prefix_expr/各 alloc 分支的 arena 分配（恒非空）。
          expr = self.alloc_expr(AstExprIndexName::new(
            Location::new(start, index.location.end),
            Node::from_raw(expr),
            index.name,
            index.location,
            op_position,
            b'.',
          ));
        }
        Type::LBRACKET => expr = self.parse_index_expr(start, expr),
        Type::COLON => expr = self.parse_method_call(start, expr),
        Type::LPAREN => {
          // This error is handled inside 'parseFunctionArgs' as well, but for
          // better error recovery we need to break out the current loop here
          // Safety: `expr` 由 parse_prefix_expr/各 alloc 分支赋值为非空 arena 存活 `AstExpr`；此处仅读基类
          // location 判断行号，不产生 `&mut`，无别名冲突。
          if !as_statement
            && unsafe { (*expr).base.location.end.line != self.lexer.current().location.begin.line }
          {
            self.report_ambiguous_call_error();
            break;
          }

          expr = self.parse_function_args(expr, false);
        }
        Type::LBRACE | Type::RAW_STRING | Type::QUOTED_STRING => {
          expr = self.parse_function_args(expr, false);
        }
        // 显式类型实例化形如 `f<<T>>()`，须回退到普通调用而非误吞 `<`。
        Type::LESS if self.lexer.lookahead().r#type == Type::LESS => {
          // Safety: `expr` 为 parser 产出的非空 arena 存活 `AstExpr`，单线程串行下重建 `&mut` 期间无其它
          // 借用；被调返回的节点随即覆盖 `expr`，旧借用随之结束，不产生重叠 `&mut`。
          expr = self.parse_explicit_type_instantiation_expr(start, unsafe { &mut *expr });
        }
        _ => break,
      }

      self.increment_recursion_counter("expression");
    }

    self.recursion_counter = old_recursion_count;

    expr
  }
}
