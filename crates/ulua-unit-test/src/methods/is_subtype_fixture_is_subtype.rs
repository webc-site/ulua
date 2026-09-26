//! Source: `tests/Fixture.cpp`

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::is_subtype_normalize::is_subtype,
  records::{
    arena_handle::Handle, normalizer::Normalizer, type_arena::TypeArena,
    type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
    unifier_shared_state::UnifierSharedState,
  },
  type_aliases::type_id::TypeId,
};
use ulua_common::fflag;

use crate::{functions::raw_handle::raw_handle, records::is_subtype_fixture::IsSubtypeFixture};
impl IsSubtypeFixture {
  pub fn is_subtype(&mut self, a: TypeId, b: TypeId) -> bool {
    let module = self.base.get_main_module(false);
    assert!(!module.is_null(), "isSubtype: expected main module");

    let module = unsafe {
      // Safety: 行 18 断言非空：module 为 resolver 注册保有的主模块（存活至 fixture 结束）；&* 物化只读借用取 module_scope；UnifierSharedState::new(&mut self.ice as *mut _) 取借用期内字段地址（cpp 把 ice 当 UnifierSharedState 指针复用的同形用法），is_subtype 全程只读。
      &*module
    };
    assert!(
      module.has_module_scope(),
      "isSubtype: module scope data is not available"
    );

    let scope = module.get_module_scope();
    let mut shared_state = UnifierSharedState::new(&mut self.base.ice as *mut _);
    let solver_mode = if fflag::DebugLuauForceOldSolver.get() {
      SolverMode::Old
    } else {
      SolverMode::New
    };
    let mut normalizer = Normalizer::new(
      Some(Handle::from_mut(&mut self.base.arena)),
      Handle::from_ptr(self.base.builtin_types),
      Some(Handle::from_mut(&mut shared_state)),
      solver_mode,
      false,
    );

    let mut arena = TypeArena::default();
    let limits = TypeCheckLimits::default();
    let mut type_function_runtime =
      TypeFunctionRuntime::new(&self.base.ice, &limits, scope.clone());

    is_subtype(
      a,
      b,
      Handle::from_mut(&mut arena),
      Handle::from_ptr(self.base.builtin_types),
      raw_handle(&scope),
      Some(Handle::from_mut(&mut normalizer)),
      Handle::from_mut(&mut type_function_runtime),
      Handle::from_mut(&mut self.base.ice),
    )
  }
}
