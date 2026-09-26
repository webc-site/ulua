//! Faithful port of `void registerTypeUserData(LuaState* l)`
//! (Analysis/src/TypeFunctionRuntime.cpp:1932-2064).
//!
//! Creates and registers the `"type"` metatable for type userdata: installs
//! `__type`, `__metatable`, `__eq`, the method table, an optional `issubtypeof`
//! method (gated on `LuauUdtfTypeIsSubtypeOf`), the dynamic `__index` closure,
//! and the userdata destructor.
// `kTypeUserdataTag` (Analysis/src/TypeFunctionRuntime.cpp:250).
use core::{ffi::c_void, ptr::null};

use ulua_common::fflag;
use ulua_vm::{
  functions::{
    lua_l_newmetatable::lua_l_newmetatable, lua_l_register::lua_l_register,
    lua_pushstring::lua_pushstring, lua_setfield::lua_setfield, lua_setreadonly::lua_setreadonly,
    lua_setuserdatadtor::lua_setuserdatadtor,
  },
  macros::{
    lua_newtable::lua_newtable, lua_pop::lua_pop, lua_pushcclosure::lua_pushcclosure,
    lua_pushcfunction::LUA_PUSHCFUNCTION,
  },
  records::{lua_l_reg::LuaLReg, lua_state},
};

use crate::{
  functions::{
    check_tag::check_tag,
    dealloc_type_user_data::dealloc_type_user_data,
    get_components::get_components,
    get_function_generics::get_function_generics,
    get_function_parameters::get_function_parameters,
    get_function_returns::get_function_returns,
    get_generic_is_pack::get_generic_is_pack,
    get_generic_name::get_generic_name,
    get_indexer::get_indexer,
    get_metatable_type_function_runtime::get_metatable,
    get_negated_value::get_negated_value,
    get_props::get_props,
    get_read_indexer::get_read_indexer,
    get_read_parent::get_read_parent,
    get_singleton_value::get_singleton_value,
    get_write_indexer::get_write_indexer,
    get_write_parent::get_write_parent,
    is_equal_to_type::is_equal_to_type,
    is_subtype_of::is_subtype_of,
    lua_names::{
      FIELD_EQ, FIELD_INDEX_CLOSURE, FIELD_METATABLE, FIELD_TYPE_TAG, METATABLE_LOCKED,
      METHOD_IS_SUBTYPE_OF, TYPE,
    },
    read_table_prop::read_table_prop,
    set_function_generics::set_function_generics,
    set_function_parameters::set_function_parameters,
    set_function_returns::set_function_returns,
    set_read_table_prop::set_read_table_prop,
    set_table_indexer::set_table_indexer,
    set_table_metatable::set_table_metatable,
    set_table_prop::set_table_prop,
    set_table_read_indexer::set_table_read_indexer,
    set_table_write_indexer::set_table_write_indexer,
    set_write_table_prop::set_write_table_prop,
    type_userdata_index::type_userdata_index,
    write_table_prop::write_table_prop,
  },
  type_aliases::lua_state::LuaState,
};
const K_TYPE_USERDATA_TAG: i32 = 42;

/// Generates a `LuaCfunction`-shaped thunk forwarding to an analysis-level
/// function declared over the opaque `LuaState` struct.
macro_rules! tud_thunk {
  ($thunk:ident, $real:path) => {
    unsafe extern "C-unwind" fn $thunk(l: *mut ulua_vm::records::lua_state::LuaState) -> i32 {
      // Safety: 本 thunk 仅经 LUA_PUSHCFUNCTION/luaL_register/lua_pushcclosure 注册进
      // Lua VM；VM 回调 C 函数时传入的 `l` 是当次调用独占的有效 LuaState（与 C++
      // `lua_CFunction` 入口契约相同）。analysis 侧 `LuaState` 为不透明结构体，`as`
      // 转型为地址不变透传；`$real` 的 unsafe fn 前置条件正是“C 调用期间有效的
      // LuaState 指针”，由 VM 回调约定逐次满足。
      unsafe { $real(l as *mut LuaState) }
    }
  };
}

