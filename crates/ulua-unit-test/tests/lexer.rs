//! Port of `cpp/tests/Lexer.test.cpp`（430 行，28 个 TEST_CASE）。
//!
//! 被测对象：`ulua_ast` 的 `Lexer` / `Lexeme`
//! （对应 C++ `Ast/src/Lexer.cpp`、`Ast/include/Luau/Lexer.h`）。
//!
//! 移植说明：
//! - C++ `Lexer(buffer, size, table)` 的默认 `startPosition` 为 `Position(0,0)`，
//!   Rust `Lexer::new` 无默认参数，统一传 `Position::default()`；公共前奏
//!   `Allocator + AstNameTable + Lexer` 折进 `TestLexer`（字段同帧存活，与
//!   C++ 栈对象生命周期语义一致）。
//! - 输入缓冲区：C++ `std::string::c_str` 保证 NUL 结尾；Rust 只传字节切片，
//!   `peekch` 以 `buffer_size` 判界，无需 NUL 结尾。
//! - C++ `Lexer::next()` 在 Rust 命名为 `next_lexeme()`；`lexeme.type` /
//!   `lexeme.location` 对应字段 `r#type` / `location`。
//! - `lexeme.name` 与 `std::string` 的比较镜像为 `AstName == &str`；
//!   `std::string(lexeme.data, getLength())` 镜像为 `payload()`。
//! - C++ `Lexeme::toString()` 对应 Rust `Display`（`to_string()`）。

use core::{ffi::c_char, slice};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, lexeme::Lexeme, lexer::Lexer,
  position::Position,
};

/// C++ 每个用例的公共前奏
/// `Allocator alloc; AstNameTable table(alloc); Lexer lexer(input.c_str(), size, table);`。
///
/// `AstNameTable` 内部存 `*mut Allocator`、`Lexer` 内部存 `*mut AstNameTable`，
/// 均要求被指对象地址稳定：两者都放 `Box`，结构体再移动也不会悬空（C++ 栈
/// 对象天然地址稳定，Rust 移动语义下的等价做法）。三者同帧存活，与 C++ 的
/// 对象生命周期语义一致。
struct TestLexer {
  _alloc: Box<Allocator>,
  _names: Box<AstNameTable>,
  lexer: Lexer,
}

impl TestLexer {
  fn new(input: &[u8]) -> Self {
    let mut alloc = Box::new(Allocator::new());
    let mut names = Box::new(AstNameTable::new(&mut alloc));
    let lexer = Lexer::new(
      input.as_ptr() as *const c_char,
      input.len(),
      &mut names,
      Position::default(),
    );
    TestLexer {
      _alloc: alloc,
      _names: names,
      lexer,
    }
  }
}

/// `std::string(lexeme.data, lexeme.getLength())` 的等价读取。
///
/// # Safety
/// `lexeme` 的 `data` 指向仍存活的输入缓冲区（`TestLexer` 与输入同帧），
/// `length` 由词法器保证落在缓冲区内。
fn payload(lexeme: &Lexeme) -> &[u8] {
  unsafe { slice::from_raw_parts(lexeme.data.data as *const u8, lexeme.get_length() as usize) }
}

mod broken_string_works {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:13:broken_string_works`
  //! Source: `tests/Lexer.test.cpp:13-22`

  use ulua_ast::records::{lexeme::Type, location::Location, position::Position};

  use super::TestLexer;

  #[test]
  fn broken_string_works() {
    let mut t = TestLexer::new(b"[[");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::BROKEN_STRING);
    assert_eq!(
      lexeme.location,
      Location::new(Position::new(0, 0), Position::new(0, 2))
    );
  }
}

mod broken_comment {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:24:broken_comment`
  //! Source: `tests/Lexer.test.cpp:24-33`

  use ulua_ast::records::{lexeme::Type, location::Location, position::Position};

  use super::TestLexer;

  #[test]
  fn broken_comment() {
    let mut t = TestLexer::new(b"--[[  ");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::BROKEN_COMMENT);
    assert_eq!(
      lexeme.location,
      Location::new(Position::new(0, 0), Position::new(0, 6))
    );
  }
}

