//! Source: `Ast/src/Parser.cpp:4068`
//!
//! Faithful port of `Parser::parseFunctionArgs` — the function-call argument
//! parser (`args ::= '(' [explist] ')' | tableconstructor | String`). Each
//! variant produces an `AstExprCall` node; the parenthesised branch collects
//! comma positions under `store_cst_data` and records CST open/close parens.

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::slot_ref,
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_type_or_pack::AstTypeOrPack, cst_expr_call::CstExprCall, location::Location,
    match_lexeme::MatchLexeme, parser::Parser, position::Position, temp_vector::TempVector,
  },
};

impl Parser {
  pub(crate) fn parse_function_args(&mut self, func: *mut AstExpr, self_: bool) -> *mut AstExpr {
    // `func` 是 parser 传入/产出的非空 arena 存活节点；一次性读取其基类 location
    // 供后续各分支复用 begin/end，仅读不写。
    let func_location = slot_ref(func).base.location;

    if self.lexer.current().r#type == Type::LPAREN {
      let arg_start = self.lexer.current().location.end;

      if func_location.end.line != self.lexer.current().location.begin.line {
        self.report_ambiguous_call_error();
      }

      let match_paren = MatchLexeme::new(self.lexer.current());
      self.next_lexeme();

      let mut args = TempVector::new(&mut self.scratch_expr);
      let mut comma_positions = TempVector::new(&mut self.scratch_position);

      if self.lexer.current().r#type != Type::RPAREN {
        self.parse_expr_list(
          &mut args,
          if self.options.store_cst_data {
            Some(&mut comma_positions)
          } else {
            None
          },
        );
      }

      let end = self.lexer.current().location;
      let arg_end = end.end;

      let closing_paren_found = self.expect_match_and_consume(')', &match_paren, true);

      let args_array = self.copy_temp_vector_t(&args);
      let explicit_types: AstArray<AstTypeOrPack> = AstArray::EMPTY;

      let node = self.alloc_expr(AstExprCall::new(
        Location::new(func_location.begin, end.end),
        func,
        args_array,
        self_,
        explicit_types,
        Location::new(arg_start, arg_end),
      ));

      if self.options.store_cst_data {
        let comma_positions_array = self.copy_temp_vector_t(&comma_positions);
        let close_position = if closing_paren_found {
          self.lexer.previous_location().begin
        } else {
          Position::missing()
        };
        self.attach_cst(node, |alloc| {
          alloc.alloc(CstExprCall::new(
            match_paren.position,
            close_position,
            comma_positions_array,
          ))
        });
      }

      node
    } else if self.lexer.current().r#type == Type::LBRACE {
      let arg_start = self.lexer.current().location.end;
      let expr = self.parse_table_constructor();
      let expr_location = slot_ref(expr).base.location;
      let arg_end = self.lexer.previous_location().end;

      let exprs = self.copy_initializer_list_t(&[expr]);
      let explicit_types: AstArray<AstTypeOrPack> = AstArray::EMPTY;

      let node = self.alloc_expr(AstExprCall::new(
        Location::new(func_location.begin, expr_location.end),
        func,
        exprs,
        self_,
        explicit_types,
        Location::new(arg_start, arg_end),
      ));

      self.attach_cst(node, |alloc| {
        alloc.alloc(CstExprCall::new(
          Position::missing(),
          Position::missing(),
          AstArray::EMPTY,
        ))
      });

      node
    } else if self.lexer.current().r#type == Type::RAW_STRING
      || self.lexer.current().r#type == Type::QUOTED_STRING
    {
      let arg_location = self.lexer.current().location;
      let expr = self.parse_string();
      let expr_location = slot_ref(expr).base.location;

      let exprs = self.copy_initializer_list_t(&[expr]);
      let explicit_types: AstArray<AstTypeOrPack> = AstArray::EMPTY;

      let node = self.alloc_expr(AstExprCall::new(
        Location::new(func_location.begin, expr_location.end),
        func,
        exprs,
        self_,
        explicit_types,
        arg_location,
      ));

      self.attach_cst(node, |alloc| {
        alloc.alloc(CstExprCall::new(
          Position::missing(),
          Position::missing(),
          AstArray::EMPTY,
        ))
      });

      node
    } else {
      self.report_function_args_error(func, self_)
    }
  }
}
