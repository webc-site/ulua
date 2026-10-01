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

use crate::{
  functions::add_sealed_table_type::add_sealed_table_type,
  records::overload_resolver_fixture::OverloadResolverFixture,
};

impl OverloadResolverFixture {
  /// 对应 C++ `OverloadResolverFixture::tableWithCall`（tests/OverloadResolver.test.cpp:82-88）：
  /// 造一个 `{ __call: callMm }` 元表包裹的密封表。
  pub fn table_with_call(&self, call_mm: TypeId) -> TypeId {
    let mut props = Props::new();
    props.insert(String::from("__call"), Property::rw_type_id(call_mm));

    // Safety: `self.arena` 为 `arena_` Box 的稳定非空堆地址（随 fixture 存活）；
    // 本帧取一次独占视图，三笔 add_type 顺序追加密封表/元表/组合（cpp fixture
    // 同款独占 arena 写，见 [`OverloadResolverFixture::arena_view`] 契约）。
    let arena = unsafe { self.arena_view() };
    // null_mut() 对应 cpp 可选 scope 形参（callee 按契约容忍 null 仅字段存储）。
    let table = arena.add_type(TableType::table_type_table_state_type_level_scope(
      TableState::Sealed,
      TypeLevel::default(),
      null_mut(),
    ));
    let metatable = add_sealed_table_type(arena, &props, None);
    arena.add_type(MetatableType::new(table, metatable))
  }
}
