use std::sync::Arc;

use crate::{
  records::{
    normalized_type::NormalizedType, type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::type_id::TypeId,
};

impl TypeFunctionReductionGuesser {
  pub fn normalize(&mut self, ty: TypeId) -> Arc<NormalizedType> {
    let normalizer = self.normalizer;
    unsafe { (*normalizer).normalize(ty) }
  }
}
