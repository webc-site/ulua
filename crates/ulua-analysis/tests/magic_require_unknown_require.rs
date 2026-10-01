//! H1 回归：cpp MagicRequire::handleOldSolver（BuiltinDefinitions.cpp:1899）
//! 以 AstExprCall 节点本身作为 require-trace 查找 key。RequireTracer 对
//! 无法解析的 require 只按 call 节点写入空 ModuleInfo（default），旧求解器
//! 据此命中并走 checkRequire，strict 模式下报 UnknownRequire
//! （对应 cpp NonStrictTypeChecker.test.cpp 中
//! "new_non_strict_should_suppress_dynamic_require_errors" 的 strict 分支
//! 语义）。若查找 key 误用实参节点，该错误会被静默丢失。

extern crate alloc;

use alloc::string::String;

use ulua_analysis::records::unknown_require::UnknownRequire;
use ulua_ast::enums::mode::Mode;
use ulua_common::fflag;
use ulua_unit_test::{
  functions::type_error_data_ref::type_error_data_ref, records::builtins_fixture::BuiltinsFixture,
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};

#[test]
fn old_solver_unresolvable_require_still_reports_unknown_require() {
  // 强制旧求解器，驱动 magic_require_handle_old_solver。
  let _old_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  // require 的实参是函数调用表达式，RequireTracer/FileResolver 无法静态
  // 解析出模块路径 → trace 中只留下以 call 节点为 key 的空 ModuleInfo。
  let result = fixture.base.check_mode_string_optional_frontend_options(
    Mode::Strict,
    &String::from(
      r#"
local function getDynamicPath(): string
    return "hello"
end

local m = require(getDynamicPath())
"#,
    ),
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(
    type_error_data_ref::<UnknownRequire>(&result.errors[0]).is_some(),
    "应为 UnknownRequire，实际: {:?}",
    result.errors[0]
  );
}
