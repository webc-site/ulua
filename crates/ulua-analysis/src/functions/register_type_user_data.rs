//! Faithful port of `void registerTypeUserData(LuaState* l)`
//! (Analysis/src/TypeFunctionRuntime.cpp:1932-2064).
//!
//! Creates and registers the `"type"` metatable for type userdata: installs
//! `__type`, `__metatable`, `__eq`, the method table, an optional `issubtypeof`
//! method (gated on `LuauUdtfTypeIsSubtypeOf`), the dynamic `__index` closure,
//! and the userdata destructor.
// `kTypeUserdataTag` (Analysis/src/TypeFunctionRuntime.cpp:250).

use core::ffi::c_void;

use ulua_common::fflag;
use ulua_vm::{
  functions::{lua_l_register::lua_l_register_bytes, lua_setuserdatadtor::lua_setuserdatadtor},
  records::{lua_l_reg::LuaLReg, lua_state, lua_state::LuaState},
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
  macros::c_thunk,
};
const K_TYPE_USERDATA_TAG: i32 = 42;

c_thunk!(check_tag_thunk, check_tag, @ref);
c_thunk!(get_negated_value_thunk, get_negated_value, @ref);
c_thunk!(get_singleton_value_thunk, get_singleton_value, @ref);
c_thunk!(set_table_prop_thunk, set_table_prop, @ref);
c_thunk!(set_read_table_prop_thunk, set_read_table_prop, @ref);
c_thunk!(set_write_table_prop_thunk, set_write_table_prop, @ref);
c_thunk!(read_table_prop_thunk, read_table_prop, @ref);
c_thunk!(write_table_prop_thunk, write_table_prop, @ref);
c_thunk!(get_props_thunk, get_props, @ref);
c_thunk!(set_table_indexer_thunk, set_table_indexer, @ref);
c_thunk!(set_table_read_indexer_thunk, set_table_read_indexer, @ref);
c_thunk!(set_table_write_indexer_thunk, set_table_write_indexer, @ref);
c_thunk!(get_indexer_thunk, get_indexer, @ref);
c_thunk!(get_read_indexer_thunk, get_read_indexer, @ref);
c_thunk!(get_write_indexer_thunk, get_write_indexer, @ref);
c_thunk!(set_table_metatable_thunk, set_table_metatable, @ref);
c_thunk!(get_metatable_thunk, get_metatable, @ref);
c_thunk!(set_function_parameters_thunk, set_function_parameters, @ref);
c_thunk!(get_function_parameters_thunk, get_function_parameters, @ref);
c_thunk!(set_function_returns_thunk, set_function_returns, @ref);
c_thunk!(get_function_returns_thunk, get_function_returns, @ref);
c_thunk!(set_function_generics_thunk, set_function_generics, @ref);
c_thunk!(get_function_generics_thunk, get_function_generics, @ref);
c_thunk!(get_components_thunk, get_components, @ref);
c_thunk!(get_read_parent_thunk, get_read_parent, @ref);
c_thunk!(get_write_parent_thunk, get_write_parent, @ref);
c_thunk!(get_generic_name_thunk, get_generic_name, @ref);
c_thunk!(get_generic_is_pack_thunk, get_generic_is_pack, @ref);
c_thunk!(is_equal_to_type_thunk, is_equal_to_type, @ref);
c_thunk!(is_subtype_of_thunk, is_subtype_of, @ref);
c_thunk!(type_userdata_index_thunk, type_userdata_index, @ref);

