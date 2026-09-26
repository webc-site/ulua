use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_b_freebuffer::lua_b_freebuffer, lua_e_freethread::lua_e_freethread,
    lua_f_freeclosure::lua_f_freeclosure, lua_f_freeproto::lua_f_freeproto,
    lua_f_freeupval::lua_f_freeupval, lua_h_free::lua_h_free, lua_r_freeclass::lua_r_freeclass,
    lua_r_freeobject::lua_r_freeobject, lua_s_free::lua_s_free, lua_u_freeudata::lua_u_freeudata,
  },
  records::{
    gc_object::{
      GCObject, try_as_buffer_ptr, try_as_class_ptr, try_as_closure_ptr, try_as_object_ptr,
      try_as_proto_ptr, try_as_string_ptr, try_as_table_ptr, try_as_thread_ptr, try_as_udata_ptr,
      try_as_upval_ptr,
    },
    lua_page::lua_Page,
    lua_state::LuaState,
  },
};

/// # Safety
/// `l` 须为存活 LuaState（`(*l).global` 有效，释放需记账），`o` 须指向待回收的存活 GCObject 且类型已知；
/// `page` 为 `o` 所属的 lua_Page，释放路径会把内存归还该页。须在 GC sweep 阶段独占调用。cpp/VM/src/lgc.cpp:769 freeobj。
pub(crate) unsafe fn freeobj(l: *mut LuaState, o: *mut GCObject, page: *mut lua_Page) {
  // Safety: 契约保证 `l`/`o`/`page` 一致且 `o` 的 tt 与其实际类型相符，各分支仅释放 `o` 自身内存
  unsafe {
    let Some(tt) = (*o).lua_type() else {
      LUAU_ASSERT!(false);
      return;
    };
    match tt {
      LuaType::Proto => lua_f_freeproto(l, try_as_proto_ptr(o).unwrap(), page),
      LuaType::Function => lua_f_freeclosure(l, try_as_closure_ptr(o).unwrap(), page),
      LuaType::Upval => lua_f_freeupval(l, try_as_upval_ptr(o).unwrap(), page),
      LuaType::Table => lua_h_free(l, try_as_table_ptr(o).unwrap(), page),
      LuaType::Thread => {
        let th = try_as_thread_ptr(o).unwrap();
        LUAU_ASSERT!(th != l && th != (*(*l).global).mainthread);
        lua_e_freethread(l, th, page);
      }
      LuaType::String => lua_s_free(l, try_as_string_ptr(o).unwrap().cast(), page),
      LuaType::UserData => lua_u_freeudata(l, try_as_udata_ptr(o).unwrap(), page),
      LuaType::Buffer => lua_b_freebuffer(l, try_as_buffer_ptr(o).unwrap().cast(), page),
      LuaType::Class => lua_r_freeclass(l, try_as_class_ptr(o).unwrap(), page),
      LuaType::Object => lua_r_freeobject(l, try_as_object_ptr(o).unwrap(), page),
      _ => LUAU_ASSERT!(false),
    }
  }
}
