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

/// gch.tt 匹配用的 LuaType 判别值常量（i32）
const T_STRING: i32 = LuaType::String as i32;
const T_TABLE: i32 = LuaType::Table as i32;
const T_FUNCTION: i32 = LuaType::Function as i32;
const T_USERDATA: i32 = LuaType::UserData as i32;
const T_THREAD: i32 = LuaType::Thread as i32;
const T_BUFFER: i32 = LuaType::Buffer as i32;
const T_CLASS: i32 = LuaType::Class as i32;
const T_OBJECT: i32 = LuaType::Object as i32;
const T_PROTO: i32 = LuaType::Proto as i32;
const T_UPVAL: i32 = LuaType::Upval as i32;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn dumpobj(f: *mut c_void, o: *mut GCObject) {
  // gch.tt 的 LuaType 判别值是编译期常量，直接常量匹配（cpp switch(o->gch.tt) 同款）
  unsafe {
    match (*o).gch.tt as i32 {
      T_STRING => {
        dumpstring(f, gco2ts!(o) as *const _ as *mut _);
      }
      T_TABLE => {
        dumptable(f, gco2h!(o) as *mut _);
      }
      T_FUNCTION => {
        dumpclosure(f, gco2cl!(o) as *mut _);
      }
      T_USERDATA => {
        dumpudata(f, gco2u!(o) as *mut _);
      }
      T_THREAD => {
        dumpthread(f, gco2th!(o) as *mut _);
      }
      T_BUFFER => {
        dumpbuffer(f, gco2buf!(o) as *const _ as *mut _);
      }
      T_CLASS => {
        dumpclass(f, gco2class!(o) as *mut _);
      }
      T_OBJECT => {
        dumpobject(f, gco2object!(o) as *mut _);
      }
      T_PROTO => {
        dumpproto(f, gco2p!(o) as *mut _);
      }
      T_UPVAL => {
        dumpupval(f, gco2uv!(o) as *mut _);
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }
}
