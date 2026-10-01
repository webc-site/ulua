use core::{ffi::c_void, mem::size_of, ptr::write_unaligned, slice::from_raw_parts_mut};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    ensure_stack::ensure_stack, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_u_newudata::lua_u_newudata,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, checkliveness::checkliveness,
    lua_c_check_gc::lua_c_check_gc, obj_2_gco::obj2gco, utag_idtor::UTAG_IDTOR,
  },
  records::lua_state::LuaState,
  type_aliases::lua_destructor::LuaDestructor,
};

/// # Safety
/// `l` 须为存活 `LuaState`；`dtor` 须为 `Some`（`api_check`，其函数指针在 udata 回收时被调），`sz`
/// 为请求的 payload 字节数（`sz+size_of::<LuaDestructor>()` 不得溢出，内部已钳位）；`lua_c_check_gc`/
/// `lua_u_newudata`/`ensure_stack` 可分配并把 udata 挂到 `(*l).top`，须在受保护帧内调用。cpp `lapi.cpp:1671`。
pub unsafe fn lua_newuserdatadtor(l: *mut LuaState, sz: usize, dtor: LuaDestructor) -> *mut c_void {
  unsafe {
    api_check!(l, dtor.is_some());
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);

    let dtor_size = size_of::<LuaDestructor>();
    // 溢出钳位：cpp `sz + sizeof(LuaDestructor)` 的回绕保护，等价 `saturating_add`
    let as_ = sz.saturating_add(dtor_size);

    let u = lua_u_newudata(l, as_, UTAG_IDTOR);
    // 析构指针以值镜像非对齐存于 payload 尾部（`lua_u_freeudata` 对称 `read_unaligned`
    // 读回）。切片视图把界长交给 `as_` 做边界检查，替代手写 `.add(sz)` 裸指针算术
    let payload = from_raw_parts_mut((*u).data.as_mut_ptr().cast::<u8>(), as_);
    write_unaligned(payload[sz..].as_mut_ptr().cast::<LuaDestructor>(), dtor);

    (*(*l).top).value.gc = obj2gco!(u);
    (*(*l).top).tt = LuaType::UserData as i32;
    checkliveness!((*l).global, (*l).top);
    api_incr_top!(l);

    (*u).data.as_mut_ptr().cast()
  }
}
