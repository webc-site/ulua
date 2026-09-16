use core::mem::ManuallyDrop;

use crate::records::{
  closure::Closure, g_cheader::GCheader, lua_state::lua_State, lua_table::LuaTable,
  luau_buffer::LuauBuffer, luau_class::LuauClass, luau_object::LuauObject, proto::Proto,
  t_string::tstring, udata::Udata, up_val::UpVal,
};
#[repr(C)]
pub union GcObject {
  pub gch: GCheader,
  pub ts: ManuallyDrop<tstring>,
  pub u: ManuallyDrop<Udata>,
  pub cl: ManuallyDrop<Closure>,
  pub h: ManuallyDrop<LuaTable>,
  pub p: ManuallyDrop<Proto>,
  pub uv: ManuallyDrop<UpVal>,
  pub th: ManuallyDrop<lua_State>,
  pub buf: ManuallyDrop<LuauBuffer>,
  pub lclass: ManuallyDrop<LuauClass>,
  pub lobject: ManuallyDrop<LuauObject>,
}

pub type GCObject = GcObject;
