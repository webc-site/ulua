//! Node: `cxx:Record:Luau.Compiler:Compiler/include/luacode.h:23:lua_CompileOptions`
//!
//! Faithful `#[repr(C)]` port of the public `lua_CompileOptions` struct passed
//! across the C ABI to `luau_compile`. Field defaults (optimization_level=1,
//! debug_level=1, …) are documented in the Header but applied by `luau_compile`
//! when `options` is null, not by the struct, so this is a plain layout type.

use core::ffi::c_char;

use crate::type_aliases::{
  lua_library_member_constant_callback::LuaLibraryMemberConstantCallback,
  lua_library_member_type_callback::LuaLibraryMemberTypeCallback,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LuaCompileOptions {
  pub optimization_level: i32,
  pub debug_level: i32,
  pub type_info_level: i32,
  pub coverage_level: i32,

  pub vector_lib: *const c_char,
  pub vector_ctor: *const c_char,

  pub vector_type: *const c_char,

  pub mutable_globals: *const *const c_char,

  pub userdata_types: *const *const c_char,

  pub libraries_with_known_members: *const *const c_char,
  pub library_member_type_cb: LuaLibraryMemberTypeCallback,
  pub library_member_constant_cb: LuaLibraryMemberConstantCallback,

  pub disabled_builtins: *const *const c_char,
}
