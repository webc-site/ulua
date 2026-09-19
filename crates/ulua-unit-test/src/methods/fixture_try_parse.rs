//! Port of `Fixture::tryParse` (tests/Fixture.cpp:378). Unlike `parse`, this does
//! NOT throw/panic on parse errors — error-recovery tests use it to exercise the
//! parser and then inspect `result.errors` themselves. It enables declaration
//! syntax and parses into the fixture's own allocator + name table (which outlive
//! the returned AST for the test's duration).

use ulua_ast::records::{parse_options::ParseOptions, parser::Parser};

use crate::records::{fixture::Fixture, try_parse_result::TryParseResult};

impl Fixture {
  /// 根块引用借用绑定到 `&mut self`；硬错误路径（单条 ParseError unwind 返回
  /// null root）为 None，与 C++ `root == nullptr` 语义对应。
  pub fn try_parse<'a>(
    &'a mut self,
    source: &str,
    parse_options: &ParseOptions,
  ) -> TryParseResult<'a> {
    let mut options: ParseOptions = parse_options.clone();
    options.allow_declaration_syntax = true;

    // See `fixture_parse.rs` — keep the name table pointed at the live allocator.
    self
      .name_table
      .rebind_allocator(&mut self.allocator as *mut _);

    let result = Parser::parse(source, &mut self.name_table, &mut self.allocator, options);

    // SAFETY: 非 null root 指向 self.allocator 中存活的根块；生命周期绑定到
    // `&'a self`，借用期内 allocator 稳定。
    let root = (!result.root.is_null()).then(|| unsafe { &*result.root });

    TryParseResult { result, root }
  }
}
