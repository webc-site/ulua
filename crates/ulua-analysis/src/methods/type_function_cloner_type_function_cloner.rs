use alloc::vec::Vec;
use core::ptr::null;

use crate::{
  records::{type_function_cloner::TypeFunctionCloner, type_function_runtime::TypeFunctionRuntime},
  type_aliases::{
    seen_type_packs_type_function_runtime::SeenTypePacks,
    seen_types_type_function_runtime::SeenTypes,
  },
};
impl TypeFunctionCloner {
  pub fn new(runtime: *mut TypeFunctionRuntime) -> Self {
    Self {
      type_function_runtime: runtime,
      queue: Vec::new(),
      types: SeenTypes::new(null()),
      packs: SeenTypePacks::new(null()),
      steps: 0,
    }
  }
}
