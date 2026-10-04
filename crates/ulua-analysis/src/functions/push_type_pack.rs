use ulua_vm::{
  functions::{lua_createtable::lua_createtable, lua_rawseti::lua_rawseti},
  records::lua_state::LuaState,
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
    type_function_type_pack_id::TypeFunctionTypePackId, type_function_type_id::AsTypeFunctionType,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};

/// 调用序契约（正确性，非内存安全）：`l` 的存活与本次调用的栈独占由 `&mut` 承载；
/// `tp` 须为 `TypeFunctionRuntime` 的 `type_pack_arena` 中存活的序列化 pack 句柄
/// （shallow+deep 序列化完成后才会被推送）——节点解引用一律经 safe 门面
/// （`get_type_function_type_pack_id`/`as_pack`/`as_type`，arena 地址不迁移的构造
/// 不变量收口其内部 unsafe）。对应 C++
/// `void pushTypePack(lua_State* L, TypeFunctionTypePackId tp)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1254`）。
pub(crate) fn push_type_pack(l: &mut LuaState, tp: TypeFunctionTypePackId) {
  if let Some(tftp) = get_type_function_type_pack_id::<TypeFunctionTypePack>(tp) {
    // Safety: `l` 按 `&mut` 契约为宿主 lua 虚拟机创建的存活 lua_State（crate 的
    // LuaState 是不透明镜像类型），`l.as_mut_ptr()` 为同一对象的重解释；createtable
    // 压入 1 表，后续字段经 setfield(rawseti) 弹出，单线程串行执行、无别名。
    unsafe { lua_createtable(l.as_mut_ptr(), 0, 2) };

    if !tftp.head.is_empty() {
      // Safety: 同上，createtable 压入 head 长的数组表，逐项 rawseti 消费栈顶。
      unsafe { lua_createtable(l.as_mut_ptr(), tftp.head.len() as i32, 0) };
      for (idx, el) in tftp.head.iter().enumerate() {
        alloc_type_user_data(l, el.as_type().type_variant.clone(), false);
        lua_rawseti(l, -2, (idx + 1) as i32);
      }

      l.set_field_bytes(-2, FIELD_HEAD);
    }

    if let Some(tail) = tftp.tail {
      push_type_pack_tail(l, tail);
      l.set_field_bytes(-2, FIELD_TAIL);
    }
  } else if let Some(tfvp) = get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tp) {
    // Safety: 同上，createtable 压入 1 表，tail 字段经 setfield 消费。
    unsafe { lua_createtable(l.as_mut_ptr(), 0, 1) };

    alloc_type_user_data(l, tfvp.type_id.as_type().type_variant.clone(), false);
    l.set_field_bytes(-2, FIELD_TAIL);
  } else if let Some(tfgp) = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp) {
    // Safety: 同上，createtable 压入 1 表，tail 字段经 setfield 消费。
    unsafe { lua_createtable(l.as_mut_ptr(), 0, 1) };

    alloc_type_user_data(
      l,
      TypeFunctionTypeVariant::Generic(TypeFunctionGenericType {
        is_named: tfgp.is_named,
        is_pack: true,
        name: tfgp.name.clone(),
      }),
      false,
    );
    l.set_field_bytes(-2, FIELD_TAIL);
  } else {
    throw_type_error(l, format_args!("unsupported type pack type"));
  }
}

/// 调用序契约（正确性，非内存安全）：`l` 的存活/独占由 `&mut` 承载；`tail` 须为
/// 存活 pack arena 句柄且其变体属于被识别的 variadic/generic pack——解引用经
/// `get_type_function_type_pack_id`/`as_type` safe 门面收口，违反调用序仅得到
/// 错误诊断而非内存不安全。
fn push_type_pack_tail(l: &mut LuaState, tail: TypeFunctionTypePackId) {
  if let Some(tfvp) = get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tail) {
    alloc_type_user_data(l, tfvp.type_id.as_type().type_variant.clone(), false);
    return;
  }

  if let Some(tfgp) = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tail) {
    alloc_type_user_data(
      l,
      TypeFunctionTypeVariant::Generic(TypeFunctionGenericType {
        is_named: tfgp.is_named,
        is_pack: true,
        name: tfgp.name.clone(),
      }),
      false,
    );
    return;
  }

  throw_type_error(l, format_args!("unsupported type pack type"));
}
