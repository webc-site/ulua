use alloc::collections::BTreeMap;

use ulua_ast::records::position::Position;

use crate::records::fixture::Fixture;
#[derive(Debug)]
pub struct AcFixtureImpl {
  pub base: Fixture,
  /// 对齐 cpp `std::map<char, Position> markerPosition`；与
  /// `FragmentAutocompleteFixtureImpl` 一致，marker 以 Rust `char` 存储，
  /// 不再沿用 C 的 `c_char`。
  pub marker_position: BTreeMap<char, Position>,
  pub autocomplete_globals_registered: bool,
  pub register_builtins: bool,
}

impl Default for AcFixtureImpl {
  fn default() -> Self {
    Self {
      base: Fixture::fixture_bool(true),
      marker_position: BTreeMap::new(),
      autocomplete_globals_registered: false,
      register_builtins: false,
    }
  }
}

/// 标记字符入参：测试侧统一传 `'1'` 等 `char` 字面量；`u8` 实现仅供
/// `(b'1' + i)` 这类按字节递增的循环使用。
pub trait IntoMarker {
  fn into_marker(self) -> char;
}

impl IntoMarker for u8 {
  #[inline]
  fn into_marker(self) -> char {
    self as char
  }
}

impl IntoMarker for char {
  #[inline]
  fn into_marker(self) -> char {
    self
  }
}
