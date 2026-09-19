use alloc::string::String;
use core::ptr::null_mut;

use ulua_analysis::{
  enums::table_state::TableState,
  records::{
    metatable_type::MetatableType, property_type::Property, table_type::TableType,
    type_level::TypeLevel,
  },
  type_aliases::{props_type::Props, type_id::TypeId},
};

use crate::records::overload_resolver_fixture::OverloadResolverFixture;

impl OverloadResolverFixture {
  /// 对应 C++ `OverloadResolverFixture::tableWithCall`（tests/OverloadResolver.test.cpp:82-88）：
  /// 造一个 `{ __call: callMm }` 元表包裹的密封表。
  pub fn table_with_call(&self, call_mm: TypeId) -> TypeId {
    let mut props = Props::new();
    props.insert(String::from("__call"), Property::rw_type_id(call_mm));

    unsafe {
      let table = (*self.arena).add_type(TableType::table_type_table_state_type_level_scope(
        TableState::Sealed,
        TypeLevel::default(),
        null_mut(),
      ));
      let metatable = (*self.arena).add_type(
        TableType::table_type_props_optional_table_indexer_type_level_table_state(
          &props,
          None,
          TypeLevel::default(),
          TableState::Sealed,
        ),
      );
      (*self.arena).add_type(MetatableType::new(table, metatable))
    }
  }
}
