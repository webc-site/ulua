use ulua_analysis::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::intersection_type::IntersectionType,
  type_aliases::type_id::TypeId,
};

use crate::records::simplify_fixture::SimplifyFixture;

impl SimplifyFixture {
  pub fn is_intersection(&mut self, a: TypeId) -> bool {
    let followed = follow_type_id(a);
    get_type_id::<IntersectionType>(followed).is_some()
  }
}
