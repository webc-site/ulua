use crate::records::{function_type::FunctionType, type_cloner::TypeCloner};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_function_type(&mut self, t: *mut FunctionType) {
    unsafe {
      for g in &mut (*t).generics {
        *g = self.shallow_clone_type_id(*g);
      }
      for gp in &mut (*t).generic_packs {
        *gp = self.shallow_clone_type_pack_id(*gp);
      }
      (*t).arg_types = self.shallow_clone_type_pack_id((*t).arg_types);
      (*t).ret_types = self.shallow_clone_type_pack_id((*t).ret_types);
    }
  }
}
