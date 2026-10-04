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

/// # Safety
/// `l` 必须是本类型函数运行时会话内存活且单线程独占的 `LuaState`；`idx` 须是该状态栈上
/// 的有效索引，其内容若为 userdata 则须是由 `alloc_type_user_data` 登记的类型 userdata；
/// 主线程 thread data 已安装非空 `TypeFunctionRuntime`（由 `set_type_function_environment` 保证）。
pub(crate) unsafe fn get_generics(
  l: &mut LuaState,
  idx: i32,
  fname: &str,
) -> (Vec<TypeFunctionTypeId>, Vec<TypeFunctionTypePackId>) {
  // Safety: `l` 由 Lua VM 按类型函数运行时约定传入并全程存活（经入口以独占 `&mut` 借入）。
  // 错误分支 `throw_type_error` 返回 `!` 不返回，`lua_l_typeerror_l`
  // 之后不再解引用任何指针。单线程串行遍历，push/gettable/pop 栈操作平衡、无并发别名。
  unsafe {
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
      lua_l_typeerror_l(l.as_mut_ptr(), idx, "table");
    }

    (types, packs)
  }
}
