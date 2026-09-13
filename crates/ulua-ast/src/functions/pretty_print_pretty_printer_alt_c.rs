use alloc::string::String;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    pretty_print_pretty_printer::pretty_print_ast_stat_block_cst_node_map,
    pretty_print_with_types_pretty_printer::pretty_print_with_types_ast_stat_block_cst_node_map,
  },
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, location::Location,
    parse_options::ParseOptions, parser::Parser, pretty_print_result::PrettyPrintResult,
  },
};

pub fn pretty_print_string_view_parse_options_bool_bool(
  source: &str,
  mut options: ParseOptions,
  with_types: bool,
  ignore_parse_errors: bool,
) -> PrettyPrintResult {
  options.store_cst_data = true;

  let mut allocator = Allocator::new();
  let mut names = AstNameTable::new(&mut allocator);
  let parse_result = Parser::parse(source, source.len(), &mut names, &mut allocator, options);

  let has_errors = !parse_result.errors.is_empty();

  if has_errors && !ignore_parse_errors {
    let error = &parse_result.errors[0];
    return PrettyPrintResult {
      code: String::new(),
      error_location: *error.get_location(),
      parse_error: error.what().to_string(),
    };
  }

  LUAU_ASSERT!(!parse_result.root.is_null());
  if parse_result.root.is_null() {
    return PrettyPrintResult {
      code: String::new(),
      error_location: Location::default(),
      parse_error: String::from("Internal error: Parser yielded empty parse tree"),
    };
  }

  let root = unsafe { &mut *parse_result.root };
  if with_types {
    PrettyPrintResult {
      code: pretty_print_with_types_ast_stat_block_cst_node_map(root, parse_result.cst_node_map),
      error_location: Location::default(),
      parse_error: String::new(),
    }
  } else {
    PrettyPrintResult {
      code: pretty_print_ast_stat_block_cst_node_map(root, parse_result.cst_node_map),
      error_location: Location::default(),
      parse_error: String::new(),
    }
  }
}
