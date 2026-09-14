use alloc::vec::Vec;
use core::ptr::null_mut;

use crate::records::{
  quantifier::Quantifier, type_level::TypeLevel, type_once_visitor::TypeOnceVisitor,
};
impl Quantifier {
  pub fn new(level: TypeLevel) -> Self {
    Quantifier {
      base: TypeOnceVisitor::new("Quantifier".to_string(), false),
      level,
      generics: Vec::new(),
      generic_packs: Vec::new(),
      scope: null_mut(),
      seen_generic_type: false,
      seen_mutable_type: false,
    }
  }
}
