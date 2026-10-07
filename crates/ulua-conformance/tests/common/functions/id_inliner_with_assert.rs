use ulua_vm::records::{closure::Closure, lua_state::LuaState, proto::Proto};

use crate::common::functions::assert_inliner_data::assert_inliner_data;

/// cpp `tests/FeedbackVector.test.cpp:88-97` 的 `idInlinerWithAssert`：从
/// `L->global->ecbdata` 取回用例写入的期望值并逐项比对。
///
/// # Safety
/// `l` 及其 `global` 必须有效，且 `ecbdata` 里已由
/// [`install_assert_inliner_data`](crate::common::functions::assert_inliner_data::install_assert_inliner_data)
/// 写入过 `AssertInlinerData`；`caller` / `target` 必须是存活的 `Closure`。
pub unsafe extern "C-unwind" fn id_inliner_with_assert(
  l: *mut LuaState,
  caller: *mut Closure,
  target: *mut Closure,
  pc: u32,
) -> *mut Proto {
  // Safety: 函数级契约——`l` 的 ecbdata 已由 `install_assert_inliner_data` 写入期望值。
  let data = unsafe { assert_inliner_data(l) };

  // Safety: 函数级契约保证 `caller` 为存活 Closure，其 `inner.l.p` 字段可读。
  let caller_proto = unsafe { (*caller).inner.l.p };
  // Safety: 函数级契约保证 `target` 为存活 Closure，其 `inner.l.p` 字段可读。
  let target_proto = unsafe { (*target).inner.l.p };

  // Safety: `data` 指向 ecbdata 内已安装的 `AssertInlinerData`（函数级契约），
  // 三个期望字段可读；比对本身是 safe Rust。
  let (want_proto, want_target, want_pc) = unsafe { ((*data).proto, (*data).target, (*data).pc) };
  assert_eq!(want_proto, caller_proto);
  assert_eq!(want_target, target_proto);
  assert_eq!(want_pc, pc);

  // Safety: 同上——`data` 指向已安装记录；写 `called` 供用例收尾断言。
  unsafe { (*data).called = true };

  caller_proto
}
