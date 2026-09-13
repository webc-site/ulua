use ulua_analysis::{
  functions::get_mutable_type::get_mutable_type_id, records::table_type::TableType,
  type_aliases::type_id::TypeId,
};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn cyclic_table<F>(&mut self, cb: F) -> TypeId
  where
    F: FnOnce(&mut SubtypeFixture, TypeId, &mut TableType),
  {
    let ty = self.arena.add_type(TableType::new());
    let table = get_mutable_type_id::<TableType>(ty).expect("expected cyclic table");
    cb(self, ty, table);
    ty
  }
}
