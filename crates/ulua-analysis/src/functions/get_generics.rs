//! Faithful port of
//! `static std::tuple<std::vector<TypeFunctionTypeId>, std::vector<TypeFunctionTypePackId>>
//!  getGenerics(lua_State* l, int idx, const char* fname)`
//! (Analysis/src/TypeFunctionRuntime.cpp:1125-1177).

use alloc::vec::Vec;

use ulua_vm::{
  functions::{lua_gettable::lua_gettable, lua_l_typeerror_l::lua_l_typeerror_l},
  records::lua_state::LuaState,
};

use crate::{
  functions::{
    allocate_type_function_type_pack::allocate_type_function_type_pack,
    get_type_function_runtime::{get_type_function_runtime, get_type_function_type_id},
    get_type_user_data::get_type_user_data,
    throw_type_error::throw_type_error,
  },
  records::{
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
  },
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariant,
  },
};

/// 本函数是 safe fn：形参为 `&mut LuaState`/`i32`/`&str`，无调用方传入的裸指针；体内
/// `lua_l_typeerror_l`/`lua_gettable` 等 VM 侧门面已随 wave-6d/r16 收形为引用形安全
/// 函数，调用方无需承担任何内存安全前提。
/// 返回的 arena 句柄向量源自本次调用内部分配。
///
/// 调用序契约（正确性，非内存安全）：`l` 须为类型函数运行时会话内存活的状态；`idx` 须是
/// 该状态栈上的有效索引，其内容若为 userdata 则须是由 `alloc_type_user_data` 登记的类型
/// userdata；主线程 thread data 已在 runtime 安装期写入 `TypeFunctionRuntime`——违约时
/// `expect`/VM 错误路径以确定性 panic 收敛，不构成 UB。
pub(crate) fn get_generics(
  l: &mut LuaState,
  idx: i32,
  fname: &str,
) -> (Vec<TypeFunctionTypeId>, Vec<TypeFunctionTypePackId>) {
  // 注册期写入主线程 thread data 的非空 runtime（未挂载属契约违例，`expect` 收敛为
  // 确定性 panic）。
  let runtime = get_type_function_runtime(l).expect("runtime 于注册阶段挂载，会话内恒非空");

  let mut types: Vec<TypeFunctionTypeId> = Vec::new();
  let mut packs: Vec<TypeFunctionTypePackId> = Vec::new();

  if l.is_table(idx) {
    l.push_value(idx);

    let mut i: i32 = 1;
    while i <= l.obj_len(-1) as i32 {
      l.push_integer(i);
      lua_gettable(l, -2);

      if l.is_nil(-1) {
        l.pop(1);
        break;
      }

      // TypeFunctionTypeId ty = getTypeUserData(l, -1);
      let ty = get_type_user_data(l, -1);

      // if (auto gty = get<TypeFunctionGenericType>(ty))
      match get_type_function_type_id::<TypeFunctionGenericType>(ty) {
        Some(gty) if gty.is_pack => {
          packs.push(allocate_type_function_type_pack(
            runtime,
            TypeFunctionTypePackVariant::V2(TypeFunctionGenericTypePack {
              is_named: gty.is_named,
              name: gty.name.clone(),
            }),
          ));
        }
        Some(_) => {
          if !packs.is_empty() {
            throw_type_error(
              l,
              format_args!("{}: generic type cannot follow a generic pack", fname),
            );
          }

          types.push(ty);
        }
        None => throw_type_error(
          l,
          format_args!("{}: table member was not a generic type", fname),
        ),
      }

      l.pop(1);
      i += 1;
    }

    l.pop(1);
  } else if !l.is_none_or_nil(idx) {
    lua_l_typeerror_l(l, idx, "table");
  }

  (types, packs)
}
