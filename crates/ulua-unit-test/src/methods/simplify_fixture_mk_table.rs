//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Method:Luau.UnitTest:tests/Simplify.test.cpp:93:simplify_fixture_mk_table`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//!   - calls <- test simplify_tables (tests/Simplify.test.cpp)
//!   - calls <- test simplify_combine_disjoint_sealed_tables (tests/Simplify.test.cpp)
//!   - calls <- test simplify_non_disjoint_tables_do_not_simplify (tests/Simplify.test.cpp)
//!   - calls <- test simplify_non_disjoint_tables_do_not_simplify_2 (tests/Simplify.test.cpp)
//!   - calls <- test simplify_tables_and_top_table (tests/Simplify.test.cpp)
//!   - calls <- test simplify_tables_and_truthy (tests/Simplify.test.cpp)
//!   - calls <- test simplify_table_with_a_tag (tests/Simplify.test.cpp)
//!   - calls <- test simplify_nested_table_tag_test (tests/Simplify.test.cpp)
//!   - calls <- test simplify_some_tables_are_really_never (tests/Simplify.test.cpp)
//!   - calls <- test simplify_simplify_stops_at_cycles (tests/Simplify.test.cpp)
//!   - calls <- test simplify_x_number_y_number_x_unknown (tests/Simplify.test.cpp)
//!   - calls <- test simplify_x_number_y_number_read_x_unknown (tests/Simplify.test.cpp)
//!   - calls <- test simplify_read_x_child_x_parent (tests/Simplify.test.cpp)
//!   - calls <- test simplify_relate_write_only_number_with_number (tests/Simplify.test.cpp)
//!   - calls <- test simplify_relate_read_only_number_with_number (tests/Simplify.test.cpp)
//!   - calls <- test simplify_relate_coincident_minus_one_prop_tables (tests/Simplify.test.cpp)
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> record SimplifyFixture (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item SimplifyFixture::mkTable

use alloc::vec::Vec;
use std::collections::BTreeMap;

use ulua_analysis::{
  enums::table_state::TableState,
  records::{property_type::Property, table_type::TableType, type_level::TypeLevel},
  type_aliases::{props_type::Props, type_id::TypeId},
};

use crate::records::simplify_fixture::SimplifyFixture;
impl SimplifyFixture {
  pub fn mk_table(&mut self, prop_types: &[(&str, TypeId)]) -> TypeId {
    let prop_types: Vec<_> = prop_types
      .iter()
      .map(|&(name, ty)| (name, Property::rw_type_id(ty)))
      .collect();

    self.mk_table_props(&prop_types)
  }

  pub fn mk_table_props(&mut self, prop_types: &[(&str, Property)]) -> TypeId {
    let mut props: Props = BTreeMap::new();

    for (name, prop) in prop_types {
      props.insert((*name).to_string(), prop.clone());
    }

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
