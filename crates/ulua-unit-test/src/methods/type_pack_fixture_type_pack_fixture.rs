//! Node: `cxx:Method:Luau.UnitTest:tests/TypePack.test.cpp:15:type_pack_fixture_type_pack_fixture`
//! Source: `tests/TypePack.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypePack.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypePack.test.cpp
//!   - type_ref <- record TypePackFixture (tests/TypePack.test.cpp)
//! - outgoing:
//!   - type_ref -> record TypePackFixture (tests/TypePack.test.cpp)
//!   - translates_to -> rust_item TypePackFixture::TypePackFixture

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
