use ulua_analysis::{
  functions::{follow_type, get_type},
  records::intersection_type::IntersectionType,
  type_aliases::type_id::TypeId,
};

use crate::records::simplify_fixture::SimplifyFixture;

impl SimplifyFixture {
  pub fn is_intersection(&mut self, a: TypeId) -> bool {
    let followed = follow_type::follow(a);
    get_type::get::<IntersectionType>(followed).is_some()
  }
}
