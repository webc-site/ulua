use crate::records::{
  table_type::TableType, type_function_property::TypeFunctionProperty,
  type_function_serializer::TypeFunctionSerializer,
  type_function_table_indexer::TypeFunctionTableIndexer,
  type_function_table_type::TypeFunctionTableType,
};

impl TypeFunctionSerializer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn serialize_children_table_type_type_function_table_type(
    &mut self,
    t1: *const TableType,
    t2: *mut TypeFunctionTableType,
  ) {
    unsafe {
      let t1 = &*t1;
      let t2 = &mut *t2;

      for (name, prop) in &t1.props {
        let read_ty = prop
          .read_ty
          .map(|read_ty| self.shallow_serialize_type_id(read_ty));
        let write_ty = prop
          .write_ty
          .map(|write_ty| self.shallow_serialize_type_id(write_ty));

        t2.props
          .insert(name.clone(), TypeFunctionProperty { read_ty, write_ty });
      }

      if let Some(indexer) = &t1.indexer {
        let key_type = self.shallow_serialize_type_id(indexer.index_type);
        let value_type = self.shallow_serialize_type_id(indexer.index_result_type);
        t2.indexer = Some(TypeFunctionTableIndexer::new(key_type, value_type));
      }
    }
  }
}