/// `extern "C-unwind"` destructor thunk for `deallocTypeUserData`. Its
/// signature matches the VM's `LuaDestructor`
/// (`extern "C-unwind" fn(*mut vm::LuaState, *mut c_void)`) exactly, so it can
/// be registered without any pointer-type adaptation.
unsafe extern "C-unwind" fn dealloc_type_user_data_thunk(
  l: *mut lua_state::LuaState,
  data: *mut c_void,
) {
  // Safety: FFI 边界——VM 只在 userdata 存活且本次析构独占该 state 时回调本 thunk，
  // 故可重建为独占借用；借用窗止于 `dealloc_type_user_data` 返回。
  unsafe { dealloc_type_user_data(&mut *l, data) };
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

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型
/// 承载）：`l` 是当前存活、可执行 Lua C API 的 `LuaState`（对应 C++
/// `registerTypeUserData(LuaState* l)` 的入参契约：VM 已构造完毕且栈可
/// push/setfield），调用期间不得有其它线程或借用并发改写该 VM 的全局状态；
/// 本函数只在 VM 初始化的单线程阶段调用一次。
pub(crate) fn register_type_user_data(l: &mut LuaState) {
  // 方法表是 `static` 常量数组，名字字节串与 thunk 函数指针均为 'static；VM 注册
  // debugname/键时经 intern 当场复制，注册表留存它们无悬垂。注册的 thunk 只在 VM
  // 回调时运行，届时 VM 保证自身 `l` 参数有效（见 `c_thunk!` 展开内证成）。
  {
    // Create and register metatable for type userdata
    // luaL_newmetatable(l, "type");
    l.new_metatable_by_bytes(TYPE);

    // lua_pushstring(l, "type"); lua_setfield(l, -2, "__type");
    l.push_bytes(TYPE);
    l.set_field_bytes(-2, FIELD_TYPE_TAG);

    // Protect metatable from being changed
    // lua_pushstring(l, "The metatable is locked"); lua_setfield(l, -2, "__metatable");
    l.push_bytes(METATABLE_LOCKED);
    l.set_field_bytes(-2, FIELD_METATABLE);

    // lua_pushcfunction(l, isEqualToType, "__eq"); lua_setfield(l, -2, "__eq");
    l.push_c_function(Some(is_equal_to_type_thunk), None);
    l.set_field_bytes(-2, FIELD_EQ);

    // Indexing will be a dynamic function because some type fields are dynamic
    // lua_newtable(l);
    l.new_table();
    // luaL_register(l, nullptr, typeUserdataMethods);
    // （lua_l_register_bytes 与 push_c_function/push_c_closure 均为安全门面，
    // debugname/键由 VM 当场 intern 复制）
    lua_l_register_bytes(l, None, &TYPE_USERDATA_METHODS);

    // if (FFlag::LuauUdtfTypeIsSubtypeOf)
    if fflag::LuauUdtfTypeIsSubtypeOf.get() {
      // lua_pushcfunction(l, isSubtypeOf, "issubtypeof"); lua_setfield(l, -2, "issubtypeof");
      l.push_c_function(Some(is_subtype_of_thunk), Some(METHOD_IS_SUBTYPE_OF));
      l.set_field_bytes(-2, METHOD_IS_SUBTYPE_OF);
    }

    // lua_setreadonly(l, -1, true);
    l.set_readonly(-1, true);
    // LUA_PUSHCCLOSURE(l, typeUserdataIndex, "__index", 1);
    l.push_c_closure(Some(type_userdata_index_thunk), Some(FIELD_INDEX_CLOSURE), 1);
    // lua_setfield(l, -2, "__index");
    l.set_field_bytes(-2, FIELD_INDEX_CLOSURE);

    // lua_setreadonly(l, -1, true);
    l.set_readonly(-1, true);
    // lua_pop(l, 1);
    l.pop(1);

    // Sets up a destructor for the type userdata.
    // lua_setuserdatadtor(l, kTypeUserdataTag, deallocTypeUserData);
    // 登记 dtor：`lua_setuserdatadtor` 收 `&mut`（safe），前面各 `l.…` 具名方法调用
    // 均为短借即还，此处无并存别名；`l` 存活由本函数 `&mut` 接收者承载。
    lua_setuserdatadtor(l, K_TYPE_USERDATA_TAG, Some(dealloc_type_user_data_thunk));
  }
}
