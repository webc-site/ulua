//! Inline port of the upstream Luau lexer unit tests (`luau/tests/Lexer.test.cpp`,
//! `TEST_SUITE("LexerTests")`). Each C++ `TEST_CASE` becomes a Rust `#[test]`.
//! These are collocated unit tests for the hand-ported lexer; they exercise only
//! the lexer (no Analysis dependency) and run under `cargo test -p luau-ast`.
//!
//! API mapping vs C++:
//! - `Lexer lexer(input, size, table)` -> `Lexer::new(&input[..size], &mut names, Position{0,0})`
//! - `lexeme.type` -> `lexeme.r#type` (a `Type(i32)` newtype; char tokens are `Type(c)`)
//! - `lexeme.location` -> `lexeme.location` (`Location{begin,end}`)
//! - `lexeme.getBlockDepth()` -> `lexeme.get_block_depth()` (translated name)
//! - `lexeme.data`/`getLength()` -> `lexeme.data.data`/`get_length()`
//! - `lexeme.name` -> `lexeme.data.data` (cpp union arm alias of `data`; read via the `AstName` façade)

use core::slice;

use ulua_ast::{
  enums::type_lexer::Type,
  records::{
    allocator::Allocator,
    ast_name::AstName,
    ast_name_table::AstNameTable,
    lexeme::{Lexeme, QuoteStyle},
    lexer::Lexer,
    location::Location,
    position::Position,
  },
};

/// Set up a lexer over `input` (NUL-terminated like `std::string::c_str()`, with
/// `size` excluding the terminator) and run `f`. The backing Buffer and name
/// table outlive the Closure, so token data/name pointers stay valid inside it.
fn with_lexer<R>(input: &[u8], f: impl FnOnce(&mut Lexer) -> R) -> R {
  let mut buf = input.to_vec();
  buf.push(0);
  let size = input.len();
  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let mut lexer = Lexer::new(&buf[..size], &mut names, Position { line: 0, column: 0 });
  f(&mut lexer)
}

fn loc(bl: u32, bc: u32, el: u32, ec: u32) -> Location {
  Location {
    begin: Position {
      line: bl,
      column: bc,
    },
    end: Position {
      line: el,
      column: ec,
    },
  }
}

/// A single-character token's type (`':'`, `'{'`, ...): the char's code point.
fn ch(c: u8) -> Type {
  Type(c as i32)
}

/// The interned name of a `Name`/reserved lexeme (`Lexeme::name` in C++).
fn name_str(lx: &Lexeme) -> String {
  let ptr = lx.data.data;
  // 经 `AstName` 门面读取（null/非 UTF-8 折叠为空串；用例输入均为 ASCII 名）
  AstName::ast_name_u8(ptr).as_str_or_empty().into()
}

/// The raw payload bytes of a data-bearing lexeme (`std::string(lexeme.data,
/// lexeme.getLength())` in C++).
fn data_bytes(lx: &Lexeme) -> Vec<u8> {
  let len = lx.get_length() as usize;
  let ptr = lx.data.data;
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`lexer` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { slice::from_raw_parts(ptr, len) }.to_vec()
}

