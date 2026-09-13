use core::ffi::CStr;

use ulua_ast::records::ast_type_pack_generic::AstTypePackGeneric;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  swapped_generic_type_parameter::SwappedGenericTypeParameter,
  type_checker_2::TypeChecker2,
  unknown_symbol::{Context, UnknownSymbol},
};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `tp` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_pack_generic(&mut self, tp: *mut AstTypePackGeneric) {
    let location = unsafe { (*tp).base.base.location };
    let scope_ptr = self.find_innermost_scope(location);
    LUAU_ASSERT!(!scope_ptr.is_null());
    let scope = unsafe { &*scope_ptr };

    let generic_name = unsafe { (*tp).generic_name };
    let name = unsafe {
      CStr::from_ptr(generic_name.value)
        .to_string_lossy()
        .into_owned()
    };

    if scope.lookup_pack(&name).is_some() {
      return;
    }

    if scope.lookup_type(&name).is_some() {
      let kind = SwappedGenericTypeParameter::PACK;
      let error = SwappedGenericTypeParameter { name, kind };
      let location_ref = &location;
      self.report_error_type_error_data_location(error.into(), location_ref);
      return;
    }

    let context = Context::Type;
    let error = UnknownSymbol::new(name.to_string(), context);
    let location_ref = &location;
    self.report_error_type_error_data_location(error.into(), location_ref);
  }
}
