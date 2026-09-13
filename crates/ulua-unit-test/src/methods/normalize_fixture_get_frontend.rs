//! @interface-stub
use alloc::sync::Arc;

use ulua_analysis::records::{frontend::Frontend, normalizer::Normalizer, scope::Scope};

use crate::{
  functions::register_hidden_types::register_hidden_types,
  records::normalize_fixture::NormalizeFixture,
};
impl NormalizeFixture {
  pub fn get_frontend(&mut self) -> &mut Frontend {
    if self.base.frontend.is_none() || self.normalizer.is_none() {
      let solver_mode = {
        let frontend = self.base.get_frontend();
        register_hidden_types(frontend);
        frontend.get_luau_solver_mode()
      };

      let builtin_types = self.base.builtin_types;
      assert!(
        !builtin_types.is_null(),
        "NormalizeFixture::get_frontend expected builtin types"
      );

      let any_type_pack = unsafe { (*builtin_types).any_type_pack };
      self.global_scope = Some(Arc::new(Scope::scope_type_pack_id(any_type_pack)));

      self.normalizer = Some(Normalizer::new(
        &mut self.arena as *mut _,
        builtin_types,
        &mut self.unifier_state as *mut _,
        solver_mode,
        false,
      ));
    }

    self.base.get_frontend()
  }
}
