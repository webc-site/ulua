use crate::{
  functions::get_error::get_type_error,
  records::{type_error::TypeError, unification_too_complex::UnificationTooComplex},
  type_aliases::error_vec::ErrorVec,
};

pub fn has_unification_too_complex(errors: &ErrorVec) -> Option<TypeError> {
  let mut found: Option<TypeError> = None;

  for te in errors.iter() {
    let unification = get_type_error::<UnificationTooComplex>(te);
    if !unification.is_none() {
      found = Some(te.clone());
      break;
    }
  }

  found
}
