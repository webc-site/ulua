//! Source: `CLI/src/Repl.cpp:122` (`copts`) — faithful port.
//!
//! Returns `Luau::CompileOptions` seeded from the CLI's `globalOptions`
//! (optimization/debug level), with `type_info_level = 1` and `coverageLevel`
//! derived from whether coverage is active. We return the C-ABI
//! `LuaCompileOptions` (layout-identical to `Luau::CompileOptions`) since that
//! is what `luau_compile` consumes.

use core::ptr::null;

use ulua_compiler::records::lua_compile_options::LuaCompileOptions;

use crate::functions::repl_main::global_options;

pub fn copts() -> LuaCompileOptions {
  let opts = global_options();

  LuaCompileOptions {
    optimization_level: opts.optimization_level,
    debug_level: opts.debug_level,
    type_info_level: 1,
    coverage_level: 0,
    vector_lib: null(),
    vector_ctor: null(),
    vector_type: null(),
    mutable_globals: null(),
    userdata_types: null(),
    libraries_with_known_members: null(),
    library_member_type_cb: None,
    library_member_constant_cb: None,
    disabled_builtins: null(),
  }
}
