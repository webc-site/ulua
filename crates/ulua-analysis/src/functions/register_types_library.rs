//! Faithful port of `void registerTypesLibrary(lua_State* l)`
//! (Analysis/src/TypeFunctionRuntime.cpp:1876-1914).
/// Generates a `LuaCfunction`-shaped thunk that forwards to the analysis-level
/// function declared over the `c_void` `lua_State` alias. `LuaCfunction` is
/// `unsafe extern "C-unwind" fn(*mut vm::lua_State) -> c_int`, so each registered function needs a
/// thin bridging thunk.
use core::ptr::null;

use ulua_vm::{
  functions::{lua_l_register::lua_l_register, lua_setfield::lua_setfield},
  macros::lua_pop::lua_pop,
  records::{lua_l_reg::LuaLReg, lua_state},
  type_aliases::lua_c_function::LuaCfunction,
};

use crate::{
  functions::{
    create_any::create_any, create_boolean::create_boolean, create_buffer::create_buffer,
    create_function::create_function, create_generic::create_generic,
    create_intersection::create_intersection, create_negation::create_negation,
    create_never::create_never, create_number::create_number, create_optional::create_optional,
    create_singleton::create_singleton, create_string::create_string, create_table::create_table,
    create_thread::create_thread, create_union::create_union, create_unknown::create_unknown,
    deep_copy::deep_copy,
  },
  type_aliases::lua_state::LuaState,
};
macro_rules! type_lib_thunk {
  ($thunk:ident, $real:path) => {
    unsafe extern "C-unwind" fn $thunk(
      l: *mut ulua_vm::records::lua_state::LuaState,
    ) -> core::ffi::c_int {
      unsafe { $real(l as *mut LuaState) }
    }
  };
}

type_lib_thunk!(create_unknown_thunk, create_unknown);
type_lib_thunk!(create_never_thunk, create_never);
type_lib_thunk!(create_any_thunk, create_any);
type_lib_thunk!(create_boolean_thunk, create_boolean);
type_lib_thunk!(create_number_thunk, create_number);
type_lib_thunk!(create_string_thunk, create_string);
type_lib_thunk!(create_thread_thunk, create_thread);
type_lib_thunk!(create_buffer_thunk, create_buffer);

type_lib_thunk!(create_singleton_thunk, create_singleton);
type_lib_thunk!(create_negation_thunk, create_negation);
type_lib_thunk!(create_union_thunk, create_union);
type_lib_thunk!(create_intersection_thunk, create_intersection);
type_lib_thunk!(create_optional_thunk, create_optional);
type_lib_thunk!(create_table_thunk, create_table);
type_lib_thunk!(create_function_thunk, create_function);
type_lib_thunk!(deep_copy_thunk, deep_copy);
type_lib_thunk!(create_generic_thunk, create_generic);

/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn register_types_library(l: *mut LuaState) {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;

    // luaL_Reg fields[] = { ... {nullptr, nullptr} };
    let fields: [LuaLReg; 9] = [
      LuaLReg {
        name: c"unknown".as_ptr(),
        func: Some(create_unknown_thunk),
      },
      LuaLReg {
        name: c"never".as_ptr(),
        func: Some(create_never_thunk),
      },
      LuaLReg {
        name: c"any".as_ptr(),
        func: Some(create_any_thunk),
      },
      LuaLReg {
        name: c"boolean".as_ptr(),
        func: Some(create_boolean_thunk),
      },
      LuaLReg {
        name: c"number".as_ptr(),
        func: Some(create_number_thunk),
      },
      LuaLReg {
        name: c"string".as_ptr(),
        func: Some(create_string_thunk),
      },
      LuaLReg {
        name: c"thread".as_ptr(),
        func: Some(create_thread_thunk),
      },
      LuaLReg {
        name: c"buffer".as_ptr(),
        func: Some(create_buffer_thunk),
      },
      LuaLReg {
        name: null(),
        func: None,
      },
    ];

    // luaL_Reg methods[] = { ... {nullptr, nullptr} };
    let methods: [LuaLReg; 10] = [
      LuaLReg {
        name: c"singleton".as_ptr(),
        func: Some(create_singleton_thunk),
      },
      LuaLReg {
        name: c"negationof".as_ptr(),
        func: Some(create_negation_thunk),
      },
      LuaLReg {
        name: c"unionof".as_ptr(),
        func: Some(create_union_thunk),
      },
      LuaLReg {
        name: c"intersectionof".as_ptr(),
        func: Some(create_intersection_thunk),
      },
      LuaLReg {
        name: c"optional".as_ptr(),
        func: Some(create_optional_thunk),
      },
      LuaLReg {
        name: c"newtable".as_ptr(),
        func: Some(create_table_thunk),
      },
      LuaLReg {
        name: c"newfunction".as_ptr(),
        func: Some(create_function_thunk),
      },
      LuaLReg {
        name: c"copy".as_ptr(),
        func: Some(deep_copy_thunk),
      },
      LuaLReg {
        name: c"generic".as_ptr(),
        func: Some(create_generic_thunk),
      },
      LuaLReg {
        name: null(),
        func: None,
      },
    ];

    // luaL_register(l, "types", methods);
    lua_l_register(vm_l, c"types".as_ptr(), methods.as_ptr());

    // Set fields for type userdata
    // for (luaL_Reg* l = fields; l->name; l++)
    let mut i = 0usize;
    while !fields[i].name.is_null() {
      // l->func(l);
      let func: LuaCfunction = fields[i].func;
      (func.unwrap())(vm_l);
      // lua_setfield(l, -2, l->name);
      lua_setfield(vm_l, -2, fields[i].name);
      i += 1;
    }

    // lua_pop(l, 1);
    lua_pop(vm_l, 1);
  }
}
