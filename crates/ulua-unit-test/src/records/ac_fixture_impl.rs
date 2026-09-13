use alloc::collections::BTreeMap;
use core::ffi::c_char;

use ulua_ast::records::position::Position;

use crate::records::fixture::Fixture;
#[derive(Debug)]
pub struct AcFixtureImpl {
  pub base: Fixture,
  pub marker_position: BTreeMap<c_char, Position>,
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

pub trait IntoMarker {
  fn into_marker(self) -> c_char;
}

impl IntoMarker for i8 {
  #[inline]
  fn into_marker(self) -> c_char {
    self as c_char
  }
}

impl IntoMarker for u8 {
  #[inline]
  fn into_marker(self) -> c_char {
    self as c_char
  }
}

impl IntoMarker for char {
  #[inline]
  fn into_marker(self) -> c_char {
    self as u8 as c_char
  }
}
