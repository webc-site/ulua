use alloc::{string::String, sync::Arc, vec::Vec};
use core::ptr::{null, null_mut};

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    builtin_types::BuiltinTypes, clone_public_interface::ClonePublicInterface,
    clone_state::CloneState, internal_error::InternalError,
    internal_error_reporter::InternalErrorReporter, module::Module, scope::Scope, txn_log::TxnLog,
    type_error::TypeError,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl Module {
  /// # Safety
  /// 调用方须保证 `builtin_types` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// `void Module::clonePublicInterface(NotNull<BuiltinTypes> builtinTypes, InternalErrorReporter& ice, SolverMode mode)`.
  /// Reference: `Module.cpp:299-348`.
  pub unsafe fn clone_public_interface(
    &mut self,
    builtin_types: *mut BuiltinTypes,
    _ice: &mut InternalErrorReporter,
    mode: SolverMode,
  ) {
    // C++ `CloneState cloneState{builtinTypes};` — declared (parity) but the
    // interface clone is driven by `ClonePublicInterface`'s own substitution.
    let _clone_state = CloneState {
      builtin_types,
      seen_types: DenseHashMap::new(null()),
      seen_type_packs: DenseHashMap::new(null()),
    };

    let module_scope = self.get_module_scope();
    // The C++ mutates the Scope behind the shared_ptr; mirror that by taking a
    // raw pointer to the aliased Scope object.
    let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;

    let return_type: TypePackId = unsafe { (*module_scope_ptr).return_type };
    let varargpack: Option<TypePackId> = if mode == SolverMode::New {
      None
    } else {
      unsafe { (*module_scope_ptr).vararg_pack }
    };

    // C++ `TxnLog log;` — a fresh, empty transaction log.
    let log = TxnLog {
      type_var_changes: DenseHashMap::new(null()),
      type_pack_changes: DenseHashMap::new(null()),
      parent: null_mut(),
      owned_seen: Vec::new(),
      // Empty; lazily owns a box on first `push_seen` (freed on drop).
      shared_seen: null_mut(),
      owned_seen_box: None,
      radioactive: false,
    };
    let mut clone_public_interface =
      unsafe { ClonePublicInterface::new(&log, builtin_types, self as *mut Module, mode) };

    let return_type = clone_public_interface.clone_type_pack(return_type);

    unsafe { (*module_scope_ptr).return_type = return_type };
    if let Some(vp) = varargpack {
      let varargpack = clone_public_interface.clone_type_pack(vp);
      unsafe { (*module_scope_ptr).vararg_pack = Some(varargpack) };
    }

    unsafe {
      for tf in (*module_scope_ptr).exported_type_bindings.values_mut() {
        let cloned = clone_public_interface.clone_type_fun(&*tf);
        *tf = cloned;
      }
    }

    for ty in self.declared_globals.values_mut() {
      *ty = clone_public_interface.clone_type(*ty);
    }

    for tf in self.type_function_aliases.iter_mut() {
      let cloned = clone_public_interface.clone_type_fun(tf);
      **tf = cloned;
    }

    if clone_public_interface.internal_type_escaped {
      self
        .errors
        .push(TypeError::type_error_location_module_name_type_error_data(
          // Not amazing but the best we can do.
          Location::default(),
          self.name.clone(),
          InternalError::new(String::from(
            "An internal type is escaping this module; please report this bug at \
                     https://github.com/luau-lang/luau/issues",
          ))
          .into(),
        ));
    }

    // Copy external stuff over to Module itself
    self.return_type = unsafe { (*module_scope_ptr).return_type };
    self.exported_type_bindings = unsafe { (*module_scope_ptr).exported_type_bindings.clone() };
  }
}
