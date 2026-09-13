use alloc::string::String;

use ulua_analysis::{
  functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
  type_aliases::type_id::TypeId,
};

use crate::records::simplify_fixture::SimplifyFixture;

impl SimplifyFixture {
  pub fn intersect_str(&mut self, a: TypeId, b: TypeId) -> String {
    let ty = self.intersect(a, b);
    to_string_type_id_to_string_options(ty, &mut self.opts)
  }
}