mod broken_comment_kept {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:35:broken_comment_kept`
  //! Source: `tests/Lexer.test.cpp:35-43`
  //!
  //! `skip_comments` 只跳过合法注释，BrokenComment 仍会产出（与 C++ 一致）。

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn broken_comment_kept() {
    let mut t = TestLexer::new(b"--[[  ");
    t.lexer.set_skip_comments(true);
    assert_eq!(t.lexer.next_lexeme().r#type, Type::BROKEN_COMMENT);
  }
}

mod comment_skipped {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:45:comment_skipped`
  //! Source: `tests/Lexer.test.cpp:45-53`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn comment_skipped() {
    let mut t = TestLexer::new(b"--  ");
    t.lexer.set_skip_comments(true);
    assert_eq!(t.lexer.next_lexeme().r#type, Type::EOF);
  }
}

mod multiline_comment_with_lexeme_in_and_after {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:55:multilineCommentWithLexemeInAndAfter`
  //! Source: `tests/Lexer.test.cpp:55-69`

  use ulua_ast::records::{lexeme::Type, location::Location, position::Position};

  use super::TestLexer;

  #[test]
  fn multiline_comment_with_lexeme_in_and_after() {
    let mut t = TestLexer::new(b"--[[ function \n]] end");
    let comment = *t.lexer.next_lexeme();
    let end = *t.lexer.next_lexeme();

    assert_eq!(comment.r#type, Type::BLOCK_COMMENT);
    assert_eq!(
      comment.location,
      Location::new(Position::new(0, 0), Position::new(1, 2))
    );
    assert_eq!(end.r#type, Type::RESERVED_END);
    assert_eq!(
      end.location,
      Location::new(Position::new(1, 3), Position::new(1, 6))
    );
  }
}

mod test_broken_escape_tolerant {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:71:testBrokenEscapeTolerant`
  //! Source: `tests/Lexer.test.cpp:71-81`

  use ulua_ast::records::{lexeme::Type, location::Location, position::Position};

  use super::TestLexer;

  #[test]
  fn test_broken_escape_tolerant() {
    let input = r"'\3729472897292378'";
    let mut t = TestLexer::new(input.as_bytes());
    let item = *t.lexer.next_lexeme();

    assert_eq!(item.r#type, Type::QUOTED_STRING);
    assert_eq!(
      item.location,
      Location::new(Position::new(0, 0), Position::new(0, input.len() as u32))
    );
  }
}

mod test_big_delimiters {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:83:testBigDelimiters`
  //! Source: `tests/Lexer.test.cpp:83-97`

  use ulua_ast::records::{lexeme::Type, location::Location, position::Position};

  use super::TestLexer;

  #[test]
  fn test_big_delimiters() {
    let mut t = TestLexer::new(b"--[===[\n\n\n\n]===]");
    let item = *t.lexer.next_lexeme();

    assert_eq!(item.r#type, Type::BLOCK_COMMENT);
    assert_eq!(
      item.location,
      Location::new(Position::new(0, 0), Position::new(4, 5))
    );
  }
}

mod lookahead {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:99:lookahead`
  //! Source: `tests/Lexer.test.cpp:99-139`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lookahead() {
    let mut t = TestLexer::new(b"foo --[[ comment ]] bar : nil end");
    t.lexer.set_skip_comments(true);
    t.lexer.next_lexeme(); // must call next() before reading data from lexer at least once

