// 边界契约测试：null 系 c-API 合法实参（既有约定 review.md §2）
// 错误对象与 protected call 语义用例
// 移植自 `cpp/tests/Conformance.test.cpp`。

use core::ptr::null_mut;

use crate::common::functions::cstr::cstr;

#[test]
fn conformance_errors() {
  use ulua_common::fflag;

  use crate::common::{
    functions::run_conformance::run_fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // cpp `Conformance.test.cpp:1421-1424` 的 Errors 用例不带任何作用域旗标；本端口
  // `resume_finish.rs` 按更新上游移植了 xpcall 消息 yield 路径修复（本地 cpp oracle
  // 是修复前的旧快照，见该文件 DELIBERATE DEVIATION 注），errors.luau 的
  // xpcall/yield 段落按旗开语义成立，故这里显式置真。
  let _luau_xpcall_fix = ScopedFastFlag::new(&fflag::LuauXpcallFixMessageYieldPath, true);

  run_fixture("errors.luau");
}

#[test]
fn conformance_exception_object() {

  use crate::common::functions::{
    conformance_exception_object_capture_exception::conformance_exception_object_capture_exception,
    ends_with::ends_with, limited_realloc::limited_realloc, run_conformance::run_conformance,
    safe_api::newstate,
  };

  // run_conformance 按其契约接管 `newstate` 产出的 state（limited_realloc 自定义
  // 分配器、ud 遵 C 侧 NULL 哨兵）；`l` 于返回后存活。
  // FFI: c-API 要求 NULL
  let global_state = run_conformance(
    "exceptions.luau",
    None,
    None,
    Some(newstate(Some(limited_realloc), null_mut())),
    None,
    false,
    None,
  );
  let l = global_state.as_ptr();

  // 捕获桩在 `l` 上建临时线程跑具名函数，返回 owned 的 ExceptionResult。
  let result =
    conformance_exception_object_capture_exception(l, cstr(b"infinite_recursion_error\0"));
  assert!(result.exception_generated);

  // empty_function 正常返回，不产生异常。
  let result =
    conformance_exception_object_capture_exception(l, cstr(b"empty_function\0"));
  assert!(!result.exception_generated);

  // 数字参数进 error 产生异常，描述尾为 "42"。
  let result =
    conformance_exception_object_capture_exception(l, cstr(b"pass_number_to_error\0"));
  assert!(result.exception_generated);
  assert!(ends_with(&result.description, "42"));

  // 字符串参数进 error 产生异常，描述尾为参数串。
  let result =
    conformance_exception_object_capture_exception(l, cstr(b"pass_string_to_error\0"));
  assert!(result.exception_generated);
  assert!(ends_with(&result.description, "string argument"));

  // 表参数进 error 同样产生异常。
  let result =
    conformance_exception_object_capture_exception(l, cstr(b"pass_table_to_error\0"));
  assert!(result.exception_generated);

  // large_allocation_error 触发分配失败异常。
  let result =
    conformance_exception_object_capture_exception(l, cstr(b"large_allocation_error\0"));
  assert!(result.exception_generated);
}
