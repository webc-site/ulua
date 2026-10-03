//! 共享实现：cpp `getReadIndexer`/`getWriteIndexer` 中 table/extern 两分支
//! 完全一致的「把 indexer 推成 {index, result} 表（或 nil）」逻辑抽此一处。

use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    lua_names::{FIELD_INDEX, FIELD_RESULT},
  },
  records::type_function_table_indexer::TypeFunctionTableIndexer,
};

/// # Safety
/// `l` 须为存活且本次调用独占的 `LuaState`；`indexer` 内各 `TypeFunctionTypeId`（裸指针）
/// 若非空须指向 type_arena 中存活的 `TypeFunctionType` 节点，其 `type_variant` 字段在
/// 本调用期内有效。`indexer` 为 `None` 时仅推 nil，无此要求。
pub(crate) unsafe fn push_table_indexer(
  l: &mut LuaState,
  indexer: &Option<TypeFunctionTableIndexer>,
) {
  unsafe {
    match indexer {
      None => l.push_nil(),
      Some(indexer) => {
        l.create_table(0, 2);
        alloc_type_user_data(l, (*indexer.key_type).type_variant.clone(), false);
        l.set_field_bytes(-2, FIELD_INDEX);
        alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
        l.set_field_bytes(-2, FIELD_RESULT);
      }
    }
  }
}
