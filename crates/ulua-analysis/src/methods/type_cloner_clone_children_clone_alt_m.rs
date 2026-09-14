use alloc::vec::Vec;

use crate::records::{table_type::TableType, type_cloner::TypeCloner};
impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_table_type(&mut self, t: *mut TableType) {
    unsafe {
      if let Some(indexer) = &mut (*t).indexer {
        indexer.index_type = self.shallow_clone_type_id(indexer.index_type);
        indexer.index_result_type = self.shallow_clone_type_id(indexer.index_result_type);
      }

      // `for (auto& [_, p] : t->props) p = shallowClone(p);`
      // Take ownership of each property, clone it, and write it back. We
      // collect the keys first because `shallow_clone_type_id` borrows
      // `self` mutably while we would otherwise hold a borrow on `props`.
      let keys: Vec<_> = (*t).props.keys().cloned().collect();
      for key in keys {
        let p = (*t).props.get(&key).unwrap().clone();
        let cloned = self.shallow_clone_property(&p);
        (*t).props.insert(key, cloned);
      }

      for ty in (*t).instantiated_type_params.iter_mut() {
        *ty = self.shallow_clone_type_id(*ty);
      }

      for tp in (*t).instantiated_type_pack_params.iter_mut() {
        *tp = self.shallow_clone_type_pack_id(*tp);
      }
    }
  }
}
