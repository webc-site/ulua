//! Source: `Ast/src/Parser.cpp:726`
//!
//! Faithful port of `Parser::parseFor` — both numeric `for i = a, b [, c] do`
//! and generic `for a, b in exprs do` loops. The first binding is parsed before
//! the `=`/`in` is known; numeric loops push the single control variable, while
//! generic loops parse a binding list and an expression list, copying both into
//! arena `AstArray`s for `AstStatForIn`. CST positions (commas / `=` / colon
//! annotations) are recorded only under `store_cst_data`.

use core::ptr::NonNull;

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::node_opt,
  records::{
    ast_array::AstArray,
    ast_expr::AstExpr,
    ast_stat::AstStat,
    ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn,
    cst_stat_for::CstStatFor,
    cst_stat_for_in::CstStatForIn,
    location::Location,
    match_lexeme::MatchLexeme,
    node_handle::{Node, OptNode},
    parser::Parser,
    position::Position,
    temp_vector::TempVector,
  },
};

impl Parser {
  pub fn parse_for(&mut self) -> *mut AstStat {
    let start = self.lexer.current().location;

    self.next_lexeme(); // for

    let varname = self.parse_binding(false);

    if self.lexer.current().r#type == Type::EQUAL_SIGN {
      let equals_position = self.lexer.current().location.begin;
      self.next_lexeme();

      let from = self.parse_expr(0);

      let has_end_comma = self.expect_and_consume_char(',', "index range");
      let end_comma_position = if has_end_comma {
        self.lexer.previous_location().begin
      } else {
        Position::missing()
      };

      let to = self.parse_expr(0);

      let mut step_comma_position = Position::missing();
      let mut step: Option<NonNull<AstExpr>> = None;

      if self.lexer.current().r#type == Type::COMMA {
        step_comma_position = self.lexer.current().location.begin;
        self.next_lexeme();

        step = node_opt(self.parse_expr(0));
      }

      let match_do = *self.lexer.current();
      let has_do = self.expect_and_consume_type(Type::RESERVED_DO, "for loop");

      let locals_begin = self.save_locals();

      // loop_depth 升降为循环骨架（见 parser_loop_body）：控制变量入栈须在提升后、体解析前
      let (var, body) = self.with_loop_depth(|p| {
        let var = p.push_local(&varname);
        (var, p.parse_block())
      });

      self.restore_locals(locals_begin);

      // Safety: `body` 由 parse_block 返回、指向 arena 存活的 `AstStatBlock`（非空）。
      let end = unsafe { self.consume_loop_end(&MatchLexeme::new(&match_do), body) };

      let node = self.alloc_stat(AstStatFor::new(
        Location::new(start.begin, end.end),
        // 槽位收进 arena 句柄：var 出自 push_local(alloc 恒非空)、from/to/body
        // 出自 parse_expr/parse_block(恒非空)，step 以 Option<NonNull> 原样落
        // OptNode（论证见 records::ast_stat_for 字段注释）。
        Node::from_raw(var),
        Node::from_raw(from),
        Node::from_raw(to),
        OptNode::from_non_null(step),
        Node::from_raw(body),
        has_do,
        match_do.location,
      ));

      self.attach_cst(node, |alloc| {
        alloc.alloc(CstStatFor::new(
          varname.colon_position,
          equals_position,
          end_comma_position,
          step_comma_position,
        ))
      });

      node
    } else {
      let mut names = TempVector::new(&mut self.scratch_binding);
      let mut vars_comma_position: AstArray<Position> = AstArray::EMPTY;
      names.push_back(varname);

      if self.lexer.current().r#type == Type::COMMA {
        if self.options.store_cst_data {
          let initial_comma_position = self.lexer.current().location.begin;
          self.next_lexeme();
          let _ = self.parse_binding_list(
            &mut names,
            false,
            Some(&mut vars_comma_position),
            Some(&initial_comma_position),
            None,
            false,
          );
        } else {
          self.next_lexeme();
          let _ = self.parse_binding_list(&mut names, false, None, None, None, false);
        }
      }

      let in_location = self.lexer.current().location;
      let has_in = self.expect_and_consume_type(Type::RESERVED_IN, "for loop");

      let mut values = TempVector::new(&mut self.scratch_expr);
      let mut values_comma_positions = TempVector::new(&mut self.scratch_position);
      self.parse_expr_list(
        &mut values,
        if self.options.store_cst_data {
          Some(&mut values_comma_positions)
        } else {
          None
        },
      );

      let match_do = *self.lexer.current();
      let has_do = self.expect_and_consume_type(Type::RESERVED_DO, "for loop");

      let locals_begin = self.save_locals();

      let mut vars = TempVector::new(&mut self.scratch_local);

      // loop_depth 升降为循环骨架（见 parser_loop_body）：全部控制变量入栈后再解析体
      let body = self.with_loop_depth(|p| {
        for name in names.iter() {
          let local = p.push_local(name);
          vars.push_back(local);
        }
        p.parse_block()
      });

      self.restore_locals(locals_begin);

      // Safety: `body` 由 parse_block 返回、指向 arena 存活的 `AstStatBlock`（非空）。
      let end = unsafe { self.consume_loop_end(&MatchLexeme::new(&match_do), body) };

      let vars_array = self.copy_temp_vector_t(&vars);
      let values_array = self.copy_temp_vector_t(&values);

      let node = self.alloc_stat(AstStatForIn::new(
        Location::new(start.begin, end.end),
        vars_array,
        values_array,
        // body 收进 arena 句柄：parse_block 恒非空（论证见 records::ast_stat_for_in
        // 字段注释）。
        Node::from_raw(body),
        has_in,
        in_location,
        has_do,
        match_do.location,
      ));

      if self.options.store_cst_data {
        let annotation = self.extract_annotation_colon_positions(&names);
        let values_comma = self.copy_temp_vector_t(&values_comma_positions);
        self.attach_cst(node, |alloc| {
          alloc.alloc(CstStatForIn::new(
            annotation,
            vars_comma_position,
            values_comma,
          ))
        });
      }

      node
    }
  }
}
