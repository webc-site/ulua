//! 共享实现：cpp `getReadIndexer`/`getWriteIndexer` 中 table/extern 两分支
//! 完全一致的「把 indexer 推成 {index, result} 表（或 nil）」逻辑抽此一处。

/// # Safety
/// 调用方须保证 `l`、`l.as_mut_ptr()` 裸指针有效，满足 C++ 原实现的调用契约。
use crate::{functions::{alloc_type_user_data::alloc_type_user_data, lua_names::{FIELD_INDEX, FIELD_RESULT}}, records::type_function_table_indexer::TypeFunctionTableIndexer};
use ulua_vm::records::lua_state::LuaState;
pub(crate) unsafe fn push_table_indexer(
  l: &mut LuaState,
  indexer: &Option<TypeFunctionTableIndexer>,
) {
  unsafe {
    match indexer {
      None => l.push_nil(),
      Some(indexer) => {
        l.create_table(0, 2);
        alloc_type_user_data(&mut *l, (*indexer.key_type).type_variant.clone(), false);
        l.set_field_bytes(-2, FIELD_INDEX);
        alloc_type_user_data(&mut *l, (*indexer.value_type).type_variant.clone(), false);
        l.set_field_bytes(-2, FIELD_RESULT);
      }
    }
  }
}
