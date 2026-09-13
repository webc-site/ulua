//! `lua_State` as seen by CodeGen — the VM's own type, not an opaque twin
//! (CodeGen executes against real VM state).

pub use ulua_vm::records::lua_state::lua_State;
