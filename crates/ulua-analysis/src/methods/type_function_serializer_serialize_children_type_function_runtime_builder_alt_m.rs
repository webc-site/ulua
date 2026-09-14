use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    metatable_type::MetatableType, table_type::TableType,
    type_function_serializer::TypeFunctionSerializer,
    type_function_table_type::TypeFunctionTableType,
  },
};

impl TypeFunctionSerializer {
  pub fn serialize_children_metatable_type_type_function_table_type(
    &mut self,
    m1: &MetatableType,
    m2: &mut TypeFunctionTableType,
  ) {
    let table = follow_type_id(m1.table);

    if let Some(table) = get_type_id::<TableType>(table) {
      // &T/&mut T 隐式转清单外旧签名的裸指针参数
      unsafe { self.serialize_children_table_type_type_function_table_type(table, m2) };
    }

    m2.metatable = Some(self.shallow_serialize_type_id(m1.metatable));
  }
}
