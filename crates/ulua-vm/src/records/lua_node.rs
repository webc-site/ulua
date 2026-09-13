use crate::records::{lua_t_value::TValue, t_key::TKey};
#[derive(Debug, Clone, Copy)]
#[repr(C)]
#[derive(Default)]
pub struct LuaNode {
  pub val: TValue,
  pub key: TKey,
}
