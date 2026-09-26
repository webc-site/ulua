use alloc::{collections::BTreeMap, string::String};

use ulua_vm::{
  functions::{
    lua_createtable::lua_createtable, lua_gettop::lua_gettop, lua_setfield::lua_setfield,
    lua_settable::lua_settable,
  },
  records::lua_state,
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
    lua_state::LuaState, type_function_singleton_variant::TypeFunctionSingletonVariant,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int getProps(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1568`）。
pub unsafe fn get_props(l: *mut LuaState) -> i32 {
  // Safety: `l` 由 Lua 虚拟机按其 C 函数调用约定传入，是一个有效、对齐且存活于本次
  // 调用期间的 `*mut LuaState`（对应 `lua_CFunction` 形参契约）；据此重解释得到的
  // `vm_l` 指向同一状态机。`lua_gettop`/`throw_type_error` 等 lua_* 调用只以该状态为参数
  // 操作栈。`get_type_user_data`/`get_type_function_type_id` 均为 `unsafe fn`，契约要求
  // 传入存活 userdata/类型句柄并返回空或指向存活 arena 节点的指针：`tftt`/`tfct` 在
  // `!is_null()` 守卫后才解引用，且其 RTTI class-index 命中 ⇒ `repr(C)` 基址重合，指向
  // 的 `props` 字段随类型 arena 存活；`get_tag` 同理只读该 userdata。单线程执行无别名。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_check_args!(vm_l, != 1, "type.properties: expected 1 arguments, but got {}");

    let self_ty = get_type_user_data(l, 1);

    let tftt = get_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if !tftt.is_null() {
      push_props(l, vm_l, &(*tftt).props);
      return 1;
    }

    let tfct = get_type_function_type_id::<TypeFunctionExternType>(self_ty);
    if !tfct.is_null() {
      push_props(l, vm_l, &(*tfct).props);
      return 1;
    }

    throw_type_error(
      vm_l,
      format_args!(
        "type.properties: expected self to be either a table or class, but got {} instead",
        get_tag(l, self_ty)
      ),
    );
  }
}

/// # Safety
/// `l` 与 `vm_l` 须为同一有效 Lua 状态（`vm_l` 是 `l` 重解释为
/// `*mut lua_state::LuaState`）。`props` 内每个 `TypeFunctionProperty` 的
/// `read_ty`/`write_ty`（`TypeId = *const Type`）须指向存活的类型 arena 节点，
/// 因本函数以 `(*read_ty).type_variant` 解引用它们；随后经 Lua C-API（FFI）写栈。
unsafe fn push_props(
  l: *mut LuaState,
  vm_l: *mut lua_state::LuaState,
  props: &BTreeMap<String, TypeFunctionProperty>,
) {
  // Safety: `l` 与 `vm_l` 由调用方 `get_props` 保证是同一有效存活的 `*mut LuaState`
  // （见其 C-函数契约）；`lua_createtable`/`lua_setfield`/`lua_settable` 仅向该状态栈写入，
  // `alloc_type_user_data` 亦以此为参。`props` 内 `read_ty`/`write_ty`（`TypeId = *const Type`）
  // 是在 `is_some()` 守卫后经 `.clone()` 读取的存活 arena 类型句柄，`(*read_ty).type_variant`
  // 只读、指向 arena 中该类型节点；单线程遍历 `props` 时对这些 arena 节点无并存可变借用。
  unsafe {
    lua_createtable(vm_l, props.len() as i32, 0);

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

      lua_createtable(vm_l, 0, size);

      if let Some(read_ty) = prop.read_ty {
        alloc_type_user_data(l, (*read_ty).type_variant.clone(), false);
        lua_setfield(vm_l, -2, FIELD_READ.as_ptr().cast());
      }

      if let Some(write_ty) = prop.write_ty {
        alloc_type_user_data(l, (*write_ty).type_variant.clone(), false);
        lua_setfield(vm_l, -2, FIELD_WRITE.as_ptr().cast());
      }

      lua_settable(vm_l, -3);
    }
  }
}
