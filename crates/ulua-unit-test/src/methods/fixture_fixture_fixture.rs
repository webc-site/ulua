use crate::records::fixture::Fixture;

impl Fixture {
  /// C++: `explicit Fixture(bool prepareAutocomplete = false) : forAutocomplete(prepareAutocomplete) {}`
  /// (tests/Fixture.cpp:263). An associated constructor — every other member is
  /// built by `Default`, then `for_autocomplete` is set to the requested value.
  pub fn fixture_bool(prepare_autocomplete: bool) -> Self {
    Self::default().with_for_autocomplete(prepare_autocomplete)
  }

  /// Fixture 实现 Drop，禁用 FRU，改用 builder 方法设置字段
  fn with_for_autocomplete(mut self, for_autocomplete: bool) -> Self {
    self.for_autocomplete = for_autocomplete;
    self
  }
}
