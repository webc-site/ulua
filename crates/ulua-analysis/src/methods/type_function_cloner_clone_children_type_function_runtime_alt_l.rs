//! Faithful port of
//! `TypeFunctionCloner::cloneChildren(TypeFunctionTableType* t1, TypeFunctionTableType* t2)`
//! (Analysis/src/TypeFunctionRuntime.cpp:2832-2852).
use alloc::{string::String, vec::Vec};

use crate::{
  records::{
    type_function_cloner::TypeFunctionCloner, type_function_property::TypeFunctionProperty,
    type_function_table_indexer::TypeFunctionTableIndexer,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl TypeFunctionCloner {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_type_function_table_type_type_function_table_type(
    &mut self,
    t1: *mut TypeFunctionTableType,
    t2: *mut TypeFunctionTableType,
  ) {
    unsafe {
      // for (auto& [k, p] : t1->props)
      // Collect first so that `self` is free to be borrowed mutably by
      // shallowClone (C++ iterates t1 while writing into t2).
      let props: Vec<(
        String,
        Option<TypeFunctionTypeId>,
        Option<TypeFunctionTypeId>,
      )> = (*t1)
        .props
        .iter()
        .map(|(k, p)| (k.clone(), p.read_ty, p.write_ty))
        .collect();

      for (k, read_ty_in, write_ty_in) in props {
        // std::optional<TypeFunctionTypeId> readTy;
        // if (p.readTy) readTy = shallowClone(*p.readTy);
        let mut read_ty: Option<TypeFunctionTypeId> = None;
        if let Some(rt) = read_ty_in {
          read_ty = Some(self.shallow_clone_type_function_type_id(rt));
        }

        // std::optional<TypeFunctionTypeId> writeTy;
        // if (p.writeTy) writeTy = shallowClone(*p.writeTy);
        let mut write_ty: Option<TypeFunctionTypeId> = None;
        if let Some(wt) = write_ty_in {
          write_ty = Some(self.shallow_clone_type_function_type_id(wt));
        }

        // t2->props[k] = TypeFunctionProperty{readTy, writeTy};
        (*t2)
          .props
          .insert(k, TypeFunctionProperty { read_ty, write_ty });
      }

      // if (t1->indexer.has_value())
      //     t2->indexer = TypeFunctionTableIndexer(shallowClone(t1->indexer->keyType), shallowClone(t1->indexer->valueType));
      let indexer = (*t1).indexer.clone();
      if let Some(idx) = indexer {
        let key_type = self.shallow_clone_type_function_type_id(idx.key_type);
        let value_type = self.shallow_clone_type_function_type_id(idx.value_type);
        (*t2).indexer = Some(TypeFunctionTableIndexer {
          key_type,
          value_type,
        });
      }

      // if (t1->metatable.has_value())
      //     t2->metatable = shallowClone(*t1->metatable);
      let metatable = (*t1).metatable;
      if let Some(mt) = metatable {
        (*t2).metatable = Some(self.shallow_clone_type_function_type_id(mt));
      }
    }
  }
}
