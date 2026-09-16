use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    validateclass::validateclass, validateclosure::validateclosure, validateobject::validateobject,
    validateobjref::validateobjref, validateproto::validateproto, validateref::validateref,
    validatestack::validatestack, validatetable::validatetable,
  },
  macros::{
    gco_2_cl::gco2cl, gco_2_class::gco2class, gco_2_h::gco2h, gco_2_object::gco2object,
    gco_2_p::gco2p, gco_2_th::gco2th, gco_2_u::gco2u, gco_2_uv::gco2uv, isdead::isdead,
    obj_2_gco::obj2gco,
  },
  records::{gc_object::GCObject, global_state::global_State, udata::Udata, up_val::UpVal},
};

pub(crate) unsafe fn validateobj(g: *mut global_State, o: *mut GCObject) {
  unsafe {
    if isdead!(g, o) {
      LUAU_ASSERT!((*g).gcstate == 4);
      return;
    }

    match (*o).gch.tt as i32 {
      t if t == LuaType::String as i32 => {}
      t if t == LuaType::Table as i32 => {
        validatetable(g, gco2h!(o) as *mut _);
      }
      t if t == LuaType::Function as i32 => {
        validateclosure(g, gco2cl!(o) as *mut _);
      }
      t if t == LuaType::UserData as i32 => {
        let u = gco2u!(o) as *const _ as *mut Udata;
        if !(*u).metatable.is_null() {
          validateobjref(g, o, obj2gco!((*u).metatable));
        }
      }
      t if t == LuaType::Thread as i32 => {
        validatestack(g, gco2th!(o) as *mut _);
      }
      t if t == LuaType::Buffer as i32 => {}
      t if t == LuaType::Proto as i32 => {
        validateproto(g, gco2p!(o) as *mut _);
      }
      t if t == LuaType::Upval as i32 => {
        let uv = gco2uv!(o) as *const _ as *mut UpVal;
        validateref(g, o, &*(*uv).v);
      }
      t if t == LuaType::Class as i32 => {
        validateclass(g, gco2class!(o) as *mut _);
      }
      t if t == LuaType::Object as i32 => {
        validateobject(g, gco2object!(o) as *mut _);
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }
}
