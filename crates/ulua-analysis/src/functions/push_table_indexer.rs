//! 共享实现：cpp `getReadIndexer`/`getWriteIndexer` 中 table/extern 两分支
//! 完全一致的「把 indexer 推成 {index, result} 表（或 nil）」逻辑抽此一处。

use ulua_vm::{
  functions::{
    lua_createtable::lua_createtable, lua_pushnil::lua_pushnil, lua_setfield::lua_setfield,
  },
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    lua_names::{FIELD_INDEX, FIELD_RESULT},
  },
  records::type_function_table_indexer::TypeFunctionTableIndexer,
  type_aliases::lua_state::LuaState,
};

/// # Safety
/// 调用方须保证 `l`、`vm_l` 裸指针有效，满足 C++ 原实现的调用契约。
pub(crate) unsafe fn push_table_indexer(
  l: *mut LuaState,
  vm_l: *mut lua_state::LuaState,
  indexer: &Option<TypeFunctionTableIndexer>,
) {
  unsafe {
    match indexer {
      None => lua_pushnil(vm_l),
      Some(indexer) => {
        lua_createtable(vm_l, 0, 2);
        alloc_type_user_data(l, (*indexer.key_type).type_variant.clone(), false);
        lua_setfield(vm_l, -2, FIELD_INDEX.as_ptr().cast());
        alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
        lua_setfield(vm_l, -2, FIELD_RESULT.as_ptr().cast());
      }
    }
  }
}
