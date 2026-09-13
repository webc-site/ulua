use alloc::string::String;

use ulua_analysis::{
  functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
  type_aliases::type_id::TypeId,
};

use crate::records::generalization_fixture::GeneralizationFixture;

impl GeneralizationFixture {
  pub fn to_string_type_id(&mut self, ty: TypeId) -> String {
    to_string_type_id_to_string_options(ty, &mut self.opts)
  }
}
