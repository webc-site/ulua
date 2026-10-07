use crate::records::lua_state::LuaState;

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；体内压值/
/// 写表全经安全门面，无裸操作，故本体降为安全 `fn`）：`l` 栈顶相对索引 -2 处须为可写表——本函数
/// 先把 `value` 压入 top（占 1 个空槽），再 `set_field_bytes(-2, key)` rawset 并消费该值，栈形
/// 净变化为零。`key` 为纯 Rust 字节切片（如 `b"sec"`），直接传给 `set_field_bytes`。
pub(crate) fn setfield(l: &mut LuaState, key: &[u8], value: i32) {
  l.push_integer(value);

  l.set_field_bytes(-2, key);
}
