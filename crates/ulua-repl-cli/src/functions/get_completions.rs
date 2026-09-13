use ulua_vm::type_aliases::lua_state::lua_State;

use crate::functions::complete_indexer::complete_indexer;

pub unsafe fn get_completions(
  l: *mut lua_State,
  edit_buffer: &str,
  add_completion_callback: &dyn Fn(&str, &str),
) {
  unsafe {
    complete_indexer(l, edit_buffer, add_completion_callback);
  }
}
