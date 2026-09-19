//! Source: `tests/TypePack.test.cpp`

use alloc::boxed::Box;

use ulua_analysis::records::{
  primitive_type::{PrimitiveType, Type as PrimitiveKind},
  r#type::Type,
};

use crate::records::type_pack_fixture::TypePackFixture;

impl TypePackFixture {
  pub fn new() -> Self {
    let mut fixture = TypePackFixture::default();

    for kind in [
      PrimitiveKind::NilType,
      PrimitiveKind::Boolean,
      PrimitiveKind::Number,
      PrimitiveKind::String,
    ] {
      let ty = Box::new(Type::from(PrimitiveType::primitive_type_type_item(kind)));
      let ty_id = ty.as_ref() as *const Type;
      fixture.type_vars.push(ty);
      fixture.types.push(ty_id);
    }

    fixture
  }
}
