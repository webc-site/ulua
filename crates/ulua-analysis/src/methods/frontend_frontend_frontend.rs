//! C++ `Frontend::Frontend(SolverMode mode, FileResolver*, ConfigResolver*,
//! FrontendOptions options)` (`Analysis/src/Frontend.cpp:435-446`).
use alloc::vec::Vec;
use core::{
  ptr::{NonNull, null_mut},
  sync::atomic::AtomicI32,
};
use std::collections::HashMap;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    builtin_types::BuiltinTypes, config_resolver::ConfigResolver, file_resolver::FileResolver,
    frontend::Frontend, frontend_module_resolver::FrontendModuleResolver,
    frontend_options::FrontendOptions, global_types::GlobalTypes,
    internal_error_reporter::InternalErrorReporter,
  },
};
impl Frontend {
  /// Owned constructor for `Frontend::Frontend(SolverMode, FileResolver*,
  /// ConfigResolver*, FrontendOptions)`.
  ///
  /// The C++ member-init list wires several self-referential pointers:
  /// `builtinTypes(NotNull{&builtinTypes_})`, `moduleResolver(this)`,
  /// `moduleResolverForAutocomplete(this)`, and the two `GlobalTypes` members
  /// capture `builtinTypes` (i.e. `&builtinTypes_`). None of those can be set
  /// here because the returned value is moved into its final slot, so they are
  /// left null/dangling-free and wired by [`Frontend::wire_self_pointers`]
  /// once the `Frontend` lives at a stable address.
  ///
  /// `builtinTypes_`'s arena is heap-boxed, so moving the `BuiltinTypes` value
  /// itself is sound; `GlobalTypes::new` runs its arena mutations through the
  /// temporary `&builtinTypes_` pointer (valid for the duration of this call),
  /// and only the cached `builtin_types` back-pointer is re-pointed afterward.
  pub fn frontend_solver_mode_file_resolver_config_resolver_frontend_options(
    mode: SolverMode,
    file_resolver: *mut FileResolver,
    config_resolver: *mut ConfigResolver,
    options: FrontendOptions,
  ) -> Self {
    // useNewLuauSolver(mode)
    let use_new_luau_solver = AtomicI32::new(mode as i32);

    // builtinTypes_ is default-constructed; builtinTypes = NotNull{&builtinTypes_}.
    let mut builtin_types_ = BuiltinTypes::new();
    let bt_ptr: *mut BuiltinTypes = &mut builtin_types_;

    // getLuauSolverMode() == useNewLuauSolver.load() == mode.
    let solver_mode = mode;

    // globals(builtinTypes, getLuauSolverMode())
    // globalsForAutocomplete(builtinTypes, getLuauSolverMode())
    let globals = GlobalTypes::new(unsafe { NonNull::new_unchecked(bt_ptr) }, solver_mode);
    let globals_for_autocomplete =
      GlobalTypes::new(unsafe { NonNull::new_unchecked(bt_ptr) }, solver_mode);

    Frontend {
      use_new_luau_solver,
      environments: HashMap::new(),
      builtin_definitions: HashMap::new(),
      builtin_types_,
      // builtinTypes(NotNull{&builtinTypes_}) — wired in wire_self_pointers.
      builtin_types: null_mut(),
      file_resolver,
      // moduleResolver(this) / moduleResolverForAutocomplete(this) — wired below.
      module_resolver: FrontendModuleResolver::new(null_mut()),
      module_resolver_for_autocomplete: FrontendModuleResolver::new(null_mut()),
      globals,
      globals_for_autocomplete,
      config_resolver,
      options,
      ice_handler: InternalErrorReporter::default(),
      prepare_module_scope: None,
      write_json_log: None,
      source_nodes: HashMap::new(),
      source_modules: HashMap::new(),
      require_trace: HashMap::new(),
      stats: Default::default(),
      module_queue: Vec::new(),
    }
  }

  /// Wires the self-referential pointers the C++ `Frontend` member-init list
  /// sets in place: `builtinTypes(&builtinTypes_)`, the two `GlobalTypes`'
  /// captured `builtinTypes`, and `moduleResolver(this)` /
  /// `moduleResolverForAutocomplete(this)`.
  ///
  /// Must be called once the `Frontend` is at its final address and before any
  /// use of `builtin_types`, `globals.builtin_types`, or the module resolvers.
  ///
  /// # Safety
  /// The `Frontend` must not be moved after this call, or the wired pointers
  /// dangle.
  pub unsafe fn wire_self_pointers(&mut self) {
    unsafe {
      let bt_ptr: *mut BuiltinTypes = &mut self.builtin_types_;
      self.builtin_types = bt_ptr;
      self.globals.builtin_types = NonNull::new_unchecked(bt_ptr);
      self.globals_for_autocomplete.builtin_types = NonNull::new_unchecked(bt_ptr);

      let self_ptr: *mut Frontend = self;
      self.module_resolver.frontend = self_ptr;
      self.module_resolver_for_autocomplete.frontend = self_ptr;
    }
  }
}
