use alloc::string::String;

use ulua_analysis::{
  functions::{freeze::freeze, unfreeze::unfreeze},
  records::{
    frontend::Frontend, global_types::GlobalTypes,
    load_definition_file_result::LoadDefinitionFileResult,
  },
};

use crate::records::fixture::Fixture;

impl Fixture {
  /// C++ `Fixture::loadDefinition`（`Fixture.cpp:691-704`）：
  /// `unfreeze` → `loadDefinitionFile` → `freeze` → `REQUIRE_MESSAGE(result.success, ..)`。
  ///
  /// 借用形态：cpp 只有一条 `frontend` 对象血缘——`GlobalTypes& globals = ...;
  /// frontend.loadDefinitionFile(globals, globals.globalScope, ...)`。Rust 侧callee 现以
  /// **目标表选择器闭包**承接这条血缘（`load_definition_file` 两阶段拆分：阶段一只借
  /// `&mut Frontend`，阶段二才经选择器物化字段借用），因此这里不再需要"先降级为
  /// `*mut Frontend` 再就地取字段"的裸句柄手法：选择器是普通 `fn` 指针，
  /// `unfreeze/load/freeze` 每步的 `&mut` 止于实参位，全程普通借用。
  pub fn load_definition(
    &mut self,
    source: &str,
    for_autocomplete: bool,
  ) -> LoadDefinitionFileResult {
    let frontend = self.get_frontend();

    // 选择器：autocomplete 变体走 `globals_for_autocomplete`，否则走 `globals`。
    let select: for<'a> fn(&'a mut Frontend) -> &'a mut GlobalTypes = if for_autocomplete {
      |frontend| &mut frontend.globals_for_autocomplete
    } else {
      |frontend| &mut frontend.globals
    };

    unfreeze(select(frontend).global_types_mut());
    let target_scope = select(frontend).global_scope();
    let result = frontend.load_definition_file(
      select,
      target_scope,
      source,
      String::from("@test"),
      false,
      for_autocomplete,
    );
    freeze(select(frontend).global_types_mut());

    assert!(
      result.success,
      "loadDefinition: unable to load definition file"
    );
    result
  }
}