    assert_eq!(t.lexer.current().r#type, Type::NAME);
    assert_eq!(t.lexer.current().name(), "foo");
    assert_eq!(t.lexer.lookahead().r#type, Type::NAME);
    assert_eq!(t.lexer.lookahead().name(), "bar");

    t.lexer.next_lexeme();

    assert_eq!(t.lexer.current().r#type, Type::NAME);
    assert_eq!(t.lexer.current().name(), "bar");
    assert_eq!(t.lexer.lookahead().r#type, Type::COLON);

    t.lexer.next_lexeme();

    assert_eq!(t.lexer.current().r#type, Type::COLON);
    assert_eq!(t.lexer.lookahead().r#type, Type::RESERVED_NIL);

    t.lexer.next_lexeme();

    assert_eq!(t.lexer.current().r#type, Type::RESERVED_NIL);
    assert_eq!(t.lexer.lookahead().r#type, Type::RESERVED_END);

    t.lexer.next_lexeme();

    assert_eq!(t.lexer.current().r#type, Type::RESERVED_END);
    assert_eq!(t.lexer.lookahead().r#type, Type::EOF);

    t.lexer.next_lexeme();

    assert_eq!(t.lexer.current().r#type, Type::EOF);
    assert_eq!(t.lexer.lookahead().r#type, Type::EOF);
  }
}

mod string_interpolation_basic {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:141:string_interpolation_basic`
  //! Source: `tests/Lexer.test.cpp:141-158`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn string_interpolation_basic() {
    let mut t = TestLexer::new(br#"`foo {"bar"}`"#);

    let interp_begin = *t.lexer.next_lexeme();
    assert_eq!(interp_begin.r#type, Type::INTERP_STRING_BEGIN);

    let quote = *t.lexer.next_lexeme();
    assert_eq!(quote.r#type, Type::QUOTED_STRING);

    let interp_end = *t.lexer.next_lexeme();
    assert_eq!(interp_end.r#type, Type::INTERP_STRING_END);
    // The InterpStringEnd should start with }, not `.
    assert_eq!(interp_end.location.begin.column, 11);
  }
}

mod string_interpolation_full {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:160:string_interpolation_full`
  //! Source: `tests/Lexer.test.cpp:160-188`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn string_interpolation_full() {
    let mut t = TestLexer::new(br#"`foo {"bar"} {"baz"} end`"#);

    let interp_begin = *t.lexer.next_lexeme();
    assert_eq!(interp_begin.r#type, Type::INTERP_STRING_BEGIN);
    assert_eq!(interp_begin.to_string(), "`foo {");

    let quote1 = *t.lexer.next_lexeme();
    assert_eq!(quote1.r#type, Type::QUOTED_STRING);
    assert_eq!(quote1.to_string(), "\"bar\"");

    let interp_mid = *t.lexer.next_lexeme();
    assert_eq!(interp_mid.r#type, Type::INTERP_STRING_MID);
    assert_eq!(interp_mid.to_string(), "} {");
    assert_eq!(interp_mid.location.begin.column, 11);

    let quote2 = *t.lexer.next_lexeme();
    assert_eq!(quote2.r#type, Type::QUOTED_STRING);
    assert_eq!(quote2.to_string(), "\"baz\"");

    let interp_end = *t.lexer.next_lexeme();
    assert_eq!(interp_end.r#type, Type::INTERP_STRING_END);
    assert_eq!(interp_end.to_string(), "} end`");
    assert_eq!(interp_end.location.begin.column, 19);
  }
}

mod string_interpolation_double_brace {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:190:string_interpolation_double_brace`
  //! Source: `tests/Lexer.test.cpp:190-206`

  use ulua_ast::records::lexeme::Type;

  use super::{TestLexer, payload};

  #[test]
  fn string_interpolation_double_brace() {
    let mut t = TestLexer::new(br#"`foo{{bad}}bar`"#);

    let broken_interp_begin = *t.lexer.next_lexeme();
    assert_eq!(broken_interp_begin.r#type, Type::BROKEN_INTERP_DOUBLE_BRACE);
    assert_eq!(payload(&broken_interp_begin), b"foo");

    assert_eq!(t.lexer.next_lexeme().r#type, Type::NAME);

    let interp_end = *t.lexer.next_lexeme();
    assert_eq!(interp_end.r#type, Type::INTERP_STRING_END);
    assert_eq!(payload(&interp_end), b"}bar");
  }
}

mod string_interpolation_double_but_unmatched_brace {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:208:string_interpolation_double_but_unmatched_brace`
  //! Source: `tests/Lexer.test.cpp:208-220`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn string_interpolation_double_but_unmatched_brace() {
    let mut t = TestLexer::new(br#"`{{oops}`, 1"#);

    assert_eq!(
      t.lexer.next_lexeme().r#type,
      Type::BROKEN_INTERP_DOUBLE_BRACE
    );
    assert_eq!(t.lexer.next_lexeme().r#type, Type::NAME);
    assert_eq!(t.lexer.next_lexeme().r#type, Type::INTERP_STRING_END);
    assert_eq!(t.lexer.next_lexeme().r#type, Type::COMMA);
    assert_eq!(t.lexer.next_lexeme().r#type, Type::NUMBER);
  }
}

mod string_interpolation_unmatched_brace {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:222:string_interpolation_unmatched_brace`
  //! Source: `tests/Lexer.test.cpp:222-236`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn string_interpolation_unmatched_brace() {
    let input =
      "{\n        `hello {\"world\"}\n    } -- this might be incorrectly parsed as a string";
    let mut t = TestLexer::new(input.as_bytes());

    assert_eq!(t.lexer.next_lexeme().r#type, Type(b'{' as i32));
    assert_eq!(t.lexer.next_lexeme().r#type, Type::INTERP_STRING_BEGIN);
    assert_eq!(t.lexer.next_lexeme().r#type, Type::QUOTED_STRING);
    assert_eq!(t.lexer.next_lexeme().r#type, Type::BROKEN_STRING);
    assert_eq!(t.lexer.next_lexeme().r#type, Type(b'}' as i32));
  }
}

mod string_interpolation_with_unicode_escape {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:238:string_interpolation_with_unicode_escape`
  //! Source: `tests/Lexer.test.cpp:238-247`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn string_interpolation_with_unicode_escape() {
    let mut t = TestLexer::new(br"`\u{1F41B}`");

    assert_eq!(t.lexer.next_lexeme().r#type, Type::INTERP_STRING_SIMPLE);
    assert_eq!(t.lexer.next_lexeme().r#type, Type::EOF);
  }
}

mod single_quoted_string {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:249:single_quoted_string`
  //! Source: `tests/Lexer.test.cpp:249-259`

  use ulua_ast::records::lexeme::{QuoteStyle, Type};

  use super::TestLexer;

  #[test]
  fn single_quoted_string() {
    let mut t = TestLexer::new(b"'test'");

    let lexeme = *t.lexer.next_lexeme();
    assert_eq!(lexeme.r#type, Type::QUOTED_STRING);
    assert_eq!(lexeme.get_quote_style(), QuoteStyle::Single);
  }
}

mod double_quoted_string {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:261:double_quoted_string`
  //! Source: `tests/Lexer.test.cpp:261-271`

  use ulua_ast::records::lexeme::{QuoteStyle, Type};

  use super::TestLexer;

  #[test]
  fn double_quoted_string() {
    let mut t = TestLexer::new(br#""test""#);

    let lexeme = *t.lexer.next_lexeme();
    assert_eq!(lexeme.r#type, Type::QUOTED_STRING);
    assert_eq!(lexeme.get_quote_style(), QuoteStyle::Double);
  }
}

mod lexer_determines_string_block_depth_0 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:273:lexer_determines_string_block_depth_0`
  //! Source: `tests/Lexer.test.cpp:273-283`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lexer_determines_string_block_depth_0() {
    let mut t = TestLexer::new(b"[[ test ]]");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::RAW_STRING);
    assert_eq!(lexeme.get_block_depth(), 0);
  }
}

mod lexer_determines_string_block_depth_0_multiline_1 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:285:lexer_determines_string_block_depth_0_multiline_1`
  //! Source: `tests/Lexer.test.cpp:285-297`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lexer_determines_string_block_depth_0_multiline_1() {
    let mut t = TestLexer::new(b"[[ test\n    ]]");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::RAW_STRING);
    assert_eq!(lexeme.get_block_depth(), 0);
  }
}

mod lexer_determines_string_block_depth_0_multiline_2 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:299:lexer_determines_string_block_depth_0_multiline_2`
  //! Source: `tests/Lexer.test.cpp:299-312`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lexer_determines_string_block_depth_0_multiline_2() {
    let mut t = TestLexer::new(b"[[\n    test\n    ]]");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::RAW_STRING);
    assert_eq!(lexeme.get_block_depth(), 0);
  }
}

mod lexer_determines_string_block_depth_0_multiline_3 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:314:lexer_determines_string_block_depth_0_multiline_3`
  //! Source: `tests/Lexer.test.cpp:314-326`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lexer_determines_string_block_depth_0_multiline_3() {
    let mut t = TestLexer::new(b"[[\n    test ]]");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::RAW_STRING);
    assert_eq!(lexeme.get_block_depth(), 0);
  }
}

mod lexer_determines_string_block_depth_1 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:328:lexer_determines_string_block_depth_1`
  //! Source: `tests/Lexer.test.cpp:328-338`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lexer_determines_string_block_depth_1() {
    let mut t = TestLexer::new(b"[=[[%s]]=]");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::RAW_STRING);
    assert_eq!(lexeme.get_block_depth(), 1);
  }
}

mod lexer_determines_string_block_depth_2 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:340:lexer_determines_string_block_depth_2`
  //! Source: `tests/Lexer.test.cpp:340-350`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lexer_determines_string_block_depth_2() {
    let mut t = TestLexer::new(b"[==[ test ]==]");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::RAW_STRING);
    assert_eq!(lexeme.get_block_depth(), 2);
  }
}

mod lexer_determines_string_block_depth_2_multiline_1 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:352:lexer_determines_string_block_depth_2_multiline_1`
  //! Source: `tests/Lexer.test.cpp:352-363`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lexer_determines_string_block_depth_2_multiline_1() {
    let mut t = TestLexer::new(b"[==[ test\n    ]==]");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::RAW_STRING);
    assert_eq!(lexeme.get_block_depth(), 2);
  }
}

mod lexer_determines_string_block_depth_2_multiline_2 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:365:lexer_determines_string_block_depth_2_multiline_2`
  //! Source: `tests/Lexer.test.cpp:365-377`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lexer_determines_string_block_depth_2_multiline_2() {
    let mut t = TestLexer::new(b"[==[\n    test\n    ]==]");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::RAW_STRING);
    assert_eq!(lexeme.get_block_depth(), 2);
  }
}

mod lexer_determines_string_block_depth_2_multiline_3 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:379:lexer_determines_string_block_depth_2_multiline_3`
  //! Source: `tests/Lexer.test.cpp:379-391`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lexer_determines_string_block_depth_2_multiline_3() {
    let mut t = TestLexer::new(b"[==[\n\n    test ]==]");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::RAW_STRING);
    assert_eq!(lexeme.get_block_depth(), 2);
  }
}

mod lexer_determines_comment_block_depth_0 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:394:lexer_determines_comment_block_depth_0`
  //! Source: `tests/Lexer.test.cpp:394-404`

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lexer_determines_comment_block_depth_0() {
    let mut t = TestLexer::new(b"--[[ test ]]");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::BLOCK_COMMENT);
    assert_eq!(lexeme.get_block_depth(), 0);
  }
}

mod lexer_determines_comment_block_depth_1 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:406:lexer_determines_string_block_depth_1`
  //! Source: `tests/Lexer.test.cpp:406-416`
  //!
  //! C++ 两个用例重名（`lexer_determines_string_block_depth_1`，注释版在
  //! 406 行）；Rust mod 名不可重复，按实际被测类型改名为 comment 版。

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lexer_determines_comment_block_depth_1() {
    let mut t = TestLexer::new("--[=[ μέλλον ]=]".as_bytes());
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::BLOCK_COMMENT);
    assert_eq!(lexeme.get_block_depth(), 1);
  }
}

mod lexer_determines_comment_block_depth_2 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Lexer.test.cpp:418:lexer_determines_string_block_depth_2`
  //! Source: `tests/Lexer.test.cpp:418-428`
  //!
  //! 同上：C++ 重名用例（注释版在 418 行），Rust 侧改名去重。

  use ulua_ast::records::lexeme::Type;

  use super::TestLexer;

  #[test]
  fn lexer_determines_comment_block_depth_2() {
    let mut t = TestLexer::new(b"--[==[ test ]==]");
    let lexeme = *t.lexer.next_lexeme();

    assert_eq!(lexeme.r#type, Type::BLOCK_COMMENT);
    assert_eq!(lexeme.get_block_depth(), 2);
  }
}
