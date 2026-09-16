use crate::{
  functions::{follow_type::follow_type_id, follow_type_pack::follow_type_pack_id},
  records::{
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// C++ `template<typename T> bool areEquivalent(const T& a, const T& b)`.
pub trait TypeFunctionInstanceEquivalent {
  fn same_function(&self, other: &Self) -> bool;
  fn type_arguments(&self) -> &[TypeId];
  fn pack_arguments(&self) -> &[TypePackId];
}

impl TypeFunctionInstanceEquivalent for TypeFunctionInstanceType {
  fn same_function(&self, other: &Self) -> bool {
    self.function == other.function
  }

  fn type_arguments(&self) -> &[TypeId] {
    &self.type_arguments
  }

  fn pack_arguments(&self) -> &[TypePackId] {
    &self.pack_arguments
  }
}

impl TypeFunctionInstanceEquivalent for TypeFunctionInstanceTypePack {
  fn same_function(&self, other: &Self) -> bool {
    self.function == other.function
  }

  fn type_arguments(&self) -> &[TypeId] {
    &self.type_arguments
  }

  fn pack_arguments(&self) -> &[TypePackId] {
    &self.pack_arguments
  }
}

pub fn are_equivalent<T: TypeFunctionInstanceEquivalent>(a: &T, b: &T) -> bool {
  if !a.same_function(b) {
    return false;
  }

  if a.type_arguments().len() != b.type_arguments().len()
    || a.pack_arguments().len() != b.pack_arguments().len()
  {
    return false;
  }

  for (a_arg, b_arg) in a.type_arguments().iter().zip(b.type_arguments().iter()) {
    if follow_type_id(*a_arg) != follow_type_id(*b_arg) {
      return false;
    }
  }

  for (a_arg, b_arg) in a.pack_arguments().iter().zip(b.pack_arguments().iter()) {
    if unsafe { follow_type_pack_id(*a_arg) } != unsafe { follow_type_pack_id(*b_arg) } {
      return false;
    }
  }

  true
}
