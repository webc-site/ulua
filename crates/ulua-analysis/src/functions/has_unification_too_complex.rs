//! Source: `Analysis/src/Unifier.cpp:308-319` (hand-ported)
use crate::{
  functions::get_error::get_type_error,
  records::{type_error::TypeError, unification_too_complex::UnificationTooComplex},
  type_aliases::error_vec::ErrorVec,
};

pub fn has_unification_too_complex(errors: &ErrorVec) -> Option<TypeError> {
  errors
    .iter()
    .find(|te| get_type_error::<UnificationTooComplex>(te).is_some())
    .cloned()
}
