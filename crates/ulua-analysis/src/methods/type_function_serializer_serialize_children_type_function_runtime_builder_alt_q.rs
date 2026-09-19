use crate::records::{
  type_function_serializer::TypeFunctionSerializer, type_function_type_pack::TypeFunctionTypePack,
  type_pack::TypePack,
};

impl TypeFunctionSerializer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn serialize_children_type_pack_type_function_type_pack(
    &mut self,
    t1: *const TypePack,
    t2: *mut TypeFunctionTypePack,
  ) {
    unsafe {
      let t1 = &*t1;
      let t2 = &mut *t2;
      for &ty in &t1.head {
        t2.head.push(self.shallow_serialize_type_id(ty));
      }
      if let Some(tail) = t1.tail {
        t2.tail = Some(self.shallow_serialize_type_pack_id(tail));
      }
    }
  }
}
