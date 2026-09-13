#[macro_export]
macro_rules! savestack {
  ($l:expr, $p:expr) => {{
    let l_ptr = $l as *mut $crate::records::lua_state::lua_State;
    let p_ptr = $p as *mut $crate::type_aliases::t_value::TValue;
    $crate::macros::check_exp::check_exp!(
      (p_ptr as usize >= (*l_ptr).stack as usize)
        && (p_ptr as usize
          <= (*l_ptr).stack as usize
            + ((*l_ptr).stacksize as usize
              * core::mem::size_of::<$crate::type_aliases::t_value::TValue>())),
      p_ptr as isize - (*l_ptr).stack as isize
    )
  }};
}

pub use savestack;
