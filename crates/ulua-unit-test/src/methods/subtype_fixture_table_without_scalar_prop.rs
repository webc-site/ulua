use alloc::vec;

use ulua_analysis::{records::property_type::Property, type_aliases::type_id::TypeId};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn table_without_scalar_prop(&mut self) -> TypeId {
    let no_args_no_returns =
      self.fn_item_initializer_list_type_id_initializer_list_type_id(vec![], vec![]);

    self.tbl(SubtypeFixture::props(vec![(
      "insaneThingNoScalarHas",
      Property::rw_type_id(no_args_no_returns),
    )]))
  }
}
