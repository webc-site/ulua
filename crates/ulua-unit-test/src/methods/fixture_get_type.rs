use ulua_analysis::type_aliases::type_id::TypeId;
use ulua_common::fflag;

use crate::{
  functions::{linear_search_for_binding::linear_search_for_binding, lookup_name::lookup_name},
  records::fixture::Fixture,
};
impl Fixture {
  pub fn get_type(&mut self, name: &str, for_autocomplete: bool) -> Option<TypeId> {
    let module = self.get_main_module(for_autocomplete);
    if module.is_null() {
      panic!("getType: No main module");
    }

    let module = unsafe {
      // Safety: module 行 14-16 判空 panic 后存活（resolver 持有的主模块）；&* 物化只读借用查 has_module_scope/module_scope，借用不出帧。
      &*module
    };
    if !module.has_module_scope() {
      return None;
    }

    let scope = module.get_module_scope();
    // C++ `Fixture::getType` (Fixture.cpp:457-469): the new solver reads the
    // raw binding via the free `linearSearchForBinding`; the old solver uses
    // `lookupName` (the `Scope` method). Both ultimately return
    // `binding.typeId`, so the distinction is preserved for fidelity.
    // 两个下游都是 `&Scope` 只读查询，`Arc<Scope>` 直接借用即可，无需再取句柄。
    if !fflag::DebugLuauForceOldSolver.get() {
      linear_search_for_binding(&scope, name)
    } else {
      lookup_name(&scope, name)
    }
  }
}
