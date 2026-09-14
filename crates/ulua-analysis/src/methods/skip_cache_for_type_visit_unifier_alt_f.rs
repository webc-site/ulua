use std::ptr::eq;

use crate::{
  enums::table_state::TableState,
  functions::get_mutable_type::get_mutable_type_id,
  records::{skip_cache_for_type::SkipCacheForType, table_type::TableType},
  type_aliases::type_id::TypeId,
};
impl SkipCacheForType {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn visit_type_id_table_type(&mut self, ty: TypeId, _tt: &TableType) -> bool {
    unsafe {
      if !eq((*ty).owning_arena, self.type_arena) {
        return false;
      }
      if let Some(ttv) = get_mutable_type_id::<TableType>(ty)
        && (ttv.bound_to.is_some() || ttv.state != TableState::Sealed)
      {
        self.result = true;
        return false;
      }
    }
    true
  }
}
