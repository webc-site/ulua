use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type_function_runtime_alt_n::get_type_function_type_pack_id,
  records::{
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::type_function_type_pack_id::TypeFunctionTypePackId,
};
impl IterativeTypeFunctionTypeVisitor {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn process_type_function_type_pack_id(&mut self, tp: TypeFunctionTypePackId) {
    if self.has_seen(tp as *const c_void) {
      return;
    }

    let tftp = unsafe { get_type_function_type_pack_id::<TypeFunctionTypePack>(tp) };
    let tfvtp = unsafe { get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tp) };
    let tfgtv = unsafe { get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp) };

    if !tftp.is_null() {
      if self.visit_type_function_type_pack_id_type_function_type_pack(tp, unsafe { &*tftp }) {
        let head = unsafe { (*tftp).head.clone() };
        for ty in head {
          self.traverse_type_function_type_id(ty);
        }

        if let Some(tail) = unsafe { (*tftp).tail } {
          self.traverse_type_function_type_pack_id(tail);
        }
      }
    } else if !tfvtp.is_null() {
      if self
        .visit_type_function_type_pack_id_type_function_variadic_type_pack(tp, unsafe { &*tfvtp })
      {
        let inner = unsafe { (*tfvtp).type_id };
        self.traverse_type_function_type_id(inner);
      }
    } else if !tfgtv.is_null() {
      self.visit_type_function_type_pack_id_type_function_generic_type_pack(tp, unsafe { &*tfgtv });
    } else {
      LUAU_ASSERT!(
        false /* "GenericTypeFunctionTypeVisitor::traverse(TypeFunctionTypePackId) is not exhaustive!" */
      );
    }

    self.unsee(tp as *const c_void);
  }
}
