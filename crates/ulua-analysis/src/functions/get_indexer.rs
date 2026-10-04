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
};
pub(crate) fn get_indexer(l: &mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 运行时约定传入并全程存活（经 `c_thunk!` 蹦床重建为独占 `&mut`）。`tftt`/`tfct`
  // 按 class-index 下转，仅命中 Some 分支才读取 `indexer`；indexer 字段先经 `is_none()` 判定、
  // else 支由同判据 `expect` 取回（必为 Some），其 key_type/value_type 是 arena 存活 TypeId
  // （地址不移动）。各 `throw_type_error` 分支返回 `!` 不返回。单线程串行执行，无并发别名。
  unsafe {
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
}

/// `type.indexer` 的 table/extern 两支共用形态：无 indexer 推 nil，否则推
/// `{index, readresult, writeresult}`（read/write 同为 value_type，cpp 同构）。
///
/// # Safety
/// `l` 须为存活且本次调用独占的 `LuaState`（写栈经 C-API）；`indexer` 内的
/// `TypeFunctionTypeId`（裸指针）若非空须指向 type_arena 存活节点，本函数以
/// `(*..).type_variant` 只读它们并经 `alloc_type_user_data` 消费。
unsafe fn push_indexer_3(l: &mut LuaState, indexer: &Option<TypeFunctionTableIndexer>) {
  if indexer.is_none() {
    l.push_nil();
    return;
  }

  l.create_table(0, 3);

  // Safety: 上方 is_none 分支已 return，此处必为 Some（cpp 同位 is_none 后直 deref）。
  let indexer = indexer
    .as_ref()
    .expect("is_none 分支已 return，至此必为 Some");
  // Safety: 本函数 `# Safety` 段契约——`key_type`/`value_type` 为 type_arena 存活
  // 句柄、variant 只读克隆，`l` 存活且本次调用独占（`alloc_type_user_data` 与
  // vm 栈写原语的未批次 unsafe 前提）。
  unsafe {
    alloc_type_user_data(l, (*indexer.key_type).type_variant.clone(), false);
    l.set_field_bytes(-2, FIELD_INDEX);
    alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
    l.set_field_bytes(-2, FIELD_READ_RESULT);
    alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
    l.set_field_bytes(-2, FIELD_WRITE_RESULT);
  }
}
