use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{negation_type::NegationType, type_simplifier::TypeSimplifier},
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn mk_negation(&self, ty: TypeId) -> TypeId {
    let builtin_types = unsafe { &*self.builtin_types };
    let arena = unsafe { &mut *self.arena.cast_mut() };
    if ty == builtin_types.truthy_type {
      builtin_types.falsy_type
    } else if ty == builtin_types.falsy_type {
      builtin_types.truthy_type
    } else if let Some(ntv) = get_type_id::<NegationType>(ty) {
      follow_type_id(ntv.ty)
    } else {
      arena.add_type(NegationType { ty })
    }
  }
}
