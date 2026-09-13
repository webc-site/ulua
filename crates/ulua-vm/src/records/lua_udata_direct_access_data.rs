use crate::{
  records::lua_t_value::lua_TValue,
  type_aliases::{
    lua_userdata_direct_access::LuaUserdataDirectAccess,
    lua_userdata_direct_namecall::LuaUserdataDirectNamecall,
  },
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
#[derive(Default)]
pub struct lua_UdataDirectAccessData {
  pub(crate) indextm: lua_TValue,
  pub(crate) newindextm: lua_TValue,
  pub(crate) namecalltm: lua_TValue,
  pub(crate) index: LuaUserdataDirectAccess,
  pub(crate) newindex: LuaUserdataDirectAccess,
  pub(crate) namecall: LuaUserdataDirectNamecall,
}

pub type LuaUdataDirectAccessData = lua_UdataDirectAccessData;
