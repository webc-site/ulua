use crate::type_aliases::{l_value::LValue, refinement_map::RefinementMap, type_id::TypeId};

pub fn add_refinement(refis: &mut RefinementMap, lvalue: &LValue, ty: TypeId) {
  refis.insert(lvalue.clone(), ty);
}
