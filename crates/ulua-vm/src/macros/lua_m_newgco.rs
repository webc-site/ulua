#[macro_export]
macro_rules! lua_m_newgco {
  ($l:expr, $t:ty, $size:expr, $memcat:expr) => {
    $crate::macros::cast_to::cast_to!(
      $t,
      $crate::functions::lua_m_newgco::luaM_newgco_(
        $l as *mut $crate::records::lua_state::lua_State,
        $size,
        $memcat
      ) as *mut $crate::records::gc_object::GCObject
    )
  };
}

pub use lua_m_newgco;
pub use lua_m_newgco as luaM_newgco;
