use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  methods::subtyping_bind_generic::dense_hash_map_find_no_default,
  records::{
    apply_mapped_generics::ApplyMappedGenerics, extern_type::ExternType,
    function_type::FunctionType,
  },
  type_aliases::type_id::TypeId,
};
impl ApplyMappedGenerics {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    let env = unsafe { &*self.env };
    if get_type_id::<ExternType>(ty).is_some() {
      return true;
    }
    if let Some(f) = get_type_id::<FunctionType>(ty) {
      for &g in &f.generics {
        let g = follow_type_id(g);
        if let Some(bounds) = dense_hash_map_find_no_default(&env.mapped_generics, &g)
          && !bounds.is_empty()
        {
          return true;
        }
      }
    }
    // SAFETY: ty 指向 TypeArena 内 Type；仅读取 persistent 标志。
    unsafe { (*ty).persistent }
  }
}
