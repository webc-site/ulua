use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    enumbuffer::enumbuffer, enumclass::enumclass, enumclosure::enumclosure, enumobject::enumobject,
    enumproto::enumproto, enumstring::enumstring, enumtable::enumtable, enumthread::enumthread,
    enumudata::enumudata, enumupval::enumupval,
  },
  macros::{
    gco_2_buf::gco2buf, gco_2_cl::gco2cl, gco_2_class::gco2class, gco_2_h::gco2h,
    gco_2_object::gco2object, gco_2_p::gco2p, gco_2_th::gco2th, gco_2_ts::gco2ts, gco_2_u::gco2u,
    gco_2_uv::gco2uv,
  },
  records::{enum_context::EnumContext, gc_object::GCObject},
};

pub(crate) unsafe fn enumobj(ctx: *mut EnumContext, o: *mut GCObject) {
  unsafe {
    match (*o).gch.tt as i32 {
      t if t == LuaType::String as i32 => {
        enumstring(ctx, gco2ts!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Table as i32 => {
        enumtable(ctx, gco2h!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Function as i32 => {
        enumclosure(ctx, gco2cl!(o) as *const _ as *mut _);
      }
      t if t == LuaType::UserData as i32 => {
        enumudata(ctx, gco2u!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Thread as i32 => {
        enumthread(ctx, gco2th!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Buffer as i32 => {
        enumbuffer(ctx, gco2buf!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Class as i32 => {
        enumclass(ctx, gco2class!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Object as i32 => {
        enumobject(ctx, gco2object!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Proto as i32 => {
        enumproto(ctx, gco2p!(o) as *const _ as *mut _);
      }
      t if t == LuaType::Upval as i32 => {
        enumupval(ctx, gco2uv!(o) as *const _ as *mut _);
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }
}
