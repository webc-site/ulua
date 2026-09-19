use core::ptr::null_mut;

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::{freeze::freeze, register_builtin_globals::register_builtin_globals},
  records::{
    file_resolver::FileResolver, frontend::Frontend, frontend_options::FrontendOptions,
    null_file_resolver::NullFileResolver, null_module_resolver::NullModuleResolver,
  },
};
use ulua_common::fflag;
use ulua_vm::{
  functions::lua_setfield::lua_setfield,
  macros::{lua_newtable::lua_newtable, lua_setglobal::lua_setglobal},
  records::lua_state::lua_State,
};

use crate::common::functions::populate_rtti::populate_rtti;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_types_setup(l: *mut lua_State) {
  unsafe {
    let _module_resolver = NullModuleResolver::new();
    let mut file_resolver = NullFileResolver::new();
    let mode = if fflag::DebugLuauForceOldSolver.get() {
      SolverMode::Old
    } else {
      SolverMode::New
    };

    let mut frontend =
      Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
        mode,
        &mut file_resolver as *mut dyn FileResolver,
        null_mut(),
        FrontendOptions::default(),
      );
    frontend.wire_self_pointers();

    let frontend_ptr = &mut frontend as *mut Frontend;
    register_builtin_globals(&mut *frontend_ptr, &mut (*frontend_ptr).globals, false);
    freeze((*frontend_ptr).globals.global_types_mut());

    lua_newtable(l);

    let global_scope = (*frontend_ptr).globals.global_scope();
    for (name, binding) in &global_scope.bindings {
      populate_rtti(l, binding.type_id);
      // Symbol::c_str 已随其他代理重构移除；AstName.value 即 NUL 结尾指针。
      lua_setfield(l, -2, name.ast_name().value);
    }

    lua_setglobal(l, c"RTTI".as_ptr());
  }
}
