//! Faithful port of `void registerTypeUserData(lua_State* l)`
//! (Analysis/src/TypeFunctionRuntime.cpp:1932-2064).
//!
//! Creates and registers the `"type"` metatable for type userdata: installs
//! `__type`, `__metatable`, `__eq`, a method table (gated on
//! `LuauTypeFunctionRobustness`), an optional `issubtypeof` method (gated on
//! `LuauUdtfTypeIsSubtypeOf`), the dynamic `__index` Closure, and the userdata
//! destructor.
// `kTypeUserdataTag` (Analysis/src/TypeFunctionRuntime.cpp:250).
use core::{ffi::c_void, ptr::null};

use ulua_common::FFlag;
use ulua_vm::{
  functions::{
    lua_l_newmetatable::lua_l_newmetatable, lua_l_register::lua_l_register,
    lua_pushstring::lua_pushstring, lua_setfield::lua_setfield, lua_setreadonly::lua_setreadonly,
    lua_setuserdatadtor::lua_setuserdatadtor,
  },
  macros::{
    LUA_PUSHCCLOSURE::LUA_PUSHCCLOSURE, lua_newtable::lua_newtable, lua_pop::lua_pop,
    lua_pushcfunction::LUA_PUSHCFUNCTION,
  },
  records::{lua_l_reg::LuaLReg, lua_state},
};

use crate::{
  functions::{
    check_tag::check_tag, dealloc_type_user_data::dealloc_type_user_data,
    get_components::get_components, get_function_generics::get_function_generics,
    get_function_parameters::get_function_parameters, get_function_returns::get_function_returns,
    get_generic_is_pack::get_generic_is_pack, get_generic_name::get_generic_name,
    get_indexer::get_indexer, get_metatable_type_function_runtime::get_metatable,
    get_negated_value::get_negated_value, get_props::get_props, get_read_indexer::get_read_indexer,
    get_read_parent::get_read_parent, get_singleton_value::get_singleton_value,
    get_write_indexer::get_write_indexer, get_write_parent::get_write_parent,
    is_equal_to_type::is_equal_to_type, is_subtype_of::is_subtype_of,
    read_table_prop::read_table_prop, set_function_generics::set_function_generics,
    set_function_parameters::set_function_parameters, set_function_returns::set_function_returns,
    set_read_table_prop::set_read_table_prop, set_table_indexer::set_table_indexer,
    set_table_metatable::set_table_metatable, set_table_prop::set_table_prop,
    set_table_read_indexer::set_table_read_indexer,
    set_table_write_indexer::set_table_write_indexer, set_write_table_prop::set_write_table_prop,
    type_userdata_index::type_userdata_index, write_table_prop::write_table_prop,
  },
  type_aliases::lua_state::LuaState,
};
const K_TYPE_USERDATA_TAG: i32 = 42;

