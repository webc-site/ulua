use ulua_ast::records::{
  ast_stat_block::AstStatBlock, parse_errors::ParseErrors, parse_options::ParseOptions,
  parser::Parser,
};

use crate::records::cfg_fixture::CfgFixture;

impl CfgFixture {
  /// 返回借用绑定到 `&mut self` 的根块引用：AST 内存位于 fixture.allocator，
  /// 借用期内 allocator 不移动/不销毁，调用侧免 unsafe 解引用。
  pub fn parse<'a>(&'a mut self, code: &str) -> &'a AstStatBlock {
    self.names.rebind_allocator(&mut self.allocator as *mut _);

    let result = Parser::parse(
      code,
      code.len(),
      &mut self.names,
      &mut self.allocator,
      ParseOptions::default(),
    );

    if !result.errors.is_empty() {
      panic!("{}", ParseErrors::new(result.errors));
    }

    // SAFETY: root 指向 self.allocator 中存活的根块；生命周期绑定到 `&'a self`。
    unsafe { &*result.root }
  }
}
