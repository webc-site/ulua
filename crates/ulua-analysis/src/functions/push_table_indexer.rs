//! 共享实现：cpp `getReadIndexer`/`getWriteIndexer` 中 table/extern 两分支
//! 完全一致的「把 indexer 推成 {index, result} 表（或 nil）」逻辑抽此一处。

use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    lua_names::{FIELD_INDEX, FIELD_RESULT},
  },
  records::type_function_table_indexer::TypeFunctionTableIndexer,
  type_aliases::type_function_type_id::AsTypeFunctionType,
};

/// 调用序契约（正确性，非内存安全）：`l` 的存活与独占由 `&mut` 承载；`indexer`
/// 为 arena 存活节点的字段借用，其内 `TypeFunctionTypeId` 句柄若非空须指向
/// type_arena 存活节点——读 `type_variant` 一律经 `as_type()` safe 门面（runtime
/// arena bump 分配、地址不迁移的不变量收口其内部 unsafe），违反调用序只会读到
/// 错节点得到错误诊断，不越出内存安全边界。`indexer` 为 `None` 时仅推 nil。
pub(crate) fn push_table_indexer(l: &mut LuaState, indexer: &Option<TypeFunctionTableIndexer>) {
  match indexer {
    None => l.push_nil(),
    Some(indexer) => {
      l.create_table(0, 2);
      alloc_type_user_data(l, indexer.key_type.as_type().type_variant.clone(), false);
      l.set_field_bytes(-2, FIELD_INDEX);
      alloc_type_user_data(l, indexer.value_type.as_type().type_variant.clone(), false);
      l.set_field_bytes(-2, FIELD_RESULT);
    }
  }
}
