use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    get_tag::get_tag,
    get_type_function_runtime::get_type_function_type_id,
    get_type_user_data::get_type_user_data,
    lua_names::{FIELD_INDEX, FIELD_READ_RESULT, FIELD_WRITE_RESULT},
    throw_type_error::throw_type_error,
  },
  macros::lua_check_args,
  records::{
    type_function_extern_type::TypeFunctionExternType,
    type_function_table_indexer::TypeFunctionTableIndexer,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::type_function_type_id::AsTypeFunctionType,
};
pub(crate) fn get_indexer(l: &mut LuaState) -> i32 {
  // `l` 的存活/独占由 `&mut` 承载（`c_thunk!` 蹦床重建）；`tftt`/`tfct` 按 class-index
  // 下转、仅命中 Some 分支才读 `indexer`，其 key_type/value_type 句柄经 `as_type()`
  // safe 门面消费（arena 地址不迁移的构造不变量收口）。全部调用为 safe fn，无 unsafe。
  lua_check_args!(l, != 1, "type.indexer: expected 1 arguments, but got {}");

  let self_ty = get_type_user_data(l, 1);

  if let Some(tftt) = get_type_function_type_id::<TypeFunctionTableType>(self_ty) {
    push_indexer_3(l, &tftt.indexer);
    return 1;
  }

  if let Some(tfct) = get_type_function_type_id::<TypeFunctionExternType>(self_ty) {
    push_indexer_3(l, &tfct.indexer);
    return 1;
  }

  let tag = get_tag(l, self_ty);
  throw_type_error(
    l,
    format_args!(
      "type.indexer: self to be either a table or class, but got {} instead",
      tag
    ),
  );
}

/// `type.indexer` 的 table/extern 两支共用形态：无 indexer 推 nil，否则推
/// `{index, readresult, writeresult}`（read/write 同为 value_type，cpp 同构）。
///
/// 调用序契约（正确性，非内存安全）：`l` 的存活/独占由 `&mut` 承载；`indexer` 为
/// arena 存活节点的字段借用，其内 `TypeFunctionTypeId` 句柄经 `as_type()` safe
/// 门面只读 `type_variant`，违反调用序仅得到错误诊断。
fn push_indexer_3(l: &mut LuaState, indexer: &Option<TypeFunctionTableIndexer>) {
  let Some(indexer) = indexer else {
    l.push_nil();
    return;
  };

  l.create_table(0, 3);

  alloc_type_user_data(l, indexer.key_type.as_type().type_variant.clone(), false);
  l.set_field_bytes(-2, FIELD_INDEX);
  alloc_type_user_data(l, indexer.value_type.as_type().type_variant.clone(), false);
  l.set_field_bytes(-2, FIELD_READ_RESULT);
  alloc_type_user_data(l, indexer.value_type.as_type().type_variant.clone(), false);
  l.set_field_bytes(-2, FIELD_WRITE_RESULT);
}
