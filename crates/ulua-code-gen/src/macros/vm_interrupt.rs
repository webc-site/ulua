#[macro_export]
macro_rules! VM_INTERRUPT {
  ($l:expr) => {
    // Safety: 调用点契约——$l 为存活 lua_State* 且其 global 已初始化；interrupt 回调先判空
    // 再按 C ABI (lua_State*, int) 调用，null 时跳过，本宏不解引用其他悬垂字段。
    unsafe {
      let l_state = $l;
      let interrupt_fn = (*(*l_state).global).cb.interrupt;
      if ulua_common::LUAU_UNLIKELY!(!interrupt_fn.is_null()) {
        interrupt_fn(l_state, 0);
      }
    }
  };
}

pub use VM_INTERRUPT;
