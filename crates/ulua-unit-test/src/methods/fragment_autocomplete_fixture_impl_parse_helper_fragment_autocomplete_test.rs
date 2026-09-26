use alloc::{string::String, sync::Arc};

use ulua_analysis::records::source_module::SourceModule;
use ulua_ast::records::{parse_options::ParseOptions, parse_result::ParseResult, parser::Parser};

use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

impl FragmentAutocompleteFixtureImpl {
  pub fn parse_helper_(&mut self, source: &mut SourceModule, document: String) -> ParseResult {
    let parse_options = ParseOptions {
      capture_comments: true,
      ..Default::default()
    };
    let allocator = Arc::get_mut(&mut source.allocator)
      .expect("fresh fragment source allocator must be unique") as *mut _;
    let names =
      Arc::get_mut(&mut source.names).expect("fresh fragment source name table must be unique");
    names.rebind_allocator(allocator);

    let parse_result =
      // Safety: allocator 是 Arc::get_mut 成功（行 14-15 唯一强引用证明）后的块地址，&mut * 重借用把独占权交 Parser::parse（cpp 同形，AST 分配进此 arena、随 allocator 存活）；names 亦经 Arc::get_mut 得独占可变借用；document.as_str 活过本调用。
      unsafe { Parser::parse(document.as_str(), names, &mut *allocator, parse_options) };

    source.parse_errors = parse_result.errors.clone();
    source.root = parse_result.root;
    source.hotcomments = parse_result.hotcomments.clone();
    source.comment_locations = parse_result.comment_locations.clone();

    parse_result
  }
}

impl FragmentAutocompleteFixtureImpl {
  pub fn parse_helper(&mut self, document: String) -> ParseResult {
    let source: *mut SourceModule = self.get_source();
    // Safety: source 为 get_source() 刚注册/复用的 SourceModule*——容器（sources 持有 Box/unique）独占保有，非空存活至本帧；&mut *source 物化临时独占借用，parse_helper_ 填字段后即结束，无第二可变借用。
    unsafe { self.parse_helper_(&mut *source, document) }
  }
}
