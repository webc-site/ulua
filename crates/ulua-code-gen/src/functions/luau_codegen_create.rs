use ulua_vm::records::lua_state::lua_State;

pub fn luau_codegen_create(_l: *mut lua_State) {
  // The Rust port does not link Luau's native C++ codegen context creation yet.
}
