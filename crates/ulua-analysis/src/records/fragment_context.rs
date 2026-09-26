use alloc::string::String;

use ulua_ast::records::{parse_result::ParseResult, position::Position};

use crate::records::{
  frontend_options::FrontendOptions, i_fragment_autocomplete_reporter::ReporterRef,
};
#[derive(Debug, Clone)]
pub struct FragmentContext<'a> {
  pub(crate) new_src: String,
  pub(crate) fresh_parse: &'a ParseResult,
  pub(crate) opts: Option<FrontendOptions>,
  pub(crate) deprecated_fragment_end_position: Option<Position>,
  pub(crate) reporter: ReporterRef<'a>,
}

impl<'a> FragmentContext<'a> {
  pub fn new(new_src: &str, fresh_parse: &'a ParseResult) -> Self {
    Self {
      new_src: String::from(new_src),
      fresh_parse,
      opts: None,
      deprecated_fragment_end_position: None,
      reporter: ReporterRef::NULL,
    }
  }

  /// C++ aggregate `FragmentContext{new_src, fresh_parse, opts, fragmentEndPosition}`.
  pub fn new_with_options(
    new_src: &str,
    fresh_parse: &'a ParseResult,
    opts: Option<FrontendOptions>,
    fragment_end_position: Option<Position>,
  ) -> Self {
    Self {
      new_src: String::from(new_src),
      fresh_parse,
      opts,
      deprecated_fragment_end_position: fragment_end_position,
      reporter: ReporterRef::NULL,
    }
  }

  pub fn new_src(&self) -> &str {
    &self.new_src
  }

  pub fn fresh_parse(&self) -> &ParseResult {
    self.fresh_parse
  }

  pub fn opts(&self) -> Option<&FrontendOptions> {
    self.opts.as_ref()
  }

  pub fn deprecated_fragment_end_position(&self) -> Option<Position> {
    self.deprecated_fragment_end_position
  }

  pub fn reporter(&self) -> ReporterRef<'a> {
    self.reporter
  }
}
