//! Node: `cxx:Method:Luau.UnitTest:tests/Subtyping.test.cpp:117:subtype_fixture_tbl`
//! Source: `tests/Subtyping.test.cpp:117-120`
use ulua_analysis::{
  enums::table_state::TableState,
  records::{table_type::TableType, type_level::TypeLevel},
  type_aliases::{props_type::Props, type_id::TypeId},
};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  /// C++ `TypeId tbl(TableType::Props&& props)`.
  pub fn tbl(&mut self, props: Props) -> TypeId {
    self.arena.add_type(
      TableType::table_type_props_optional_table_indexer_type_level_table_state(
        &props,
        None,
        TypeLevel::default(),
        TableState::Sealed,
      ),
    )
  }
}
