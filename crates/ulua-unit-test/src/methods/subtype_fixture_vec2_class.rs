use alloc::vec;

use ulua_analysis::{records::property_type::Property, type_aliases::type_id::TypeId};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn vec2_class(&mut self) -> TypeId {
    let number_ty = self.builtin_types.number_type;
    self.cls_string_extern_type_props(
      "Vec2",
      SubtypeFixture::props(vec![
        ("X", Property::rw_type_id(number_ty)),
        ("Y", Property::rw_type_id(number_ty)),
      ]),
    )
  }
}
