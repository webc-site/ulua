//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:239:reallymarkobject`
//! Source: `VM/src/lgc.cpp` (lgc.cpp:239-309, hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  macros::{
    gco_2_cl::gco2cl, gco_2_class::gco2class, gco_2_h::gco2h, gco_2_object::gco2object,
    gco_2_p::gco2p, gco_2_th::gco2th, gco_2_u::gco2u, gco_2_uv::gco2uv, gray_2_black::gray2black,
    isdead::isdead, iswhite::iswhite, markobject::markobject, markvalue::markvalue,
    upisopen::upisopen, white_2_gray::white2gray,
  },
  records::{
    closure::Closure, gc_object::GCObject, global_state::global_State, lua_table::LuaTable,
    luau_class::LuauClass, luau_object::LuauObject, proto::Proto, up_val::UpVal,
  },
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn reallymarkobject(g: *mut global_State, o: *mut GCObject) {
  unsafe {
    LUAU_ASSERT!(iswhite!(o) && !isdead!(g, o));
    white2gray!(o);
    match (*o).gch.tt as i32 {
      t if t == LuaType::String as i32 => {}
      t if t == LuaType::UserData as i32 => {
        let mt: *mut LuaTable = (*gco2u!(o)).metatable;
        gray2black!(o); // udata are never gray
        if !mt.is_null() {
          markobject!(g, mt);
        }
      }
      t if t == LuaType::Upval as i32 => {
        let uv = gco2uv!(o) as *const _ as *mut UpVal;
        markvalue!(g, (*uv).v);
        if !upisopen!(uv) {
          // closed?
          gray2black!(o); // open upvalues are never black
        }
      }
      t if t == LuaType::Function as i32 => {
        (*(gco2cl!(o) as *const _ as *mut Closure)).gclist = (*g).gray;
        (*g).gray = o;
      }
      t if t == LuaType::Table as i32 => {
        (*(gco2h!(o) as *const _ as *mut LuaTable)).gclist = (*g).gray;
        (*g).gray = o;
      }
      t if t == LuaType::Thread as i32 => {
        (*(gco2th!(o) as *const _ as *mut lua_State)).gclist = (*g).gray;
        (*g).gray = o;
      }
      t if t == LuaType::Buffer as i32 => {
        gray2black!(o); // buffers are never gray
      }
      t if t == LuaType::Proto as i32 => {
        (*(gco2p!(o) as *const _ as *mut Proto)).gclist = (*g).gray;
        (*g).gray = o;
      }
      t if t == LuaType::Class as i32 => {
        (*(gco2class!(o) as *const _ as *mut LuauClass)).gclist = (*g).gray;
        (*g).gray = o;
      }
      t if t == LuaType::Object as i32 => {
        (*(gco2object!(o) as *const _ as *mut LuauObject)).gclist = (*g).gray;
        (*g).gray = o;
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }
}
