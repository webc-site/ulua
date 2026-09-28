//! Source: `CLI/src/Ast.cpp:24-90` (hand-ported)
use alloc::boxed::Box;
use core::mem::take;

use ulua_analysis::functions::{
  to_json_ast_json_encoder::to_json_with_comment_locations,
  to_string_to_string::to_string_location_i32_bool,
};
use ulua_ast::{
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
  },
  rtti::AstNodePtr,
};
use ulua_cli_lib::functions::{
  cli_preamble::cli_preamble, read_file::read_file, read_stdin::read_stdin,
};

use crate::functions::display_help::display_help;

/// cpp `int main(int argc, char** argv)`
pub fn run(args: &[String]) -> i32 {
  // cpp 的 `char** argv` 按字节使用；`env::args()` 遇非 UTF-8 会 panic，统一由调用方走 lossy argv()。
  // argv0 → 断言处理器 → 默认旗标（含原 `for (flag...) Luau` 前缀启用循环，收敛于
  // `set_luau_flags_default`）→ 顶层 `--help` 检测，均并入共享前奏 `cli_preamble`。
  let preamble = cli_preamble(args, "ulua-ast");
  let argv0 = preamble.argv0;
  let argc = args.len();

  // if (argc >= 2 && strcmp(argv[1], "--help") == 0) { displayHelp(argv[0]); return 0; }
  if preamble.help_requested {
    display_help(argv0);
    return 0;
  }
  // else if (argc < 2) { displayHelp(argv[0]); return 1; }
  else if argc < 2 {
    display_help(argv0);
    return 1;
  }

  // const char* name = argv[1];
  let name = &args[1];

  // cpp: maybeSource = strcmp(name, "-") == 0 ? readStdin() : readFile(name);
  // if (!maybeSource) { fprintf(stderr, "Couldn't read source %s\n", name); return 1; }
  let Some(source) = (if name == "-" {
    read_stdin()
  } else {
    read_file(name)
  }) else {
    eprintln!("Couldn't read source {name}");
    return 1;
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
  // 上转 `*mut AstStatBlock → *mut AstNode` 收口在 `AstNodePtr::as_ast_node`
  // 门面（repr(C) 单继承基址重合由 rtti 内部兑现），调用点零 `as` 裸强转。
  // Safety: `root` 由 Parser 在 parser arena 上分配、非 null 且存活至本帧末
  // （allocator/names 同在栈上）；门面只换类型视图不改地址，被调方契约
  // （`to_json_with_comment_locations` 的 /// # Safety）由此满足。
  let root = parse_result.root.as_ast_node();
  let json = unsafe {
    to_json_with_comment_locations(
      root,
      // cpp 直接 move 走 commentLocations；这里 take 出来免 O(n) 克隆
      take(&mut parse_result.comment_locations),
    )
  };
  print!("{}", json);

  // return parseResult.errors.size() > 0 ? 1 : 0;
  if parse_result.errors.is_empty() { 0 } else { 1 }
}
