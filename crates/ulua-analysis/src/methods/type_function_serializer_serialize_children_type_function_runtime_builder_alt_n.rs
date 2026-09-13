use ulua_common::FFlag;

use crate::records::{
  function_type::FunctionType, type_function_function_type::TypeFunctionFunctionType,
  type_function_serializer::TypeFunctionSerializer,
};
impl TypeFunctionSerializer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn serialize_children_function_type_type_function_function_type(
    &mut self,
    f1: *const FunctionType,
    f2: *mut TypeFunctionFunctionType,
  ) {
    unsafe {
      let f1 = &*f1;
      let f2 = &mut *f2;
      f2.generics.reserve(f1.generics.len());
      for &ty in &f1.generics {
        let t = self.shallow_serialize_type_id(ty);
        f2.generics.push(t);
      }
      f2.generic_packs.reserve(f1.generic_packs.len());
      for &tp in &f1.generic_packs {
        let t = self.shallow_serialize_type_pack_id(tp);
        f2.generic_packs.push(t);
      }
      f2.arg_types = self.shallow_serialize_type_pack_id(f1.arg_types);
      f2.ret_types = self.shallow_serialize_type_pack_id(f1.ret_types);

      if FFlag::LuauTypeFunctionSerializeArgNames.get() {
        f2.arg_names.reserve(f1.arg_names.len());
        for arg_name in &f1.arg_names {
          if let Some(arg_name) = arg_name {
            f2.arg_names.push(Some(arg_name.name.clone()));
          } else {
            f2.arg_names.push(None);
          }
        }
      }
    }
  }
}
