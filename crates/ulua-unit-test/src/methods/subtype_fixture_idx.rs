use ulua_analysis::{records::table_indexer::TableIndexer, type_aliases::type_id::TypeId};

use crate::{
  functions::add_sealed_table_type::add_sealed_table_type, records::subtype_fixture::SubtypeFixture,
};

impl SubtypeFixture {
  pub fn idx(&mut self, key_ty: TypeId, value_ty: TypeId) -> TypeId {
    let indexer = TableIndexer {
      index_type: key_ty,
      index_result_type: value_ty,
      is_read_only: false,
    };
    add_sealed_table_type(&mut self.arena, &Default::default(), Some(indexer))
  }
}
