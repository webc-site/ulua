//! Node: `cxx:Function:Luau.Conformance:tests/FeedbackVector.test.cpp:78:sealing_inliner`
//! Source: `tests/FeedbackVector.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/FeedbackVector.test.cpp
//! - source_includes:
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file VM/src/lstate.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/FeedbackVector.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item sealingInliner

use core::ptr::null_mut;

use ulua_vm::records::{Closure::Closure, Proto::Proto, lua_state::lua_State};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn sealing_inliner(
  _l: *mut lua_State,
  _caller: *mut Closure,
  _target: *mut Closure,
  _pc: u32,
) -> *mut Proto {
  null_mut()
}
