//! Port of `cpp/tests/Lexer.test.cpp`（430 行，28 个 TEST_CASE）。
//!
//! 被测对象：`ulua_ast` 的 `Lexer` / `Lexeme`
//! （对应 C++ `Ast/src/Lexer.cpp`、`Ast/include/Luau/Lexer.h`）。
//!
//! 移植说明：
//! - C++ `Lexer(buffer, size, table)` 的默认 `startPosition` 为 `Position(0,0)`，
//!   Rust `Lexer::new` 无默认参数（buffer 收 `&[u8]` 切片），统一传 `Position::default()`；公共前奏
//!   `Allocator + AstNameTable + Lexer` 折进 `TestLexer`（字段同帧存活，与
//!   C++ 栈对象生命周期语义一致）。
//! - 输入缓冲区：C++ `std::string::c_str` 保证 NUL 结尾；Rust 只传字节切片，
//!   `peekch` 以切片长度判界，无需 NUL 结尾。
//! - C++ `Lexer::next()` 在 Rust 命名为 `next_lexeme()`；`lexeme.type` /
//!   `lexeme.location` 对应字段 `r#type` / `location`。
//! - `lexeme.name` 与 `std::string` 的比较镜像为 `AstName == &str`；
//!   `std::string(lexeme.data, getLength())` 镜像为 `payload()`。
//! - C++ `Lexeme::toString()` 对应 Rust `Display`（`to_string()`）。

use core::slice;

use ulua_ast::{
  enums::type_lexer::Type,
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, lexeme::Lexeme, lexer::Lexer,
    position::Position,
  },
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
    let lexer = Lexer::new(input, &mut names, Position::default());
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
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`names` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { slice::from_raw_parts(lexeme.data.data, lexeme.get_length() as usize) }
}

// Source: `tests/Lexer.test.cpp:13-22`
#[test]
fn broken_string_works() {
  use ulua_ast::{
    enums::type_lexer::Type,
    records::{location::Location, position::Position},
  };

  use crate::TestLexer;

  let mut t = TestLexer::new(b"[[");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::BROKEN_STRING);
  assert_eq!(
    lexeme.location,
    Location::new(Position::new(0, 0), Position::new(0, 2))
  );
}

// Source: `tests/Lexer.test.cpp:24-33`
#[test]
fn broken_comment() {
  use ulua_ast::records::{location::Location, position::Position};

  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"--[[  ");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::BROKEN_COMMENT);
  assert_eq!(
    lexeme.location,
    Location::new(Position::new(0, 0), Position::new(0, 6))
  );
}

// Source: `tests/Lexer.test.cpp:35-43`
//
// `skip_comments` 只跳过合法注释，BrokenComment 仍会产出（与 C++ 一致）。
#[test]
fn broken_comment_kept() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"--[[  ");
  t.lexer.set_skip_comments(true);
  assert_eq!(t.lexer.next_lexeme().r#type, Type::BROKEN_COMMENT);
}

// Source: `tests/Lexer.test.cpp:45-53`
#[test]
fn comment_skipped() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"--  ");
  t.lexer.set_skip_comments(true);
  assert_eq!(t.lexer.next_lexeme().r#type, Type::EOF);
}

