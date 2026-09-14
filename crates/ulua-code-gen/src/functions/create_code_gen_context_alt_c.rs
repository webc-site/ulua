use core::ffi::c_void;

use crate::type_aliases::{allocation_callback::AllocationCallback, lua_state::lua_State};

/// Create a standalone native-codegen context and wire its execution callbacks.
///
/// **Out of scope.** Native code *execution* is not part of ulua's validated
/// surface — the bytecode interpreter is the execution oracle (see
/// docs/CONFORMANCE.md). The C++ setup (`StandaloneCodeGenContext`,
/// `BaseCodeGenContext::initHeaderFunctions`, `initializeExecutionCallbacks`) was
/// never ported to Rust; it survived only as `extern` declarations of the
/// original C++ mangled symbols, which have no implementation to link against.
/// lld dead-code-eliminated those phantom references on Linux/macOS, but the MSVC
/// linker kept them and failed ("unresolved external symbol"). Replacing the
/// phantom externs with an explicit stub keeps the symbol set honest and lets the
/// workspace link on every platform.
pub fn create_lua_state_usize_usize_allocation_callback_void(
  _l: *mut lua_State,
  _block_size: usize,
  _max_total_size: usize,
  _allocation_callback: *mut AllocationCallback,
  _allocation_callback_context: *mut c_void,
) {
  unimplemented!(
    "ulua does not execute JIT-compiled native code (out of scope; see docs/CONFORMANCE.md)"
  )
}
