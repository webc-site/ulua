use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    function_type::FunctionType, type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::type_id::TypeId,
};

impl TypeFunctionReductionGuesser {
  pub fn is_function_generics_saturated(
    &self,
    ftv: &FunctionType,
    args_used: &mut DenseHashSet<TypeId>,
  ) -> bool {
    let same_size = ftv.generics.len() == args_used.size();
    let mut all_generics_appear = true;
    for &gt in &ftv.generics {
      all_generics_appear = all_generics_appear && args_used.contains(&gt);
    }
    same_size && all_generics_appear
  }
}
