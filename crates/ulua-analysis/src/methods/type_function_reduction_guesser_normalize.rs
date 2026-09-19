use std::sync::Arc;

use crate::{
  records::{
    normalized_type::NormalizedType, type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::type_id::TypeId,
};

impl TypeFunctionReductionGuesser {
  /// C++ 中归一化失败返回空 shared_ptr，此处以 None 表达
  pub fn normalize(&mut self, ty: TypeId) -> Option<Arc<NormalizedType>> {
    let normalizer = self.normalizer;
    unsafe { (*normalizer).try_normalize(ty) }
  }
}
