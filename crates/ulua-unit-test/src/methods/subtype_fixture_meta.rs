use ulua_analysis::{
  records::metatable_type::MetatableType,
  type_aliases::{props_type::Props, type_id::TypeId},
};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn meta(&mut self, meta_props: Props, table_props: Props) -> TypeId {
    let meta_table = self.tbl(table_props);
    let meta_metatable = self.tbl(meta_props);
    self.meta_table_umbrella(meta_metatable, meta_table)
  }

  /// cpp `SubtypeFixture::metaTableUmbrella` 对应物：仅本夹具内部消费，
  /// 降私有可能性收敛 pub API 面（全仓零外部引用，实测于 dev@962c1d5）。
  fn meta_table_umbrella(&mut self, meta: TypeId, table: TypeId) -> TypeId {
    self.arena.add_type(MetatableType::new(table, meta))
  }
}
