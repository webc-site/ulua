use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  functions::{lua_getthreaddata::lua_getthreaddata, lua_mainthread::lua_mainthread},
  records::lua_state::LuaState,
};

use crate::{
  records::{
    arena_handle::{Handle, alias_opt, alias_ref},
    type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
    type_function_type_variant::TypeFunctionTypeVariantMember,
  },
};

/// 取回挂载在 `l` 所属主线程 userdatum 上的 `TypeFunctionRuntime` 句柄；注册阶段
/// 未挂载（或已卸载）时为 `None`。句柄目标由主线程 userdatum 保活、地址不迁移，
/// 存活/单线程独占借用契约见 `records/arena_handle.rs` 模块头。对应 C++
/// `TypeFunctionRuntime* getTypeFunctionRuntime(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:357`）。
pub(crate) fn get_type_function_runtime(l: &mut LuaState) -> Option<Handle<TypeFunctionRuntime>> {
  // `lua_mainthread`/`lua_getthreaddata` 均为 ulua-vm 的 safe 门面（收引用、还裸
  // 指针），主线程指针与 thread data 的解引用一律经 `alias_ref`/`Handle::from_opt_ptr`
  // 收口；null 折叠为 `None`，非空句柄由类型编码。
  let main_thread = lua_mainthread(l);
  let data = lua_getthreaddata(alias_ref(main_thread));
  Handle::from_opt_ptr(data.cast())
}

// Source: `Analysis/include/Luau/TypeFunctionRuntime.h:167-173` (hand-ported)
/// C++ `template<typename T> const T* get(TypeFunctionTypePackId tv)`：按变体 tag
/// 探测 pack 节点负载，未命中（含空句柄这一契约违例的确定性兜底）以 `None` 表达。
///
/// 前提由构造不变量保证：`tv` 出自 `type_pack_arena`（chunked bump 分配，元素
/// 地址随 arena 存活期不迁移），空指针在函数内先行折叠为 `None`、不解引用；
/// 解引用经 `arena_handle::alias_opt` 门面，读侧无并存可变别名的前提同该模块头契约。
pub(crate) fn get_type_function_type_pack_id<'a, T: TypeFunctionTypePackVariantMember>(
  tv: TypeFunctionTypePackId,
) -> Option<&'a T> {
  LUAU_ASSERT!(!tv.is_null());
  T::get_if(&alias_opt(tv)?.type_variant)
}

// Source: `Analysis/include/Luau/TypeFunctionRuntime.h:275-281` (hand-ported)
/// C++ `template<typename T> const T* get(TypeFunctionTypeId tv)`：[`get_type_function_type_pack_id`]
/// 的 type 侧对偶，节点属 `type_arena`。
///
/// 前提同 [`get_type_function_type_pack_id`]：构造不变量（bump arena 地址不迁移）
/// + 函数内判空，命中时按 `repr(C)` class-index 下转、只读回该变体字段。
pub(crate) fn get_type_function_type_id<'a, T: TypeFunctionTypeVariantMember>(
  tv: TypeFunctionTypeId,
) -> Option<&'a T> {
  LUAU_ASSERT!(!tv.is_null());
  T::get_if(&alias_opt(tv)?.type_variant)
}
