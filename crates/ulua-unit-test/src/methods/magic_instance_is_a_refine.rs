use ulua_analysis::records::magic_refinement_context::MagicRefinementContext;

use crate::records::magic_instance_is_a::MagicInstanceIsA;

impl MagicInstanceIsA {
  pub fn refine(&mut self, _ctx: &MagicRefinementContext) {
    // Dead duplicate skeleton node: the real refine logic is the free
    // `magic_instance_is_a_refine` wired into the `MagicFunction` vtable in
    // `crates/luau-unit-test/src/functions/make_magic_instance_is_a.rs`. This
    // method has no call site.
    unreachable!(
      "canonical refine lives in functions/make_magic_instance_is_a.rs (vtable-built MagicFunction); this skeleton node is unused"
    );
  }
}
