use crate::records::{
  type_function_cloner::TypeFunctionCloner, type_function_function_type::TypeFunctionFunctionType,
};

impl TypeFunctionCloner {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_type_function_function_type_type_function_function_type(
    &mut self,
    f1: *mut TypeFunctionFunctionType,
    f2: *mut TypeFunctionFunctionType,
  ) {
    let f1_ref = unsafe { &*f1 };
    let f2_ref = unsafe { &mut *f2 };

    for ty in &f1_ref.generics {
      f2_ref
        .generics
        .push(self.shallow_clone_type_function_type_id(*ty));
    }

    for tp in &f1_ref.generic_packs {
      f2_ref
        .generic_packs
        .push(self.shallow_clone_type_function_type_pack_id(*tp));
    }

    f2_ref.arg_types = self.shallow_clone_type_function_type_pack_id(f1_ref.arg_types);
    f2_ref.ret_types = self.shallow_clone_type_function_type_pack_id(f1_ref.ret_types);
    f2_ref.arg_names = f1_ref.arg_names.clone();
  }
}
