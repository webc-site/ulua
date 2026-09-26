//! 各测试夹具共用的「Sealed 表类型 + add_type」样板收口：镜像 cpp 夹具中
//! `arena->addType(TableType{props, indexer, TableState::Sealed})`。
use ulua_analysis::{
  enums::table_state::TableState,
  records::{
    table_indexer::TableIndexer, table_type::TableType, type_arena::TypeArena,
    type_level::TypeLevel,
  },
  type_aliases::{props_type::Props, type_id::TypeId},
};

/// 以默认 TypeLevel + Sealed 状态向 `arena` 添加一个表类型。
pub fn add_sealed_table_type(
  arena: &mut TypeArena,
  props: &Props,
  indexer: Option<TableIndexer>,
) -> TypeId {
  arena.add_type(
    TableType::table_type_props_optional_table_indexer_type_level_table_state(
      props,
      indexer,
      TypeLevel::default(),
      TableState::Sealed,
    ),
  )
}
