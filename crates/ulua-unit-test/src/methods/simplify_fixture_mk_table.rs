//! Source: `tests/Simplify.test.cpp`

use alloc::vec::Vec;
use std::collections::BTreeMap;

use ulua_analysis::{
  records::property_type::Property,
  type_aliases::{props_type::Props, type_id::TypeId},
};

use crate::{
  functions::add_sealed_table_type::add_sealed_table_type,
  records::simplify_fixture::SimplifyFixture,
};
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

    add_sealed_table_type(&mut self.arena, &props, None)
  }
}
