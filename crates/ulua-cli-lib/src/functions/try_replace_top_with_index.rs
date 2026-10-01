//! Source: `CLI/src/Repl.cpp:306` (`tryReplaceTopWithIndex`)
//!
//! C++ 单点定义于 Repl.cpp, repl-cli 与 cli-test 共用一份实现。

use ulua_vm::records::lua_state::LuaState;

use crate::functions::{safe_get_table::META_INDEX_FIELD, state_ref::state};

// DELIBERATE DEVIATION（review.md §9.3）：直接借 VM 栈顶槽做元表 `__index` 单槽替换
// （get_metafield_bytes/remove 均为 ulua-vm c-API），裸 `*mut LuaState` 系边界固有：
// 本函数即边界本体（clippy `not_unsafe_ptr_arg_deref` 认定的「解引用实参的 pub
// 函数须标 unsafe」），非 review.md §2 要消除的纯逻辑层 unsafe。
/// # Safety
///
/// `l` must be a valid, active pointer to a `LuaState` with at least one element on its stack.
pub unsafe fn try_replace_top_with_index(l: *mut LuaState) -> bool {
  // Safety: `# Safety` 契约保证 `l` 非空、活跃，经 `state` 门面物化后全走安全方法。
  let l = state(l);
  // `# Safety` 契约保证栈非空，`-1` 即栈顶；`META_INDEX_FIELD` 为静态字节串
  // （无尾部 NUL，`get_metafield_bytes` 以切片全长为键）。
  let has_index = l.get_metafield_bytes(-1, META_INDEX_FIELD);
  if !has_index {
    return false;
  }
  // 上一步把 __index 压到了栈顶，此时 `-2` 是它下面那张表。
  l.remove(-2);
  true
}
