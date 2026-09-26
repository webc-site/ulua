use alloc::vec::Vec;

use ulua_vm::{
  functions::{
    lua_getfield::lua_getfield, lua_gettable::lua_gettable, lua_gettop::lua_gettop,
    lua_l_typeerror_l::lua_l_typeerror_l, lua_objlen::lua_objlen, lua_pushinteger::lua_pushinteger,
    lua_pushvalue::lua_pushvalue,
  },
  macros::{
    lua_isnil::lua_isnil, lua_isnoneornil::lua_isnoneornil, lua_istable::lua_istable,
    lua_pop::lua_pop,
  },
  records::lua_state,
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
    arena_handle::Handle, type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariant,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int createFunction(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1308`）。
pub unsafe fn create_function(l: *mut LuaState) -> i32 {
  // Safety: 本函数是注册进 Lua 的 C 函数，VM 依调用约定传入存活非空的 `*mut
  // LuaState`，故 `l as *mut lua_state::LuaState` 为同一对象的合法重解释；块内所有
  // lua_* C-API 调用仅在该 state 上读写其自身的栈槽（下标 1..=3 与 push 后负索引
  // -1/-2 均在已校验的 argument_count 范围内），不构造悬垂/别名引用；runtime 句柄是注册期写入
  // 主线程 thread data 的非空 TypeFunctionRuntime（null 由 Handle::from_ptr 收敛为 panic）。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let runtime = Handle::from_ptr(get_type_function_runtime(l));
    lua_check_args!(vm_l, > 3, "types.newfunction: expected 0-3 arguments, but got {}");

    let arg_types: TypeFunctionTypePackId;

    if lua_istable!(vm_l, 1) {
      lua_getfield(vm_l, 1, FIELD_HEAD.as_ptr().cast());
      lua_getfield(vm_l, 1, FIELD_TAIL.as_ptr().cast());

      arg_types = get_type_pack_runtime(l, -2, -1);

      lua_pop(vm_l, 2);
    } else if !lua_isnoneornil!(vm_l, 1) {
      lua_l_typeerror_l(vm_l, 1, "table");
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

    if lua_istable!(vm_l, 2) {
      lua_getfield(vm_l, 2, FIELD_HEAD.as_ptr().cast());
      lua_getfield(vm_l, 2, FIELD_TAIL.as_ptr().cast());

      ret_types = get_type_pack_runtime(l, -2, -1);

      lua_pop(vm_l, 2);
    } else if !lua_isnoneornil!(vm_l, 2) {
      lua_l_typeerror_l(vm_l, 2, "table");
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
}

/// # Safety
/// `l` 须为 Lua VM 在本次原生函数调用中传入、调用全程有效的 `lua_State*`（本函数只经它
/// 读写 VM 栈）；`head_idx`/`tail_idx` 须是该状态栈上的有效索引，且其中若含 userdata，必须
/// 是由 `alloc_type_user_data` 登记、可被 `get_type_user_data`/`optional_type_user_data`
/// 识别的类型 userdata（这些辅助函数会解引用其 type arena 节点）。对应 C++ 原生
/// `static TypeFunctionTypePackId getTypePack(lua_State* L, int headIdx, int tailIdx)`
/// （`cpp/Analysis/src/TypeFunctionRuntime.cpp:1208`）。
pub(crate) unsafe fn get_type_pack_runtime(
  l: *mut LuaState,
  head_idx: i32,
  tail_idx: i32,
) -> TypeFunctionTypePackId {
  // Safety: 依 fn 文档契约，`l` 为存活非空 LuaState，`head_idx`/`tail_idx` 是其栈上
  // 有效索引；`l as *mut lua_state::LuaState` 为同一对象重解释。lua_* C-API 只操作该
  // state 栈；get_type_user_data/optional_type_user_data 仅识别 alloc_type_user_data
  // 登记的 userdata，(*gty) 解引用前已判 gty 非 null 且 RTTI 命中即 repr(C) 基址重合；
  // runtime 句柄同 create_function：注册期接线、非空由 Handle::from_ptr 兜底 panic。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let runtime = Handle::from_ptr(get_type_function_runtime(l));
    let mut head = Vec::new();

    if lua_istable!(vm_l, head_idx) {
      lua_pushvalue(vm_l, head_idx);

      for i in 1..=lua_objlen(vm_l, -1) {
        lua_pushinteger(vm_l, i);
        lua_gettable(vm_l, -2);

        if lua_isnil!(vm_l, -1) {
          lua_pop(vm_l, 1);
          break;
        }

        head.push(get_type_user_data(l, -1));
        lua_pop(vm_l, 1);
      }

      lua_pop(vm_l, 1);
    }

    let mut tail: Option<TypeFunctionTypePackId> = None;

    if let Some(type_id) = optional_type_user_data(l, tail_idx) {
      let gty = get_type_function_type_id::<TypeFunctionGenericType>(type_id);
      if !gty.is_null() && (*gty).is_pack {
        tail = Some(allocate_type_function_type_pack(
          runtime,
          TypeFunctionTypePackVariant::V2(TypeFunctionGenericTypePack {
            is_named: (*gty).is_named,
            name: (*gty).name.clone(),
          }),
        ));
      } else {
        tail = Some(allocate_type_function_type_pack(
          runtime,
          TypeFunctionTypePackVariant::V1(TypeFunctionVariadicTypePack { type_id }),
        ));
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
}
