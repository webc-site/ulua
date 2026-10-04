/// 对应 C++ 原生 `static int createFunction(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1308`）。
use alloc::vec::Vec;

use ulua_vm::{
  functions::{lua_gettable::lua_gettable, lua_l_typeerror_l::lua_l_typeerror_l},
  records::lua_state::LuaState,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    allocate_type_function_type_pack::allocate_type_function_type_pack,
    get_generics::get_generics,
    get_type_function_runtime::{get_type_function_runtime, get_type_function_type_id},
    get_type_user_data::get_type_user_data,
    lua_names::{FIELD_HEAD, FIELD_TAIL},
    optional_type_user_data::optional_type_user_data,
    throw_type_error::throw_type_error,
  },
  macros::lua_check_args,
  records::{
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::{
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariant,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
pub(crate) fn create_function(l: &mut LuaState) -> i32 {
  let runtime = get_type_function_runtime(l).expect("runtime 于注册阶段挂载，会话内恒非空");
  lua_check_args!(l, > 3, "types.newfunction: expected 0-3 arguments, but got {}");

  let arg_types: TypeFunctionTypePackId;

  if l.is_table(1) {
    l.get_field_bytes(1, FIELD_HEAD);
    l.get_field_bytes(1, FIELD_TAIL);

    arg_types = get_type_pack_runtime(l, -2, -1);

    l.pop(2);
  } else if !l.is_none_or_nil(1) {
    // Safety: `lua_l_typeerror_l` 是 vm 侧 C 形态门面，`l.as_mut_ptr()` 为 `&mut l`
    // 同一对象的镜像透传；下标 1 由上方 `is_table`/`is_none_or_nil` 分支确证为已入栈实参。
    unsafe { lua_l_typeerror_l(l.as_mut_ptr(), 1, "table") };
  } else {
    arg_types = allocate_type_function_type_pack(
      runtime,
      TypeFunctionTypePackVariant::V0(TypeFunctionTypePack {
        head: Vec::new(),
        tail: None,
      }),
    );
  }

  let ret_types: TypeFunctionTypePackId;

  if l.is_table(2) {
    l.get_field_bytes(2, FIELD_HEAD);
    l.get_field_bytes(2, FIELD_TAIL);

    ret_types = get_type_pack_runtime(l, -2, -1);

    l.pop(2);
  } else if !l.is_none_or_nil(2) {
    // Safety: 同上，`l.as_mut_ptr()` 为镜像透传；下标 2 是已校验范围内的实参位。
    unsafe { lua_l_typeerror_l(l.as_mut_ptr(), 2, "table") };
  } else {
    ret_types = allocate_type_function_type_pack(
      runtime,
      TypeFunctionTypePackVariant::V0(TypeFunctionTypePack {
        head: Vec::new(),
        tail: None,
      }),
    );
  }

  let (generic_types, generic_packs) = get_generics(l, 3, "types.newfunction");

  alloc_type_user_data(
    l,
    TypeFunctionTypeVariant::Function(TypeFunctionFunctionType {
      generics: generic_types,
      generic_packs,
      arg_types,
      ret_types,
      arg_names: Vec::new(),
    }),
    false,
  );

  1
}

/// 对应 C++ 原生
/// `static TypeFunctionTypePackId getTypePack(lua_State* L, int headIdx, int tailIdx)`
/// （`cpp/Analysis/src/TypeFunctionRuntime.cpp:1208`）。
///
/// 本函数是 safe fn：形参为 `&mut LuaState`/`i32`，无调用方传入的裸指针，体内亦无原生
/// unsafe 操作（VM 读写全部经 safe 门面）；返回的 pack 句柄由 runtime bump arena 分配、
/// 比 `l` 长寿，与 `get_type_user_data` 同形态。
///
/// 调用序契约（正确性，非内存安全）：`l` 须为本次原生函数调用全程存活的状态；
/// `head_idx`/`tail_idx` 须是其栈上有效索引，且其中若含 userdata，必须是由
/// `alloc_type_user_data` 登记、可被 `get_type_user_data`/`optional_type_user_data`
/// 识别的类型 userdata；runtime 未挂载时 `expect` 以确定性 panic 收敛，不构成 UB。
pub(crate) fn get_type_pack_runtime(
  l: &mut LuaState,
  head_idx: i32,
  tail_idx: i32,
) -> TypeFunctionTypePackId {
  // 调用序契约见函数头：runtime 未挂载由 expect 兜底 panic；
  // get_type_user_data/optional_type_user_data 仅识别 alloc_type_user_data
  // 登记的 userdata；gty 命中 Some 后仅读取 is_pack/is_named/name（arena 块地址
  // 不移动）。
  let runtime = get_type_function_runtime(l).expect("runtime 于注册阶段挂载，会话内恒非空");
  let mut head = Vec::new();

  if l.is_table(head_idx) {
    l.push_value(head_idx);

    for i in 1..=l.obj_len(-1) as i32 {
      l.push_integer(i);
      lua_gettable(l, -2);

      if l.is_nil(-1) {
        l.pop(1);
        break;
      }

      head.push(get_type_user_data(l, -1));
      l.pop(1);
    }

    l.pop(1);
  }

  let mut tail: Option<TypeFunctionTypePackId> = None;

  if let Some(type_id) = optional_type_user_data(l, tail_idx) {
    match get_type_function_type_id::<TypeFunctionGenericType>(type_id) {
      Some(gty) if gty.is_pack => {
        tail = Some(allocate_type_function_type_pack(
          runtime,
          TypeFunctionTypePackVariant::V2(TypeFunctionGenericTypePack {
            is_named: gty.is_named,
            name: gty.name.clone(),
          }),
        ));
      }
      _ => {
        tail = Some(allocate_type_function_type_pack(
          runtime,
          TypeFunctionTypePackVariant::V1(TypeFunctionVariadicTypePack { type_id }),
        ));
      }
    }
  }

  match tail {
    Some(t) if head.is_empty() => t,
    tail => allocate_type_function_type_pack(
      runtime,
      TypeFunctionTypePackVariant::V0(TypeFunctionTypePack { head, tail }),
    ),
  }
}
