use ulua_analysis::{records::module::Module, type_aliases::module_name_type::ModuleName};
use ulua_common::fflag;

use crate::{functions::raw_handle::raw_handle, records::fixture::Fixture};

const MAIN_MODULE_NAME: &str = "MainModule";

impl Fixture {
  /// C++ `Fixture::getMainModule`（`Fixture.cpp:428-436`）。
  ///
  /// 旧 solver 才会用到补全专用的 module resolver；其余情况（含补全查询走新 solver）
  /// 都读常规 `module_resolver`。
  ///
  /// 句柄生命周期：上游返回 `ModulePtr`（`shared_ptr<Module>`），Rust 侧下游 API 与既有
  /// 调用点按 `*mut Module` 收，故经 `raw_handle` 取句柄。模块由
  /// `FrontendModuleResolver::modules` 强引用持有，与 `Fixture`（连同其 `Frontend`）同生
  /// 同死，句柄在 `Fixture` 存活期内有效；模块缺失时 `get_module` 直接 panic（对齐 cpp
  /// 把 nullptr 升级为 InternalCompilerError 的调用点），不会给出空句柄。
  pub fn get_main_module(&mut self, for_autocomplete: bool) -> *mut Module {
    let module_name = ModuleName::from(MAIN_MODULE_NAME);
    let frontend = self.get_frontend();

    let module = if for_autocomplete && fflag::DebugLuauForceOldSolver.get() {
      frontend
        .module_resolver_for_autocomplete
        .get_module(&module_name)
    } else {
      frontend.module_resolver.get_module(&module_name)
    };

    raw_handle(&module)
  }
}
