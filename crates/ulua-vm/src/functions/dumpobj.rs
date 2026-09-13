use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    dumpbuffer::dumpbuffer, dumpclass::dumpclass, dumpclosure::dumpclosure, dumpobject::dumpobject,
    dumpproto::dumpproto, dumpstring::dumpstring, dumptable::dumptable, dumpthread::dumpthread,
    dumpudata::dumpudata, dumpupval::dumpupval,
  },
  macros::{
    gco_2_buf::gco2buf, gco_2_cl::gco2cl, gco_2_class::gco2class, gco_2_h::gco2h,
    gco_2_object::gco2object, gco_2_p::gco2p, gco_2_th::gco2th, gco_2_ts::gco2ts, gco_2_u::gco2u,
    gco_2_uv::gco2uv,
  },
  records::gc_object::GCObject,
};

pub(crate) unsafe fn dumpobj(f: *mut c_void, o: *mut GCObject) {
  unsafe {
    match (*o).gch.tt as i32 {
      t if t == LuaType::String as i32 => {
        dumpstring(f, gco2ts!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Table as i32 => {
        dumptable(f, gco2h!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Function as i32 => {
        dumpclosure(f, gco2cl!(o) as *const _ as *mut _);
      }
      t if t == LuaType::UserData as i32 => {
        dumpudata(f, gco2u!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Thread as i32 => {
        dumpthread(f, gco2th!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Buffer as i32 => {
        dumpbuffer(f, gco2buf!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Class as i32 => {
        dumpclass(f, gco2class!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Object as i32 => {
        dumpobject(f, gco2object!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Proto as i32 => {
        dumpproto(f, gco2p!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Upval as i32 => {
        dumpupval(f, gco2uv!(o) as *const _ as *mut _);
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }
}
