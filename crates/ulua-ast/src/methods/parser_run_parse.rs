use core::{mem::replace, ptr::null_mut};
use std::{
  mem::take,
  panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
};

use ulua_common::{
  macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE, records::dense_hash_map::DenseHashMap,
};

use crate::{
  functions::install_parse_error_panic_hook::install_parse_error_panic_hook,
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, lexeme::Type, parse_error::ParseError,
    parse_node_result::ParseNodeResult, parse_options::ParseOptions, parser::Parser,
  },
};

impl Parser {
  pub fn run_parse<Node, F>(
    buffer: &str,
    buffer_size: usize,
    names: &mut AstNameTable,
    allocator: &mut Allocator,
    options: ParseOptions,
    f: F,
  ) -> ParseNodeResult<Node>
  where
    F: FnOnce(&mut Parser) -> *mut Node,
  {
    LUAU_TIMETRACE_SCOPE!("Parser::parse", "Parser");

    // Silence the default panic-hook noise for the parser's exception-
    // emulation unwinds (a caught `ParseError` is a normal syntax/limit
    // error, not a crash).
    install_parse_error_panic_hook();

    let mut p = Parser::new(buffer, names, allocator as *mut Allocator, options);

    // C++ try-catch is mapped to a result-like handling of ParseError panics if they occur,
    // but the source uses a catch-block for fatal errors. In Luau's Parser, ParseError
    // is often thrown via a panic-like mechanism in Rust or handled via explicit checks.
    // Following the C++ logic:
    let result = catch_unwind(AssertUnwindSafe(|| {
      let expr = f(&mut p);
      let current_lexeme = p.lexer.current();

      let mut lines = current_lexeme.location.end.line;
      if buffer_size > 0 && buffer.as_bytes()[buffer_size - 1] != b'\n' {
        lines += 1;
      }

      let eof = p.lexer.next_lexeme();
      let mut root = expr;

      if eof.r#type != Type::EOF {
        root = null_mut();
        p.parse_errors.push(ParseError::new(
          eof.location,
          "Expected end of file".to_string(),
        ));
      }

      ParseNodeResult {
        root,
        lines: lines as usize,
        hotcomments: take(&mut p.hotcomments),
        errors: take(&mut p.parse_errors),
        comment_locations: take(&mut p.comment_locations),
        cst_node_map: replace(&mut p.cst_node_map, DenseHashMap::new(null_mut())),
      }
    }));

    match result {
      Ok(res) => res,
      Err(payload) => {
        // If it's a ParseError (the C++ catch (ParseError& err) case)
        if let Some(err) = payload.downcast_ref::<ParseError>() {
          p.parse_errors.push(err.clone());

          ParseNodeResult {
            root: null_mut(),
            lines: 0,
            hotcomments: Vec::new(),
            errors: take(&mut p.parse_errors),
            comment_locations: Vec::new(),
            cst_node_map: replace(&mut p.cst_node_map, DenseHashMap::new(null_mut())),
          }
        } else {
          // Re-panic if it's not a ParseError
          resume_unwind(payload);
        }
      }
    }
  }
}
