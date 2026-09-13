use alloc::vec::Vec;

use ulua_common::FFlag;

use crate::{
  records::{extern_type::ExternType, type_cloner::TypeCloner},
  type_aliases::nominal_relation::NominalRelation,
};
impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_extern_type(&mut self, t: *mut ExternType) {
    unsafe {
      // `for (auto& [_, p] : t->props) p = shallowClone(p);`
      let keys: Vec<_> = (*t).props.keys().cloned().collect();
      for key in keys {
        let p = (*t).props.get(&key).unwrap().clone();
        let cloned = self.shallow_clone_property(&p);
        (*t).props.insert(key, cloned);
      }

      if let Some(parent) = (*t).parent {
        (*t).parent = Some(self.shallow_clone_type_id(parent));
      }

      if let Some(metatable) = (*t).metatable {
        (*t).metatable = Some(self.shallow_clone_type_id(metatable));
      }

      if let Some(indexer) = &mut (*t).indexer {
        indexer.index_type = self.shallow_clone_type_id(indexer.index_type);
        indexer.index_result_type = self.shallow_clone_type_id(indexer.index_result_type);
      }

      if FFlag::DebugLuauUserDefinedClasses.get()
        && let Some(relation) = &mut (*t).relation
      {
        match relation {
          NominalRelation::V0(obj) => {
            obj.ty = self.shallow_clone_type_id(obj.ty);
          }
          NominalRelation::V1(klass) => {
            klass.ty = self.shallow_clone_type_id(klass.ty);
          }
        }
      }
    }
  }
}
