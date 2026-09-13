use ulua_analysis::{
  enums::table_state::TableState,
  records::{table_indexer::TableIndexer, table_type::TableType, type_level::TypeLevel},
  type_aliases::{props_type::Props, type_id::TypeId},
};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn tbl_with_indexer(&mut self, props: Props, key_ty: TypeId, value_ty: TypeId) -> TypeId {
    self.arena.add_type(
      TableType::table_type_props_optional_table_indexer_type_level_table_state(
        &props,
        Some(TableIndexer {
          index_type: key_ty,
          index_result_type: value_ty,
          is_read_only: false,
        }),
        TypeLevel::default(),
        TableState::Sealed,
      ),
    )
  }
}