tud_thunk!(check_tag_thunk, check_tag);
tud_thunk!(get_negated_value_thunk, get_negated_value);
tud_thunk!(get_singleton_value_thunk, get_singleton_value);
tud_thunk!(set_table_prop_thunk, set_table_prop);
tud_thunk!(set_read_table_prop_thunk, set_read_table_prop);
tud_thunk!(set_write_table_prop_thunk, set_write_table_prop);
tud_thunk!(read_table_prop_thunk, read_table_prop);
tud_thunk!(write_table_prop_thunk, write_table_prop);
tud_thunk!(get_props_thunk, get_props);
tud_thunk!(set_table_indexer_thunk, set_table_indexer);
tud_thunk!(set_table_read_indexer_thunk, set_table_read_indexer);
tud_thunk!(set_table_write_indexer_thunk, set_table_write_indexer);
tud_thunk!(get_indexer_thunk, get_indexer);
tud_thunk!(get_read_indexer_thunk, get_read_indexer);
tud_thunk!(get_write_indexer_thunk, get_write_indexer);
tud_thunk!(set_table_metatable_thunk, set_table_metatable);
tud_thunk!(get_metatable_thunk, get_metatable);
tud_thunk!(set_function_parameters_thunk, set_function_parameters);
tud_thunk!(get_function_parameters_thunk, get_function_parameters);
tud_thunk!(set_function_returns_thunk, set_function_returns);
tud_thunk!(get_function_returns_thunk, get_function_returns);
tud_thunk!(set_function_generics_thunk, set_function_generics);
tud_thunk!(get_function_generics_thunk, get_function_generics);
tud_thunk!(get_components_thunk, get_components);
tud_thunk!(get_read_parent_thunk, get_read_parent);
tud_thunk!(get_write_parent_thunk, get_write_parent);
tud_thunk!(get_generic_name_thunk, get_generic_name);
tud_thunk!(get_generic_is_pack_thunk, get_generic_is_pack);
tud_thunk!(is_equal_to_type_thunk, is_equal_to_type);
tud_thunk!(is_subtype_of_thunk, is_subtype_of);
tud_thunk!(type_userdata_index_thunk, type_userdata_index);

/// `extern "C-unwind"` destructor thunk for `deallocTypeUserData`. Its
/// signature matches the VM's `LuaDestructor`
/// (`extern "C-unwind" fn(*mut vm::LuaState, *mut c_void)`) exactly, so it can
/// be registered without any pointer-type adaptation.
unsafe extern "C-unwind" fn dealloc_type_user_data_thunk(
  l: *mut lua_state::LuaState,
  data: *mut c_void,
) {
  dealloc_type_user_data(l as *mut LuaState, data);
}

/// `luaL_Reg typeUserdataMethods[]`：type userdata 的方法表。
///
/// 原实现另有 `typeUserdataMethods_DEPRECATED[]` 并按 `LuauTypeFunctionRobustness`
/// 二选一；两表的项目与函数映射逐字相同（DEPRECATED 只是在 `name/ispack` 之前重复
/// 了一遍 `setgenerics/generics`，再补三个空名哨兵位），注册进同一张 Lua 表的结果
/// 完全一致，故该开关分支是纯噪声，合并为单表。
static TYPE_USERDATA_METHODS: [LuaLReg; 28] = [
  LuaLReg::new(b"is", check_tag_thunk),
  // Negation type methods
  LuaLReg::new(b"inner", get_negated_value_thunk),
  // Singleton type methods
  LuaLReg::new(b"value", get_singleton_value_thunk),
  // Table type methods
  LuaLReg::new(b"setproperty", set_table_prop_thunk),
  LuaLReg::new(b"setreadproperty", set_read_table_prop_thunk),
  LuaLReg::new(b"setwriteproperty", set_write_table_prop_thunk),
  LuaLReg::new(b"readproperty", read_table_prop_thunk),
  LuaLReg::new(b"writeproperty", write_table_prop_thunk),
  LuaLReg::new(b"properties", get_props_thunk),
  LuaLReg::new(b"setindexer", set_table_indexer_thunk),
  LuaLReg::new(b"setreadindexer", set_table_read_indexer_thunk),
  LuaLReg::new(b"setwriteindexer", set_table_write_indexer_thunk),
  LuaLReg::new(b"indexer", get_indexer_thunk),
  LuaLReg::new(b"readindexer", get_read_indexer_thunk),
  LuaLReg::new(b"writeindexer", get_write_indexer_thunk),
  LuaLReg::new(b"setmetatable", set_table_metatable_thunk),
  LuaLReg::new(b"metatable", get_metatable_thunk),
  // Function type methods
  LuaLReg::new(b"setparameters", set_function_parameters_thunk),
  LuaLReg::new(b"parameters", get_function_parameters_thunk),
  LuaLReg::new(b"setreturns", set_function_returns_thunk),
  LuaLReg::new(b"returns", get_function_returns_thunk),
  LuaLReg::new(b"setgenerics", set_function_generics_thunk),
  LuaLReg::new(b"generics", get_function_generics_thunk),
  // Union and Intersection type methods
  LuaLReg::new(b"components", get_components_thunk),
  // Extern type methods
  LuaLReg::new(b"readparent", get_read_parent_thunk),
  LuaLReg::new(b"writeparent", get_write_parent_thunk),
  // Generic type methods
  LuaLReg::new(b"name", get_generic_name_thunk),
  LuaLReg::new(b"ispack", get_generic_is_pack_thunk),
];

