use crate::records::lua_state::LuaState;

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；体内压值/
/// 写表全经安全门面，无裸操作，故本体降为安全 `fn`）：`value < 0` 表示"该字段不存在"（cpp 的
/// `tm_isdst == -1`），直接返回而不触碰栈；否则 `l` 栈顶相对索引 -2 处须为可写表——本函数先把
/// `value != 0` 压入 top（占 1 个空槽），再 `set_field_bytes(-2, key)` rawset 并消费该值，栈形
/// 净变化为零。`key` 为纯 Rust 字节切片（如 `b"isdst"`）。
pub(crate) fn setboolfield(l: &mut LuaState, key: &[u8], value: i32) {
  if value < 0 {
    return;
  }

  l.push_boolean(value != 0);

  l.set_field_bytes(-2, key);
}
