use crate::records::{arena_handle::alias, stringifier_state::StringifierState};
#[derive(Debug, Clone)]
pub struct TypeStringifier {
  pub(crate) state: *mut StringifierState,
}

impl TypeStringifier {
  /// §2 裸指针收口单点：`state` 由入口 `to_string*` 构造期以自身局部
  /// `StringifierState` 的 `&mut` 裸化注入（C++ 引用成员直译），入口函数保证
  /// 其在整个字符串化会话内存活且无并存别名；解引用只发生在 [`alias`] 一处，
  /// 业务调用点经此恢复普通引用用法。
  pub(crate) fn st(&mut self) -> &'static mut StringifierState {
    alias(self.state)
  }
}
