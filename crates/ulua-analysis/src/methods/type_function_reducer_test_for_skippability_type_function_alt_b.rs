use core::ffi::c_void;

use crate::{
  enums::skip_test_result::SkipTestResult,
  functions::{follow_type_pack::follow_type_pack_id, get_type_pack::get_type_pack_id},
  records::{
    generic_type_pack::GenericTypePack,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_function_reducer::TypeFunctionReducer,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl TypeFunctionReducer {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn test_for_skippability_type_pack_id(&self, ty: TypePackId) -> SkipTestResult {
    let ty = unsafe { follow_type_pack_id(ty) };

    if !get_type_pack_id::<TypeFunctionInstanceTypePack>(ty).is_none() {
      if !self.irreducible.contains(&(ty as *const c_void)) {
        return SkipTestResult::Defer;
      } else {
        return SkipTestResult::Irreducible;
      }
    } else if !get_type_pack_id::<GenericTypePack>(ty).is_none() {
      return SkipTestResult::Generic;
    }

    SkipTestResult::Okay
  }
}
