#[macro_export]
macro_rules! VM_INTERRUPT {
  ($l:expr) => {
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
