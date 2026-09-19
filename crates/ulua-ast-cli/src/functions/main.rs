//! Source: `CLI/src/Ast.cpp:24-90` (hand-ported)
use alloc::boxed::Box;
use core::mem::take;
use std::process::exit;

use ulua_analysis::functions::{
  to_json_ast_json_encoder_alt_b::to_json, to_string_to_string_alt_t::to_string_location_i32_bool,
};
use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, ast_node::AstNode,
  parse_options::ParseOptions, parser::Parser,
};
use ulua_cli_lib::functions::{
  argv::argv, assertion_handler::install_assertion_handler, read_file::read_file,
  read_stdin::read_stdin, set_luau_flags_default::set_luau_flags_default,
};

use crate::functions::display_help::display_help;
pub fn main() {
  exit(run());
}

fn run() -> i32 {
  // cpp 的 `char** argv` 按字节使用；`env::args()` 遇非 UTF-8 会 panic，统一走 lossy argv()
  let args = argv();
  let argc = args.len();

  // Luau::assertHandler() = assertionHandler;
  install_assertion_handler();

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
  let mut parse_result = Parser::parse(&source, &mut names, &mut allocator, options);

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
      // cpp 直接 move 走 commentLocations；这里 take 出来免 O(n) 克隆
      take(&mut parse_result.comment_locations),
    )
  };
  print!("{}", json);

  // return parseResult.errors.size() > 0 ? 1 : 0;
  if parse_result.errors.is_empty() { 0 } else { 1 }
}
