//! @interface-stub
use crate::{
  records::{index_collector::IndexCollector, intersection_type::IntersectionType},
  type_aliases::type_id::TypeId,
};

impl IndexCollector {
  pub fn visit_type_id_intersection_type(&mut self, _ty: TypeId, _it: &IntersectionType) -> bool {
    true
  }
}