// Source: `tests/Lexer.test.cpp:55-69`
#[test]
fn multiline_comment_with_lexeme_in_and_after() {
  use ulua_ast::records::{location::Location, position::Position};

  use crate::{TestLexer, Type};

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

// Source: `tests/Lexer.test.cpp:71-81`
#[test]
fn test_broken_escape_tolerant() {
  use ulua_ast::records::{location::Location, position::Position};

  use crate::{TestLexer, Type};

  let input = r"'\3729472897292378'";
  let mut t = TestLexer::new(input.as_bytes());
  let item = *t.lexer.next_lexeme();

  assert_eq!(item.r#type, Type::QUOTED_STRING);
  assert_eq!(
    item.location,
    Location::new(Position::new(0, 0), Position::new(0, input.len() as u32))
  );
}

// Source: `tests/Lexer.test.cpp:83-97`
#[test]
fn test_big_delimiters() {
  use ulua_ast::records::{location::Location, position::Position};

  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"--[===[\n\n\n\n]===]");
  let item = *t.lexer.next_lexeme();

  assert_eq!(item.r#type, Type::BLOCK_COMMENT);
  assert_eq!(
    item.location,
    Location::new(Position::new(0, 0), Position::new(4, 5))
  );
}

// Source: `tests/Lexer.test.cpp:99-139`
#[test]
fn lookahead() {
  use crate::{TestLexer, Type};

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

// Source: `tests/Lexer.test.cpp:141-158`
#[test]
fn string_interpolation_basic() {
  use crate::{TestLexer, Type};

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

// Source: `tests/Lexer.test.cpp:160-188`
#[test]
fn string_interpolation_full() {
  use crate::{TestLexer, Type};

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

// Source: `tests/Lexer.test.cpp:190-206`
#[test]
fn string_interpolation_double_brace() {
  use crate::{TestLexer, Type, payload};

  let mut t = TestLexer::new(br#"`foo{{bad}}bar`"#);

  let broken_interp_begin = *t.lexer.next_lexeme();
  assert_eq!(broken_interp_begin.r#type, Type::BROKEN_INTERP_DOUBLE_BRACE);
  assert_eq!(payload(&broken_interp_begin), b"foo");

  assert_eq!(t.lexer.next_lexeme().r#type, Type::NAME);

  let interp_end = *t.lexer.next_lexeme();
  assert_eq!(interp_end.r#type, Type::INTERP_STRING_END);
  assert_eq!(payload(&interp_end), b"}bar");
}

// Source: `tests/Lexer.test.cpp:208-220`
#[test]
fn string_interpolation_double_but_unmatched_brace() {
  use crate::{TestLexer, Type};

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

// Source: `tests/Lexer.test.cpp:222-236`
#[test]
fn string_interpolation_unmatched_brace() {
  use crate::{TestLexer, Type};

  let input =
    "{\n        `hello {\"world\"}\n    } -- this might be incorrectly parsed as a string";
  let mut t = TestLexer::new(input.as_bytes());

  assert_eq!(t.lexer.next_lexeme().r#type, Type(b'{' as i32));
  assert_eq!(t.lexer.next_lexeme().r#type, Type::INTERP_STRING_BEGIN);
  assert_eq!(t.lexer.next_lexeme().r#type, Type::QUOTED_STRING);
  assert_eq!(t.lexer.next_lexeme().r#type, Type::BROKEN_STRING);
  assert_eq!(t.lexer.next_lexeme().r#type, Type(b'}' as i32));
}

// Source: `tests/Lexer.test.cpp:238-247`
#[test]
fn string_interpolation_with_unicode_escape() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(br"`\u{1F41B}`");

  assert_eq!(t.lexer.next_lexeme().r#type, Type::INTERP_STRING_SIMPLE);
  assert_eq!(t.lexer.next_lexeme().r#type, Type::EOF);
}

// Source: `tests/Lexer.test.cpp:249-259`
#[test]
fn single_quoted_string() {
  use ulua_ast::records::lexeme::QuoteStyle;

  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"'test'");

  let lexeme = *t.lexer.next_lexeme();
  assert_eq!(lexeme.r#type, Type::QUOTED_STRING);
  assert_eq!(lexeme.get_quote_style(), QuoteStyle::Single);
}

// Source: `tests/Lexer.test.cpp:261-271`
#[test]
fn double_quoted_string() {
  use ulua_ast::records::lexeme::QuoteStyle;

  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(br#""test""#);

  let lexeme = *t.lexer.next_lexeme();
  assert_eq!(lexeme.r#type, Type::QUOTED_STRING);
  assert_eq!(lexeme.get_quote_style(), QuoteStyle::Double);
}

// Source: `tests/Lexer.test.cpp:273-283`
#[test]
fn lexer_determines_string_block_depth_0() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"[[ test ]]");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::RAW_STRING);
  assert_eq!(lexeme.get_block_depth(), 0);
}

// Source: `tests/Lexer.test.cpp:285-297`
#[test]
fn lexer_determines_string_block_depth_0_multiline_1() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"[[ test\n    ]]");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::RAW_STRING);
  assert_eq!(lexeme.get_block_depth(), 0);
}

// Source: `tests/Lexer.test.cpp:299-312`
#[test]
fn lexer_determines_string_block_depth_0_multiline_2() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"[[\n    test\n    ]]");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::RAW_STRING);
  assert_eq!(lexeme.get_block_depth(), 0);
}

// Source: `tests/Lexer.test.cpp:314-326`
#[test]
fn lexer_determines_string_block_depth_0_multiline_3() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"[[\n    test ]]");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::RAW_STRING);
  assert_eq!(lexeme.get_block_depth(), 0);
}

// Source: `tests/Lexer.test.cpp:328-338`
#[test]
fn lexer_determines_string_block_depth_1() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"[=[[%s]]=]");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::RAW_STRING);
  assert_eq!(lexeme.get_block_depth(), 1);
}

// Source: `tests/Lexer.test.cpp:340-350`
#[test]
fn lexer_determines_string_block_depth_2() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"[==[ test ]==]");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::RAW_STRING);
  assert_eq!(lexeme.get_block_depth(), 2);
}

// Source: `tests/Lexer.test.cpp:352-363`
#[test]
fn lexer_determines_string_block_depth_2_multiline_1() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"[==[ test\n    ]==]");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::RAW_STRING);
  assert_eq!(lexeme.get_block_depth(), 2);
}

// Source: `tests/Lexer.test.cpp:365-377`
#[test]
fn lexer_determines_string_block_depth_2_multiline_2() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"[==[\n    test\n    ]==]");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::RAW_STRING);
  assert_eq!(lexeme.get_block_depth(), 2);
}

// Source: `tests/Lexer.test.cpp:379-391`
#[test]
fn lexer_determines_string_block_depth_2_multiline_3() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"[==[\n\n    test ]==]");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::RAW_STRING);
  assert_eq!(lexeme.get_block_depth(), 2);
}

// Source: `tests/Lexer.test.cpp:394-404`
#[test]
fn lexer_determines_comment_block_depth_0() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"--[[ test ]]");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::BLOCK_COMMENT);
  assert_eq!(lexeme.get_block_depth(), 0);
}

// Source: `tests/Lexer.test.cpp:406-416`
//
// C++ 两个用例重名（`lexer_determines_string_block_depth_1`，注释版在
// 406 行）；Rust mod 名不可重复，按实际被测类型改名为 comment 版。
#[test]
fn lexer_determines_comment_block_depth_1() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new("--[=[ μέλλον ]=]".as_bytes());
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::BLOCK_COMMENT);
  assert_eq!(lexeme.get_block_depth(), 1);
}

// Source: `tests/Lexer.test.cpp:418-428`
//
// 同上：C++ 重名用例（注释版在 418 行），Rust 侧改名去重。
#[test]
fn lexer_determines_comment_block_depth_2() {
  use crate::{TestLexer, Type};

  let mut t = TestLexer::new(b"--[==[ test ]==]");
  let lexeme = *t.lexer.next_lexeme();

  assert_eq!(lexeme.r#type, Type::BLOCK_COMMENT);
  assert_eq!(lexeme.get_block_depth(), 2);
}
