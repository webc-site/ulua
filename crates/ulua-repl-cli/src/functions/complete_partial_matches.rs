use alloc::string::String;
use core::str::from_utf8;

use ulua_cli_lib::functions::{
  safe_get_table::MAX_TRAVERSAL_LIMIT, try_replace_top_with_index::try_replace_top_with_index,
};
use ulua_vm::{
  enums::lua_type::LuaType, functions::lua_tolstring::lua_tolstring_ref,
  records::lua_state::LuaState,
};

// review.md §2/§3 收形：`l` 由裸 `*mut LuaState` 收编为借用 `&mut LuaState`（真实
// 物化点上移到 get_completions 入口一次）。遍历栈顶表键并跟随 `__index` 元表链
// （next/is_table/pop + `lua_tolstring_ref`，ulua-vm c-API）；至多 MAX_TRAVERSAL_LIMIT
// 层的计数无数据含义，故用 range 迭代器而非手写累加。
//
// 调用序契约（由调用方 complete_indexer 成立）：`l` 为活跃状态机，且待搜索的表位于栈顶。
pub(crate) fn complete_partial_matches(
  l: &mut LuaState,
  complete_only_functions: bool,
  edit_buffer: &str,
  prefix: &str,
  add_completion_callback: &mut impl FnMut(&str, &str),
) {
  // cpp `for (int i = 0; i < MAX_TRAVERSAL_LIMIT; i++)`：至多跟随 MAX_TRAVERSAL_LIMIT
  // 层 __index 链，防元表环；计数无数据含义，用 range 迭代器替代手写累加。
  for _ in 0..MAX_TRAVERSAL_LIMIT {
    // -1 为待搜索表/对象（本 fn 前置条件与
    // 各轮末的表替换），确认是表才开始遍历。
    if !l.is_table(-1) {
      break;
    }

    // 起始 key=nil，push/pop 严格配平（迭代末弹 value），
    // add_completion_callback 为本帧 &mut 借用且不触栈。
    l.push_nil();

    // Loop over all the keys in the current table
    // `LuaState::next` 为安全方法：以 -2 为 key 就地推进并在 -1 压入 value，
    // 返回 false 表示遍历结束。
    while l.next(-2) {
      // table, key, value
      // 键按原始字节读取：补全文本必须是合法 UTF-8，非法序列或非串键直接跳过
      // （避免 lossy 替换造出与实际键不符的补全项）。
      // Safety: `lua_tolstring_ref` 为 unsafe 导出；type_of 已确认 -2 为 string，
      // 其返回键的全字节切片（借用仅存活到下方 from_utf8 判定；串对象不受 GC 移动，
      // 弹 value 槽不影响 -2 键槽，与旧 (指针, 长度) 形态的存活窗口一致）。
      // 与本轮 lua_next 压入的 value 配平：非串键与串键均先弹回 value 槽，
      // 后续判定与拼接全是纯字符串逻辑（安全域）。
      let Some((key_bytes, value_type)) = (if l.type_of(-2) == LuaType::String {
        let key_bytes = unsafe { lua_tolstring_ref(l, -2) }.unwrap_or_default();
        let value_type = l.type_of(-1);
        // 先弹回遍历槽位；后续判定与拼接全是纯字符串逻辑（安全域）
        l.pop(1);
        Some((key_bytes, value_type))
      } else {
        // 非串键同样弹走本轮压入的 value，保持 -1 为下一轮 key。
        l.pop(1);
        None
      }) else {
        continue;
      };

      let Ok(key) = from_utf8(key_bytes) else {
        continue;
      };

      // If the last separator was a ':' (i.e. a method call) then only functions should be completed.
      let required_value_type = !complete_only_functions || value_type == LuaType::Function;

      if !key.is_empty() && required_value_type && key.starts_with(prefix) {
        // starts_with 已保证 prefix.len() 落在 key 的字符边界上
        let completed_component = &key[prefix.len()..];
        let mut completion =
          String::with_capacity(edit_buffer.len() + completed_component.len() + 1);
        completion.push_str(edit_buffer);
        completion.push_str(completed_component);
        if value_type == LuaType::Function {
          // Add an opening paren for function calls by default.
          completion.push('(');
        }
        add_completion_callback(&completion, key);
      }
    }

    // Replace the current table being searched with an __index table if one exists
    // try_replace_top_with_index 收形为借用后为安全 fn：-1 为遍历结束的表，
    // 命中元表 __index/_index 时单槽替换供下一轮。
    let has_index = try_replace_top_with_index(l);
    if !has_index {
      break;
    }
  }
}
