//! Source: `CLI/src/Repl.cpp:306` (`tryReplaceTopWithIndex`)
//!
//! C++ 单点定义于 Repl.cpp, repl-cli 与 cli-test 共用一份实现。

use ulua_vm::records::lua_state::LuaState;

use crate::functions::safe_get_table::META_INDEX_FIELD;

// review.md §2/§3 收形：`l` 由裸 `*mut LuaState` 收编为借用 `&mut LuaState`——
// 栈顶元表 `__index` 单槽替换（get_metafield_bytes/remove）在引用接收者上均为
// ulua-vm 安全面，解引用对象是调用方交出的存活借用，故降为安全 `fn`，原 `# Safety`
// 契约降级为下述「调用序契约」；`state` 门面随之消亡。
//
// 调用序契约（由调用方成立）：`l` 为活跃状态机且栈上至少有一个元素（`-1` 即栈顶）。
pub fn try_replace_top_with_index(l: &mut LuaState) -> bool {
  // `META_INDEX_FIELD` 为静态字节串（无尾部 NUL，`get_metafield_bytes` 以切片全长为键）。
  let has_index = l.get_metafield_bytes(-1, META_INDEX_FIELD);
  if !has_index {
    return false;
  }
  // 上一步把 __index 压到了栈顶，此时 `-2` 是它下面那张表。
  l.remove(-2);
  true
}
