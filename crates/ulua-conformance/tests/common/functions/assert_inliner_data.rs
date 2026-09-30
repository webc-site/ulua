//! `global_State::ecbdata` 这块执行回调预留存储与 `AssertInlinerData` 之间的唯一转换出口。
//!
//! cpp 侧 `tests/FeedbackVector.test.cpp:93,136,240,376` 四处都写
//! `reinterpret_cast<AssertInlinerData*>(L->global->ecbdata)`，指针转换的前提
//! （内联数组的对齐与容量）由编译期断言在这里集中表达，用例侧不再重复裸转换。

use core::mem::{align_of, size_of};

use ulua_vm::records::{
  global_state::global_State,
  lua_execution_callback_storage::{LUA_EXECUTION_CALLBACK_STORAGE, LuaExecutionCallbackStorage},
  lua_state::LuaState,
  proto::Proto,
};

use crate::common::records::assert_inliner_data::AssertInlinerData;

const _: () = assert!(size_of::<AssertInlinerData>() <= LUA_EXECUTION_CALLBACK_STORAGE);
const _: () = assert!(
  align_of::<AssertInlinerData>() <= align_of::<LuaExecutionCallbackStorage>(),
  "ecbdata 的 alignas(16) 必须覆盖 AssertInlinerData 的对齐要求"
);

/// 取回放在 `ecbdata` 里的 `AssertInlinerData`。
///
/// 返回指针不会是空（`ecbdata` 是 `global_State` 的内联数组而非指针字段），
/// 因此 `global` 一旦为空就是 `LuaState` 尚未初始化，属于调用方违约。
///
/// # Safety
/// `l` 非空且 `(*l).global` 指向存活的 `global_State`；读取方须保证该存储里确实是一个
/// 用 [`install_assert_inliner_data`] 初始化过的 `AssertInlinerData`。
pub unsafe fn assert_inliner_data(l: *mut LuaState) -> *mut AssertInlinerData {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`global` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let global: *mut global_State = unsafe { (*l).global };
  assert!(!global.is_null(), "LuaState 未初始化：global 为空");

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`global` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { (*global).ecbdata.as_mut_ptr().cast::<AssertInlinerData>() }
}

/// cpp `tests/FeedbackVector.test.cpp:136-140,240-244,376-380` 的
/// `data->proto = f; data->target = g; data->pc = ...;`：把用例期望内联回调看到的
/// caller / target / pc 写进 `ecbdata`，供 `id_inliner_with_assert` 比对。
///
/// 返回写入后的指针，便于用例在 `run()` 之后读回 `called` 标志。
///
/// # Safety
/// 见 [`assert_inliner_data`]；`proto` / `target` 必须是存活的 `Proto`。
pub unsafe fn install_assert_inliner_data(
  l: *mut LuaState,
  proto: *mut Proto,
  target: *mut Proto,
  pc: u32,
) -> *mut AssertInlinerData {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`target` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let data = unsafe { assert_inliner_data(l) };
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`target` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    data.write(AssertInlinerData {
      proto,
      target,
      pc,
      called: false,
    })
  };

  data
}
