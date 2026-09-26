use ulua_vm::{
  functions::{
    lua_createtable::lua_createtable, lua_rawseti::lua_rawseti, lua_setfield::lua_setfield,
  },
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    get_type_function_runtime::get_type_function_type_pack_id,
    lua_names::{FIELD_HEAD, FIELD_TAIL},
    throw_type_error::throw_type_error,
  },
  records::{
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
/// # Safety
/// `l` 必须是当前调用栈有效、且已挂载 `TypeFunctionRuntime` 的 `lua_State*`；本函数向该栈
/// 压入若干值（不弹出），调用方须按 C++ 原约定管理栈，且调用期间单线程独占 VM 栈。对应 C++
/// `void pushTypePack(lua_State* L, TypeFunctionTypePackId tp)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1254`）。
pub unsafe fn push_type_pack(l: *mut LuaState, tp: TypeFunctionTypePackId) {
  // Safety: 前置条件逐项——(1) `l` 按本函数契约是宿主 lua 虚拟机创建的存活
  // lua_State（crate 的 LuaState 是不透明镜像类型），`l as *mut LuaState` 为
  // 同一对象的重解释，lua_newstate 保证其对齐；(2) tp 是 TypeFunctionRuntime
  // bump arena 中的完整序列化 pack 节点（仅在 shallow+deep 序列化完成后被推送），
  // get_type_function_type_pack_id 按 class index 分派：判空后 `(*tftp)`/`(*tfvp)`/
  // `(*tfgp)` 命中即动态类型正确且基址重合；head 元素与 variadic 的 `type_id`
  // 同为 arena 分配/序列化回填的非空节点，`(**el)`、`(*(*tfvp).type_id)` 与 C++
  // oracle `tfvp->type->type`（TypeFunctionRuntime.cpp:1254）同前提；(3)
  // lua_createtable/lua_rawseti/lua_setfield/throw_type_error 依 Lua C-API 栈约定
  // 使用：createtable 压入 1 表，字段经 setfield(rawseti) 弹出，键名是 lua_names 的静态 NUL 结尾
  // NUL 结尾字面量；整块单线程串行执行，无别名。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;

    let tftp = get_type_function_type_pack_id::<TypeFunctionTypePack>(tp);
    if !tftp.is_null() {
      lua_createtable(vm_l, 0, 2);

      if !(*tftp).head.is_empty() {
        lua_createtable(vm_l, (*tftp).head.len() as i32, 0);
        for (idx, el) in (*tftp).head.iter().enumerate() {
          alloc_type_user_data(l, (**el).type_variant.clone(), false);
          lua_rawseti(vm_l, -2, (idx + 1) as i32);
        }

        lua_setfield(vm_l, -2, FIELD_HEAD.as_ptr().cast());
      }

      if let Some(tail) = (*tftp).tail {
        push_type_pack_tail(l, vm_l, tail);
        lua_setfield(vm_l, -2, FIELD_TAIL.as_ptr().cast());
      }
    } else {
      let tfvp = get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tp);
      if !tfvp.is_null() {
        lua_createtable(vm_l, 0, 1);

        alloc_type_user_data(l, (*(*tfvp).type_id).type_variant.clone(), false);
        lua_setfield(vm_l, -2, FIELD_TAIL.as_ptr().cast());
      } else {
        let tfgp = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp);
        if !tfgp.is_null() {
          lua_createtable(vm_l, 0, 1);

          alloc_type_user_data(
            l,
            TypeFunctionTypeVariant::Generic(TypeFunctionGenericType {
              is_named: (*tfgp).is_named,
              is_pack: true,
              name: (*tfgp).name.clone(),
            }),
            false,
          );
          lua_setfield(vm_l, -2, FIELD_TAIL.as_ptr().cast());
        } else {
          throw_type_error(vm_l, format_args!("unsupported type pack type"));
        }
      }
    }
  }
}

/// # Safety
/// `l` 与 `vm_l` 须为同一有效 Lua 状态（`vm_l` 是 `l` 的重解释）。`tail` 须为
/// 非空且指向存活类型函数 pack 节点的 `TypeFunctionTypePackId`，且其变体属于
/// 被识别的 variadic/generic pack——本函数在 null 检查后以 `(*tfvp).type_id` /
/// `(*tfgp).…` 解引用它，并经 `alloc_type_user_data` 与 Lua C-API（FFI）写栈。
unsafe fn push_type_pack_tail(
  l: *mut LuaState,
  vm_l: *mut lua_state::LuaState,
  tail: TypeFunctionTypePackId,
) {
  // Safety: 依函数头 # Safety——l/vm_l 为同一存活 lua_State（重解释、对齐由
  // 构造保证），tail 为运行时 arena 中存活 pack 节点；get_type_function_type_pack_id
  // 按 class index 分派，判空命中后 `(*tfvp)`/`(*tfgp)` 类型正确、基址重合，
  // `(*(*tfvp).type_id)` 的 type_id 已由 deep 序列化回填（与 C++ pushTypePack
  // 尾部同一解引用）；alloc_type_user_data/throw_type_error 按 Lua C-API 栈约定
  // 传参，格式串收敛在 throw_type_error 一处；单线程串行、无别名。
  unsafe {
    let tfvp = get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tail);
    if !tfvp.is_null() {
      alloc_type_user_data(l, (*(*tfvp).type_id).type_variant.clone(), false);
      return;
    }

    let tfgp = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tail);
    if !tfgp.is_null() {
      alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Generic(TypeFunctionGenericType {
          is_named: (*tfgp).is_named,
          is_pack: true,
          name: (*tfgp).name.clone(),
        }),
        false,
      );
      return;
    }

    throw_type_error(vm_l, format_args!("unsupported type pack type"));
  }
}
