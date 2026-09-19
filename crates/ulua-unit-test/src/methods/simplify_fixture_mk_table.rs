//! Source: `tests/Simplify.test.cpp`

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
