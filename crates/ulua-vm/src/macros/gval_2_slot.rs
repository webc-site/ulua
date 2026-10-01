#[macro_export]
macro_rules! gval2slot {
  ($t:expr, $v:expr) => {
    ((($v as *const $crate::type_aliases::t_value::TValue as *mut $crate::records::lua_node::LuaNode
      as usize)
      .wrapping_sub((*$t).node as usize)
      / core::mem::size_of::<$crate::records::lua_node::LuaNode>()) as i32)
  };
}

pub use gval2slot;