/// # Safety
///
/// `l` 必须指向当前存活、可执行 Lua C API 的 `LuaState`（对应 C++
/// `registerTypeUserData(LuaState* l)` 的入参契约：VM 已构造完毕且栈可
/// push/setfield），调用期间不得有其它线程或借用并发改写该 VM 的全局状态；
/// 本函数只在 VM 初始化的单线程阶段调用一次。
pub unsafe fn register_type_user_data(l: *mut LuaState) {
  // Safety: 块内所有解引用都发生在 ulua-vm 的 `lua_l_*`/`lua_*` unsafe fn 中，其
  // 前置条件即上面 `l` 的存活契约；`vm_l` 是 `l` 的地址透传拷贝（本 crate 的
  // `LuaState` 为不透明结构体，cast 不改变指针值）。方法表是 `static` 常量数组，
  // 名字字节串与 thunk 函数指针
  // 均为 'static，VM 注册表留存它们无悬垂。注册的 thunk 只在 VM 回调时运行，
  // 届时 VM 保证自身 `l` 参数有效（见 tud_thunk 内证成）。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;

    // Create and register metatable for type userdata
    // luaL_newmetatable(l, "type");
    lua_l_newmetatable(vm_l, TYPE.as_ptr().cast());

    // lua_pushstring(l, "type"); lua_setfield(l, -2, "__type");
    lua_pushstring(vm_l, TYPE.as_ptr().cast());
    lua_setfield(vm_l, -2, FIELD_TYPE_TAG.as_ptr().cast());

    // Protect metatable from being changed
    // lua_pushstring(l, "The metatable is locked"); lua_setfield(l, -2, "__metatable");
    lua_pushstring(vm_l, METATABLE_LOCKED.as_ptr().cast());
    lua_setfield(vm_l, -2, FIELD_METATABLE.as_ptr().cast());

    // lua_pushcfunction(l, isEqualToType, "__eq"); lua_setfield(l, -2, "__eq");
    LUA_PUSHCFUNCTION(vm_l, Some(is_equal_to_type_thunk), FIELD_EQ.as_ptr().cast());
    lua_setfield(vm_l, -2, FIELD_EQ.as_ptr().cast());

    // Indexing will be a dynamic function because some type fields are dynamic
    // lua_newtable(l);
    lua_newtable(vm_l);
    // luaL_register(l, nullptr, typeUserdataMethods);
    lua_l_register(vm_l, null(), &TYPE_USERDATA_METHODS);

    // if (FFlag::LuauUdtfTypeIsSubtypeOf)
    if fflag::LuauUdtfTypeIsSubtypeOf.get() {
      // lua_pushcfunction(l, isSubtypeOf, "issubtypeof"); lua_setfield(l, -2, "issubtypeof");
      LUA_PUSHCFUNCTION(
        vm_l,
        Some(is_subtype_of_thunk),
        METHOD_IS_SUBTYPE_OF.as_ptr().cast(),
      );
      lua_setfield(vm_l, -2, METHOD_IS_SUBTYPE_OF.as_ptr().cast());
    }

    // lua_setreadonly(l, -1, true);
    lua_setreadonly(vm_l, -1, 1);
    // LUA_PUSHCCLOSURE(l, typeUserdataIndex, "__index", 1);
    lua_pushcclosure(
      vm_l,
      Some(type_userdata_index_thunk),
      FIELD_INDEX_CLOSURE.as_ptr().cast(),
      1,
    );
    // lua_setfield(l, -2, "__index");
    lua_setfield(vm_l, -2, FIELD_INDEX_CLOSURE.as_ptr().cast());

    // lua_setreadonly(l, -1, true);
    lua_setreadonly(vm_l, -1, 1);
    // lua_pop(l, 1);
    lua_pop(vm_l, 1);

    // Sets up a destructor for the type userdata.
    // lua_setuserdatadtor(l, kTypeUserdataTag, deallocTypeUserData);
    lua_setuserdatadtor(
      vm_l,
      K_TYPE_USERDATA_TAG,
      Some(dealloc_type_user_data_thunk),
    );
  }
}
