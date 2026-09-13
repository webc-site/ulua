use crate::records::{
  type_function_serializer::TypeFunctionSerializer,
  type_function_union_type::TypeFunctionUnionType, union_type::UnionType,
};

impl TypeFunctionSerializer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn serialize_children_union_type_type_function_union_type(
    &mut self,
    u1: *const UnionType,
    u2: *mut TypeFunctionUnionType,
  ) {
    unsafe {
      let u1 = &*u1;
      let u2 = &mut *u2;

      u2.components.reserve(u1.options.len());
      for &ty in &u1.options {
        u2.components.push(self.shallow_serialize_type_id(ty));
      }
    }
  }
}
