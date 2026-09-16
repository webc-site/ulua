use ulua_analysis::{
  functions::{attach_tag_type::attach_tag, freeze::freeze, unfreeze::unfreeze},
  records::frontend::Frontend,
};
use ulua_common::FFlag;

pub fn autocomplete_attach_require_call_tag(frontend: &mut Frontend) {
  let globals = if !FFlag::DebugLuauForceOldSolver.get() {
    &mut frontend.globals
  } else {
    &mut frontend.globals_for_autocomplete
  };

  let require = globals
    .global_scope()
    .linear_search_for_binding(&String::from("require"), true)
    .expect("expected require binding");

  unfreeze(globals.global_types_mut());
  attach_tag(require.type_id, "RequireCall");
  freeze(globals.global_types_mut());
}
