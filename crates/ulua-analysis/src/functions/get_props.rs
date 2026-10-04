/// 对应 C++ 原生 `static int getProps(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1568`）。
use alloc::{collections::BTreeMap, string::String};

use ulua_vm::{
  functions::{lua_createtable::lua_createtable, lua_settable::lua_settable},
  records::lua_state::LuaState,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    get_tag::get_tag,
    get_type_function_runtime::get_type_function_type_id,
    get_type_user_data::get_type_user_data,
    lua_names::{FIELD_READ, FIELD_WRITE},
    throw_type_error::throw_type_error,
  },
  macros::lua_check_args,
  records::{
    type_function_extern_type::TypeFunctionExternType,
    type_function_property::TypeFunctionProperty,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_string_singleton::TypeFunctionStringSingleton,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::{
    type_function_singleton_variant::TypeFunctionSingletonVariant,
    type_function_type_id::AsTypeFunctionType, type_function_type_variant::TypeFunctionTypeVariant,
  },
};
pub(crate) fn get_props(l: &mut LuaState) -> i32 {
  // `l` 的存活/独占由 `&mut` 承载（`c_thunk!` 蹦床重建）；`tftt`/`tfct` 命中 Some
  // 才读 `props`，条目内 read_ty/write_ty 句柄经 `as_type()` safe 门面消费
  // （class-index 下转 + arena 地址不迁移的构造不变量收口）。全部调用为 safe fn，无 unsafe。
  lua_check_args!(l, != 1, "type.properties: expected 1 arguments, but got {}");

  let self_ty = get_type_user_data(l, 1);

  if let Some(tftt) = get_type_function_type_id::<TypeFunctionTableType>(self_ty) {
    push_props(l, &tftt.props);
    return 1;
  }

  if let Some(tfct) = get_type_function_type_id::<TypeFunctionExternType>(self_ty) {
    push_props(l, &tfct.props);
    return 1;
  }

  let tag = get_tag(l, self_ty);
  throw_type_error(
    l,
    format_args!(
      "type.properties: expected self to be either a table or class, but got {} instead",
      tag
    ),
  );
}

/// 调用序契约（正确性，非内存安全）：`l` 的存活/栈独占由 `&mut` 承载；`props` 为
/// arena 存活节点的字段借用，其内 `read_ty`/`write_ty` 句柄经 `as_type()` safe
/// 门面只读 `type_variant`，违反调用序仅得到错误诊断。
fn push_props(l: &mut LuaState, props: &BTreeMap<String, TypeFunctionProperty>) {
  // Safety: `l.as_mut_ptr()` 指向 `&mut` 契约保证的同一存活、对齐 lua_State；
  // createtable 压入 props.len() 表，后续键值经 alloc/settable 成对消费，
  // 单线程串行、无别名。
  unsafe { lua_createtable(l.as_mut_ptr(), props.len() as i32, 0) };

  for (name, prop) in props {
    alloc_type_user_data(
      l,
      TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
        variant: TypeFunctionSingletonVariant::V1(TypeFunctionStringSingleton {
          value: name.clone(),
        }),
      }),
      false,
    );

    let mut size: i32 = 0;
    if prop.read_ty.is_some() {
      size += 1;
    }
    if prop.write_ty.is_some() {
      size += 1;
    }

    // Safety: 同上，createtable 压入至多 2 项的内表，read/write 字段各由
    // setfield 消费一项。
    unsafe { lua_createtable(l.as_mut_ptr(), 0, size) };

    if let Some(read_ty) = prop.read_ty {
      alloc_type_user_data(l, read_ty.as_type().type_variant.clone(), false);
      l.set_field_bytes(-2, FIELD_READ);
    }

    if let Some(write_ty) = prop.write_ty {
      alloc_type_user_data(l, write_ty.as_type().type_variant.clone(), false);
      l.set_field_bytes(-2, FIELD_WRITE);
    }

    lua_settable(l, -3);
  }
}
