//! Source: `tests/Subtyping.test.cpp:117-120`
use ulua_analysis::type_aliases::{props_type::Props, type_id::TypeId};

use crate::{
  functions::add_sealed_table_type::add_sealed_table_type, records::subtype_fixture::SubtypeFixture,
};

impl SubtypeFixture {
  /// C++ `TypeId tbl(TableType::Props&& props)`.
  pub fn tbl(&mut self, props: Props) -> TypeId {
    add_sealed_table_type(&mut self.arena, &props, None)
  }
}
