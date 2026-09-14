#[macro_export]
macro_rules! lua_c_barrierfast {
  ($l:expr, $t:expr) => {
    if $crate::macros::isblack::isblack!($t as *mut $crate::records::gc_object::GCObject) {
      $crate::functions::lua_c_barrierback::lua_c_barrierback(
        $l as *mut $crate::records::lua_state::lua_State,
        $t as *mut $crate::records::gc_object::GCObject,
        &mut (*$t).gclist,
      );
    }
  };
}

pub use lua_c_barrierfast;
pub use lua_c_barrierfast as luaC_barrierfast;
