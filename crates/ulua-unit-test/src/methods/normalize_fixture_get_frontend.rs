use alloc::sync::Arc;

use ulua_analysis::records::{
  arena_handle::Handle, frontend::Frontend, normalizer::Normalizer, scope::Scope,
};

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

      let any_type_pack = unsafe {
        // Safety: 行 20-23 断言 self.base.builtin_types 非空——它是上次 get_frontend 刷新缓存的 Frontend.builtin_types.as_ptr()（指向 Box 内 builtin_types_ 的稳定地址）；any_type_pack 为标量读，Normalizer 以该指针布线只使用其值。
        (*builtin_types).any_type_pack
      };
      self.global_scope = Some(Arc::new(Scope::scope_type_pack_id(any_type_pack)));

      self.normalizer = Some(Normalizer::new(
        Some(Handle::from_mut(&mut self.arena)),
        Handle::from_ptr(builtin_types),
        Some(Handle::from_mut(&mut self.unifier_state)),
        solver_mode,
        false,
      ));
    }

    self.base.get_frontend()
  }
}
