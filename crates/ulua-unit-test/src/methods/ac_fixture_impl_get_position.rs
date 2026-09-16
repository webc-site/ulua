use ulua_ast::records::position::Position;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::ac_fixture_impl::{AcFixtureImpl, IntoMarker};

impl AcFixtureImpl {
  pub fn get_position(&self, marker: impl IntoMarker) -> &Position {
    let marker = marker.into_marker();
    let i = self.marker_position.get(&marker);
    LUAU_ASSERT!(i.is_some());
    match i {
      Some(pos) => pos,
      None => {
        // Safety: The C++ implementation asserts that the marker must exist.
        // In the event of an assertion failure in a non-debug build, we return a static missing position
        // to satisfy the return type while maintaining the contract that this should not be reached.
        static MISSING: Position = Position { line: 0, column: 0 };
        &MISSING
      }
    }
  }
}
