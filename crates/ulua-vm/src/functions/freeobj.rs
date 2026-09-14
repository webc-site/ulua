use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_b_freebuffer::lua_b_freebuffer, lua_e_freethread::luaE_freethread,
    lua_f_freeclosure::lua_f_freeclosure, lua_f_freeproto::luaF_freeproto,
    lua_f_freeupval::lua_f_freeupval, lua_h_free::lua_h_free, lua_r_freeclass::lua_r_freeclass,
    lua_r_freeobject::lua_r_freeobject, lua_s_free::luaS_free, lua_u_freeudata::lua_u_freeudata,
  },
  records::{gc_object::GCObject, lua_page::lua_Page},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn freeobj(l: *mut lua_State, o: *mut GCObject, page: *mut lua_Page) {
  unsafe {
    match (*o).gch.tt as i32 {
      x if x == LuaType::Proto as i32 => {
        luaF_freeproto(l, core::ptr::addr_of_mut!((*o).p) as *mut _, page);
      }
      x if x == LuaType::Function as i32 => {
        lua_f_freeclosure(l, core::ptr::addr_of_mut!((*o).cl) as *mut _, page);
      }
      x if x == LuaType::Upval as i32 => {
        lua_f_freeupval(l, core::ptr::addr_of_mut!((*o).uv) as *mut _, page);
      }
      x if x == LuaType::Table as i32 => {
        lua_h_free(l, core::ptr::addr_of_mut!((*o).h) as *mut _, page);
      }
      x if x == LuaType::Thread as i32 => {
        let th = core::ptr::addr_of_mut!((*o).th) as *mut lua_State;
        LUAU_ASSERT!(th != l && th != (*(*l).global).mainthread);
        luaE_freethread(l, th, page);
      }
      x if x == LuaType::String as i32 => {
        luaS_free(l, core::ptr::addr_of_mut!((*o).ts) as *mut _, page);
      }
      x if x == LuaType::UserData as i32 => {
        lua_u_freeudata(l, core::ptr::addr_of_mut!((*o).u) as *mut _, page);
      }
      x if x == LuaType::Buffer as i32 => {
        lua_b_freebuffer(l, core::ptr::addr_of_mut!((*o).buf) as *mut _, page);
      }
      x if x == LuaType::Class as i32 => {
        lua_r_freeclass(l, core::ptr::addr_of_mut!((*o).lclass) as *mut _, page);
      }
      x if x == LuaType::Object as i32 => {
        lua_r_freeobject(l, core::ptr::addr_of_mut!((*o).lobject) as *mut _, page);
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }
}
