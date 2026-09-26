use ulua_vm::{
  functions::lua_pushcclosurek::lua_pushcclosurek,
  macros::lua_setglobal::lua_setglobal,
  records::lua_state::LuaState,
  type_aliases::{lua_c_function::LuaCFunction, lua_continuation::LuaContinuation},
};

use crate::common::functions::{
  c_yielding_iterator::c_yielding_iterator,
  c_yielding_iterator_continuation::c_yielding_iterator_continuation, cstr::cstr,
  setup_native_helpers::setup_native_helpers,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_iter_setup(l: *mut LuaState) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    setup_native_helpers(l);

    let iterator: LuaCFunction = Some(c_yielding_iterator);
    let continuation: LuaContinuation = Some(c_yielding_iterator_continuation);

    lua_pushcclosurek(l, iterator, cstr(b"cYieldingIterator\0"), 0, continuation);
    lua_setglobal(l, cstr(b"cYieldingIterator\0"));
  }
}
