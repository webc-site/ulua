//! Source: `CLI/src/Ast.cpp:24-90` (hand-ported)
/// C++ `int main(int argc, char** argv)` (`CLI/src/Ast.cpp:24-90`).
use alloc::boxed::Box;
use alloc::{string::String, vec::Vec};
use std::{env::args, process::exit};

use ulua_analysis::functions::{
  to_json_ast_json_encoder_alt_b::to_json, to_string_to_string_alt_t::to_string_location_i32_bool,
};
use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, ast_node::AstNode,
  parse_options::ParseOptions, parser::Parser,
};
use ulua_cli_lib::functions::{
  read_file::read_file, read_stdin::read_stdin, set_luau_flags_default::set_luau_flags_default,
};
use ulua_common::functions::assert_handler::assert_handler;

use crate::functions::{assertion_handler::assertion_handler, display_help::display_help};
pub fn main() {
  exit(run());
}

fn run() -> i32 {
  let args: Vec<String> = args().collect();
  let argc = args.len();

  // Luau::assertHandler() = assertionHandler;
  *assert_handler() = Some(assertion_handler);

  // for (FValue<bool>* flag = ...; flag; flag = flag->next)
  //     if (strncmp(flag->name, "Luau", 4) == 0) flag->value = true;
  // The shared port enables every `Luau`-prefixed flag (matching the C++ loop;
  // it additionally skips experimental flags, the project's CLI convention).
  set_luau_flags_default();

  // if (argc >= 2 && strcmp(argv[1], "--help") == 0) { displayHelp(argv[0]); return 0; }
  if argc >= 2 && args[1] == "--help" {
    display_help(&args[0]);
    return 0;
  }
  // else if (argc < 2) { displayHelp(argv[0]); return 1; }
  else if argc < 2 {
    display_help(&args[0]);
    return 1;
  }

  // const char* name = argv[1];
  let name = &args[1];

  // std::optional<std::string> maybeSource;
  // if (strcmp(name, "-") == 0) maybeSource = readStdin(); else maybeSource = readFile(name);
  let maybe_source = if name == "-" {
    read_stdin()
  } else {
    read_file(name)
  };

  // if (!maybeSource) { fprintf(stderr, "Couldn't read source %s\n", name); return 1; }
  let source = match maybe_source {
    Some(s) => s,
    None => {
      eprintln!("Couldn't read source {}", name);
      return 1;
    }
  };

  // Luau::Allocator allocator; Luau::AstNameTable names(allocator);
  // The `AstNameTable` keeps a `*mut Allocator`, so both are boxed for a stable
  // address (mirrors the parser test `Fixture`).
  let mut allocator = Box::new(Allocator::new());
  let mut names = Box::new(AstNameTable::new(&mut allocator));

  // ParseOptions options; options.captureComments = true; options.allowDeclarationSyntax = true;
  let options = ParseOptions {
    capture_comments: true,
    allow_declaration_syntax: true,
    ..Default::default()
  };

  // ParseResult parseResult = Parser::parse(source.data(), source.size(), names, allocator, std::move(options));
  let parse_result = Parser::parse(&source, source.len(), &mut names, &mut allocator, options);

  // if (parseResult.errors.size() > 0) { ... print each error ... }
  if !parse_result.errors.is_empty() {
    eprintln!("Parse errors were encountered:");
    for error in &parse_result.errors {
      eprintln!(
        "  {} - {}",
        to_string_location_i32_bool(error.get_location(), 0, true),
        error.get_message()
      );
    }
    eprintln!();
  }

  // printf("%s", Luau::toJson(parseResult.root, parseResult.commentLocations).c_str());
  let json = unsafe {
    to_json(
      parse_result.root as *mut AstNode,
      parse_result.comment_locations.clone(),
    )
  };
  print!("{}", json);

  // return parseResult.errors.size() > 0 ? 1 : 0;
  if parse_result.errors.is_empty() { 0 } else { 1 }
}