/// Generates a `LuaCfunction`-shaped thunk forwarding to an analysis-level
/// function declared over the `c_void` `lua_State` alias.
macro_rules! tud_thunk {
  ($thunk:ident, $real:path) => {
    unsafe extern "C-unwind" fn $thunk(
      l: *mut ulua_vm::records::lua_state::LuaState,
    ) -> core::ffi::c_int {
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

/// `extern "C"` destructor thunk for `deallocTypeUserData`. The VM's
/// `LuaDestructor` is `extern "C-unwind" fn(*mut vm::lua_State, *mut c_void)`.
unsafe extern "C-unwind" fn dealloc_type_user_data_thunk(
  l: *mut lua_state::LuaState,
  data: *mut c_void,
) {
  dealloc_type_user_data(l as *mut LuaState, data);
}

/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn register_type_user_data(l: *mut LuaState) {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;

    // luaL_Reg typeUserdataMethods_DEPRECATED[] = { ... {nullptr, nullptr} };
    let type_userdata_methods_deprecated: [LuaLReg; 34] = [
      LuaLReg {
        name: c"is".as_ptr(),
        func: Some(check_tag_thunk),
      },
      // Negation type methods
      LuaLReg {
        name: c"inner".as_ptr(),
        func: Some(get_negated_value_thunk),
      },
      // Singleton type methods
      LuaLReg {
        name: c"value".as_ptr(),
        func: Some(get_singleton_value_thunk),
      },
      // Table type methods
      LuaLReg {
        name: c"setproperty".as_ptr(),
        func: Some(set_table_prop_thunk),
      },
      LuaLReg {
        name: c"setreadproperty".as_ptr(),
        func: Some(set_read_table_prop_thunk),
      },
      LuaLReg {
        name: c"setwriteproperty".as_ptr(),
        func: Some(set_write_table_prop_thunk),
      },
      LuaLReg {
        name: c"readproperty".as_ptr(),
        func: Some(read_table_prop_thunk),
      },
      LuaLReg {
        name: c"writeproperty".as_ptr(),
        func: Some(write_table_prop_thunk),
      },
      LuaLReg {
        name: c"properties".as_ptr(),
        func: Some(get_props_thunk),
      },
      LuaLReg {
        name: c"setindexer".as_ptr(),
        func: Some(set_table_indexer_thunk),
      },
      LuaLReg {
        name: c"setreadindexer".as_ptr(),
        func: Some(set_table_read_indexer_thunk),
      },
      LuaLReg {
        name: c"setwriteindexer".as_ptr(),
        func: Some(set_table_write_indexer_thunk),
      },
      LuaLReg {
        name: c"indexer".as_ptr(),
        func: Some(get_indexer_thunk),
      },
      LuaLReg {
        name: c"readindexer".as_ptr(),
        func: Some(get_read_indexer_thunk),
      },
      LuaLReg {
        name: c"writeindexer".as_ptr(),
        func: Some(get_write_indexer_thunk),
      },
      LuaLReg {
        name: c"setmetatable".as_ptr(),
        func: Some(set_table_metatable_thunk),
      },
      LuaLReg {
        name: c"metatable".as_ptr(),
        func: Some(get_metatable_thunk),
      },
      // Function type methods
      LuaLReg {
        name: c"setparameters".as_ptr(),
        func: Some(set_function_parameters_thunk),
      },
      LuaLReg {
        name: c"parameters".as_ptr(),
        func: Some(get_function_parameters_thunk),
      },
      LuaLReg {
        name: c"setreturns".as_ptr(),
        func: Some(set_function_returns_thunk),
      },
      LuaLReg {
        name: c"returns".as_ptr(),
        func: Some(get_function_returns_thunk),
      },
      LuaLReg {
        name: c"setgenerics".as_ptr(),
        func: Some(set_function_generics_thunk),
      },
      LuaLReg {
        name: c"generics".as_ptr(),
        func: Some(get_function_generics_thunk),
      },
      // Union and Intersection type methods
      LuaLReg {
        name: c"components".as_ptr(),
        func: Some(get_components_thunk),
      },
      // Extern type methods
      LuaLReg {
        name: c"readparent".as_ptr(),
        func: Some(get_read_parent_thunk),
      },
      LuaLReg {
        name: c"writeparent".as_ptr(),
        func: Some(get_write_parent_thunk),
      },
      // Function type methods (cont.)
      LuaLReg {
        name: c"setgenerics".as_ptr(),
        func: Some(set_function_generics_thunk),
      },
      LuaLReg {
        name: c"generics".as_ptr(),
        func: Some(get_function_generics_thunk),
      },
      // Generic type methods
      LuaLReg {
        name: c"name".as_ptr(),
        func: Some(get_generic_name_thunk),
      },
      LuaLReg {
        name: c"ispack".as_ptr(),
        func: Some(get_generic_is_pack_thunk),
      },
      LuaLReg {
        name: null(),
        func: None,
      },
      // Padding to keep array length stable with the duplicate-entry layout
      // above (C++ relies on the {nullptr,nullptr} sentinel; trailing entries
      // past the sentinel are never read).
      LuaLReg {
        name: null(),
        func: None,
      },
      LuaLReg {
        name: null(),
        func: None,
      },
      LuaLReg {
        name: null(),
        func: None,
      },
    ];

    // luaL_Reg typeUserdataMethods[] = { ... {nullptr, nullptr} };
    let type_userdata_methods: [LuaLReg; 31] = [
      LuaLReg {
        name: c"is".as_ptr(),
        func: Some(check_tag_thunk),
      },
      // Negation type methods
      LuaLReg {
        name: c"inner".as_ptr(),
        func: Some(get_negated_value_thunk),
      },
      // Singleton type methods
      LuaLReg {
        name: c"value".as_ptr(),
        func: Some(get_singleton_value_thunk),
      },
      // Table type methods
      LuaLReg {
        name: c"setproperty".as_ptr(),
        func: Some(set_table_prop_thunk),
      },
      LuaLReg {
        name: c"setreadproperty".as_ptr(),
        func: Some(set_read_table_prop_thunk),
      },
      LuaLReg {
        name: c"setwriteproperty".as_ptr(),
        func: Some(set_write_table_prop_thunk),
      },
      LuaLReg {
        name: c"readproperty".as_ptr(),
        func: Some(read_table_prop_thunk),
      },
      LuaLReg {
        name: c"writeproperty".as_ptr(),
        func: Some(write_table_prop_thunk),
      },
      LuaLReg {
        name: c"properties".as_ptr(),
        func: Some(get_props_thunk),
      },
      LuaLReg {
        name: c"setindexer".as_ptr(),
        func: Some(set_table_indexer_thunk),
      },
      LuaLReg {
        name: c"setreadindexer".as_ptr(),
        func: Some(set_table_read_indexer_thunk),
      },
      LuaLReg {
        name: c"setwriteindexer".as_ptr(),
        func: Some(set_table_write_indexer_thunk),
      },
      LuaLReg {
        name: c"indexer".as_ptr(),
        func: Some(get_indexer_thunk),
      },
      LuaLReg {
        name: c"readindexer".as_ptr(),
        func: Some(get_read_indexer_thunk),
      },
      LuaLReg {
        name: c"writeindexer".as_ptr(),
        func: Some(get_write_indexer_thunk),
      },
      LuaLReg {
        name: c"setmetatable".as_ptr(),
        func: Some(set_table_metatable_thunk),
      },
      LuaLReg {
        name: c"metatable".as_ptr(),
        func: Some(get_metatable_thunk),
      },
      // Function type methods
      LuaLReg {
        name: c"setparameters".as_ptr(),
        func: Some(set_function_parameters_thunk),
      },
      LuaLReg {
        name: c"parameters".as_ptr(),
        func: Some(get_function_parameters_thunk),
      },
      LuaLReg {
        name: c"setreturns".as_ptr(),
        func: Some(set_function_returns_thunk),
      },
      LuaLReg {
        name: c"returns".as_ptr(),
        func: Some(get_function_returns_thunk),
      },
      LuaLReg {
        name: c"setgenerics".as_ptr(),
        func: Some(set_function_generics_thunk),
      },
      LuaLReg {
        name: c"generics".as_ptr(),
        func: Some(get_function_generics_thunk),
      },
      // Union and Intersection type methods
      LuaLReg {
        name: c"components".as_ptr(),
        func: Some(get_components_thunk),
      },
      // Extern type methods
      LuaLReg {
        name: c"readparent".as_ptr(),
        func: Some(get_read_parent_thunk),
      },
      LuaLReg {
        name: c"writeparent".as_ptr(),
        func: Some(get_write_parent_thunk),
      },
      // Generic type methods
      LuaLReg {
        name: c"name".as_ptr(),
        func: Some(get_generic_name_thunk),
      },
      LuaLReg {
        name: c"ispack".as_ptr(),
        func: Some(get_generic_is_pack_thunk),
      },
      LuaLReg {
        name: null(),
        func: None,
      },
      // Padding (see note above).
      LuaLReg {
        name: null(),
        func: None,
      },
      LuaLReg {
        name: null(),
        func: None,
      },
    ];

    // Create and register metatable for type userdata
    // luaL_newmetatable(l, "type");
    lua_l_newmetatable(vm_l, c"type".as_ptr());

    // lua_pushstring(l, "type"); lua_setfield(l, -2, "__type");
    lua_pushstring(vm_l, c"type".as_ptr());
    lua_setfield(vm_l, -2, c"__type".as_ptr());

    // Protect metatable from being changed
    // lua_pushstring(l, "The metatable is locked"); lua_setfield(l, -2, "__metatable");
    lua_pushstring(vm_l, c"The metatable is locked".as_ptr());
    lua_setfield(vm_l, -2, c"__metatable".as_ptr());

    // lua_pushcfunction(l, isEqualToType, "__eq"); lua_setfield(l, -2, "__eq");
    LUA_PUSHCFUNCTION(vm_l, Some(is_equal_to_type_thunk), c"__eq".as_ptr());
    lua_setfield(vm_l, -2, c"__eq".as_ptr());

    // Indexing will be a dynamic function because some type fields are dynamic
    // lua_newtable(l);
    lua_newtable(vm_l);
    // luaL_register(l, nullptr, FFlag::LuauTypeFunctionRobustness ? typeUserdataMethods : typeUserdataMethods_DEPRECATED);
    if FFlag::LuauTypeFunctionRobustness.get() {
      lua_l_register(vm_l, null(), type_userdata_methods.as_ptr());
    } else {
      lua_l_register(vm_l, null(), type_userdata_methods_deprecated.as_ptr());
    }

    // if (FFlag::LuauUdtfTypeIsSubtypeOf)
    if FFlag::LuauUdtfTypeIsSubtypeOf.get() {
      // lua_pushcfunction(l, isSubtypeOf, "issubtypeof"); lua_setfield(l, -2, "issubtypeof");
      LUA_PUSHCFUNCTION(vm_l, Some(is_subtype_of_thunk), c"issubtypeof".as_ptr());
      lua_setfield(vm_l, -2, c"issubtypeof".as_ptr());
    }

    // lua_setreadonly(l, -1, true);
    lua_setreadonly(vm_l, -1, 1);
    // LUA_PUSHCCLOSURE(l, typeUserdataIndex, "__index", 1);
    LUA_PUSHCCLOSURE(
      vm_l,
      Some(type_userdata_index_thunk),
      c"__index".as_ptr(),
      1,
    );
    // lua_setfield(l, -2, "__index");
    lua_setfield(vm_l, -2, c"__index".as_ptr());

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