#[test]
fn broken_string_works() {
  with_lexer(b"[[", |lexer| {
    let lexeme = *lexer.next_lexeme();
    assert_eq!(lexeme.r#type, Type::BROKEN_STRING);
    assert_eq!(lexeme.location, loc(0, 0, 0, 2));
  });
}

#[test]
fn broken_comment() {
  with_lexer(b"--[[  ", |lexer| {
    let lexeme = *lexer.next_lexeme();
    assert_eq!(lexeme.r#type, Type::BROKEN_COMMENT);
    assert_eq!(lexeme.location, loc(0, 0, 0, 6));
  });
}

#[test]
fn broken_comment_kept() {
  with_lexer(b"--[[  ", |lexer| {
    lexer.set_skip_comments(true);
    assert_eq!(lexer.next_lexeme().r#type, Type::BROKEN_COMMENT);
  });
}

#[test]
fn comment_skipped() {
  with_lexer(b"--  ", |lexer| {
    lexer.set_skip_comments(true);
    assert_eq!(lexer.next_lexeme().r#type, Type::EOF);
  });
}

#[test]
fn multiline_comment_with_lexeme_in_and_after() {
  with_lexer(b"--[[ function \n]] end", |lexer| {
    let comment = *lexer.next_lexeme();
    let end = *lexer.next_lexeme();
    assert_eq!(comment.r#type, Type::BLOCK_COMMENT);
    assert_eq!(comment.location, loc(0, 0, 1, 2));
    assert_eq!(end.r#type, Type::RESERVED_END);
    assert_eq!(end.location, loc(1, 3, 1, 6));
  });
}

#[test]
fn test_broken_escape_tolerant() {
  let input = br"'\3729472897292378'";
  with_lexer(input, |lexer| {
    let item = *lexer.next_lexeme();
    assert_eq!(item.r#type, Type::QUOTED_STRING);
    assert_eq!(item.location, loc(0, 0, 0, input.len() as u32));
  });
}

#[test]
fn test_big_delimiters() {
  with_lexer(b"--[===[\n\n\n\n]===]", |lexer| {
    let item = *lexer.next_lexeme();
    assert_eq!(item.r#type, Type::BLOCK_COMMENT);
    assert_eq!(item.location, loc(0, 0, 4, 5));
  });
}

#[test]
fn lookahead() {
  with_lexer(b"foo --[[ comment ]] bar : nil end", |lexer| {
    lexer.set_skip_comments(true);
    lexer.next_lexeme(); // must call next() before reading data at least once

    let cur = *lexer.current();
    assert_eq!(cur.r#type, Type::NAME);
    assert_eq!(name_str(&cur), "foo");
    let la = lexer.lookahead();
    assert_eq!(la.r#type, Type::NAME);
    assert_eq!(name_str(&la), "bar");

    lexer.next_lexeme();
    let cur = *lexer.current();
    assert_eq!(cur.r#type, Type::NAME);
    assert_eq!(name_str(&cur), "bar");
    assert_eq!(lexer.lookahead().r#type, ch(b':'));

    lexer.next_lexeme();
    assert_eq!(lexer.current().r#type, ch(b':'));
    assert_eq!(lexer.lookahead().r#type, Type::RESERVED_NIL);

    lexer.next_lexeme();
    assert_eq!(lexer.current().r#type, Type::RESERVED_NIL);
    assert_eq!(lexer.lookahead().r#type, Type::RESERVED_END);

    lexer.next_lexeme();
    assert_eq!(lexer.current().r#type, Type::RESERVED_END);
    assert_eq!(lexer.lookahead().r#type, Type::EOF);

    lexer.next_lexeme();
    assert_eq!(lexer.current().r#type, Type::EOF);
    assert_eq!(lexer.lookahead().r#type, Type::EOF);
  });
}

#[test]
fn string_interpolation_basic() {
  with_lexer(br#"`foo {"bar"}`"#, |lexer| {
    assert_eq!(lexer.next_lexeme().r#type, Type::INTERP_STRING_BEGIN);
    assert_eq!(lexer.next_lexeme().r#type, Type::QUOTED_STRING);
    let interp_end = *lexer.next_lexeme();
    assert_eq!(interp_end.r#type, Type::INTERP_STRING_END);
    // The InterpStringEnd should start with }, not `.
    assert_eq!(interp_end.location.begin.column, 11);
  });
}

#[test]
fn string_interpolation_full() {
  with_lexer(br#"`foo {"bar"} {"baz"} end`"#, |lexer| {
    let interp_begin = *lexer.next_lexeme();
    assert_eq!(interp_begin.r#type, Type::INTERP_STRING_BEGIN);
    assert_eq!(interp_begin.to_string(), "`foo {");

    let quote1 = *lexer.next_lexeme();
    assert_eq!(quote1.r#type, Type::QUOTED_STRING);
    assert_eq!(quote1.to_string(), "\"bar\"");

    let interp_mid = *lexer.next_lexeme();
    assert_eq!(interp_mid.r#type, Type::INTERP_STRING_MID);
    assert_eq!(interp_mid.to_string(), "} {");
    assert_eq!(interp_mid.location.begin.column, 11);

    let quote2 = *lexer.next_lexeme();
    assert_eq!(quote2.r#type, Type::QUOTED_STRING);
    assert_eq!(quote2.to_string(), "\"baz\"");

    let interp_end = *lexer.next_lexeme();
    assert_eq!(interp_end.r#type, Type::INTERP_STRING_END);
    assert_eq!(interp_end.to_string(), "} end`");
    assert_eq!(interp_end.location.begin.column, 19);
  });
}

#[test]
fn string_interpolation_double_brace() {
  with_lexer(br#"`foo{{bad}}bar`"#, |lexer| {
    let broken_interp_begin = *lexer.next_lexeme();
    assert_eq!(broken_interp_begin.r#type, Type::BROKEN_INTERP_DOUBLE_BRACE);
    assert_eq!(data_bytes(&broken_interp_begin), b"foo");

    assert_eq!(lexer.next_lexeme().r#type, Type::NAME);

    let interp_end = *lexer.next_lexeme();
    assert_eq!(interp_end.r#type, Type::INTERP_STRING_END);
    assert_eq!(data_bytes(&interp_end), b"}bar");
  });
}

#[test]
fn string_interpolation_double_but_unmatched_brace() {
  with_lexer(br#"`{{oops}`, 1"#, |lexer| {
    assert_eq!(lexer.next_lexeme().r#type, Type::BROKEN_INTERP_DOUBLE_BRACE);
    assert_eq!(lexer.next_lexeme().r#type, Type::NAME);
    assert_eq!(lexer.next_lexeme().r#type, Type::INTERP_STRING_END);
    assert_eq!(lexer.next_lexeme().r#type, ch(b','));
    assert_eq!(lexer.next_lexeme().r#type, Type::NUMBER);
  });
}

#[test]
fn string_interpolation_unmatched_brace() {
  let input =
    b"{\n        `hello {\"world\"}\n    } -- this might be incorrectly parsed as a string";
  with_lexer(input, |lexer| {
    assert_eq!(lexer.next_lexeme().r#type, ch(b'{'));
    assert_eq!(lexer.next_lexeme().r#type, Type::INTERP_STRING_BEGIN);
    assert_eq!(lexer.next_lexeme().r#type, Type::QUOTED_STRING);
    assert_eq!(lexer.next_lexeme().r#type, Type::BROKEN_STRING);
    assert_eq!(lexer.next_lexeme().r#type, ch(b'}'));
  });
}

#[test]
fn string_interpolation_with_unicode_escape() {
  with_lexer(br"`\u{1F41B}`", |lexer| {
    assert_eq!(lexer.next_lexeme().r#type, Type::INTERP_STRING_SIMPLE);
    assert_eq!(lexer.next_lexeme().r#type, Type::EOF);
  });
}

#[test]
fn single_quoted_string() {
  with_lexer(b"'test'", |lexer| {
    let lexeme = *lexer.next_lexeme();
    assert_eq!(lexeme.r#type, Type::QUOTED_STRING);
    assert_eq!(lexeme.get_quote_style(), QuoteStyle::Single);
  });
}

#[test]
fn double_quoted_string() {
  with_lexer(b"\"test\"", |lexer| {
    let lexeme = *lexer.next_lexeme();
    assert_eq!(lexeme.r#type, Type::QUOTED_STRING);
    assert_eq!(lexeme.get_quote_style(), QuoteStyle::Double);
  });
}

// cpp Lexer.test.cpp:273-428 —— block depth 组（string ×9 / comment ×3）表驱动
// 收口：每例断言「首个 lexeme 的类型 + 括号层级」。cpp 侧两个 comment 用例与
// string 用例重名（Lexer.test.cpp:406/:418 的 `lexer_determines_string_block_depth_1/_2`），
// 表内按输入区分，不再需要重命名规避。
#[test]
fn lexer_determines_block_depth() {
  // (输入, 期望类型, 期望层级)
  let cases: &[(&[u8], Type, u32)] = &[
    (b"[[ test ]]", Type::RAW_STRING, 0),
    (b"[[ test\n    ]]", Type::RAW_STRING, 0),
    (b"[[\n    test\n    ]]", Type::RAW_STRING, 0),
    (b"[[\n    test ]]", Type::RAW_STRING, 0),
    (b"[=[[%s]]=]", Type::RAW_STRING, 1),
    (b"[==[ test ]==]", Type::RAW_STRING, 2),
    (b"[==[ test\n    ]==]", Type::RAW_STRING, 2),
    (b"[==[\n    test\n    ]==]", Type::RAW_STRING, 2),
    (b"[==[\n\n    test ]==]", Type::RAW_STRING, 2),
    (b"--[[ test ]]", Type::BLOCK_COMMENT, 0),
    // cpp Lexer.test.cpp:406 —— 名字撞 string 组的 comment depth 1 用例，含
    // 非 ASCII 注释体。
    ("--[=[ μέλλον ]=]".as_bytes(), Type::BLOCK_COMMENT, 1),
    // cpp Lexer.test.cpp:418 —— 名字撞 string 组的 comment depth 2 用例。
    (b"--[==[ test ]==]", Type::BLOCK_COMMENT, 2),
  ];
  for &(input, want_ty, want_depth) in cases {
    with_lexer(input, |lexer| {
      let lexeme = *lexer.next_lexeme();
      assert_eq!(lexeme.r#type, want_ty, "输入 {input:?} 的 lexeme 类型");
      assert_eq!(
        lexeme.get_block_depth(),
        want_depth,
        "输入 {input:?} 的层级"
      );
    });
  }
}
