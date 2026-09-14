use alloc::vec;

use ulua_analysis::{records::property_type::Property, type_aliases::type_id::TypeId};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn read_only_vec2_class(&mut self) -> TypeId {
    let number_ty = self.builtin_types.number_type;
    self.cls_string_extern_type_props(
      "ReadOnlyVec2",
      SubtypeFixture::props(vec![
        ("X", Property::readonly(number_ty)),
        ("Y", Property::readonly(number_ty)),
      ]),
    )
  }
}
