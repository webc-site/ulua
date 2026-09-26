use core::ffi::c_void;

use crate::{
  enums::value_view::ValueView, functions::index_2_addr::index_2_addr,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// Rust 内部读取（§11 方向）：`idx` 槽为 full userdata 且其 tag 等于 `tag` 时返回数据块首字节
/// 可变引用，否则 `None`。
///
/// 空指针哨兵收口为 `Option<&'a mut c_void>`：full userdata 的 `data` 柔性数组是 VM
/// 持有的真实可写内存（首字节地址恒非空），引用寿命 `'a` 随槽解耦（同 `VmFrame::slots_mut`）。
/// `(*u).tag` 是 u8，`tag` 入参是 i32，把字段提升到 i32 再比较。
///
/// # Safety
/// `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为栈内合法 StkId，命中 full userdata 时才读
/// `(*u).tag` 与 `data`（`uvalue` 解引用的 Udata 须存活）。返回引用仅在 userdata 存活期间有效。
/// 不抛错/不分配。cpp/VM/src/lapi.cpp:622 lua_touserdatatagged。
pub unsafe fn lua_touserdatatagged_ref<'a>(
  l: *mut LuaState,
  idx: i32,
  tag: i32,
) -> Option<&'a mut c_void> {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    match ValueView::from_tvalue(&*o) {
      ValueView::Userdata(u) if (*u).tag as i32 == tag => (*u).data.as_ptr() as *mut c_void,
      _ => return None,
    }
    .as_mut()
  }
}

/// C-ABI 适配垫片：把 [`lua_touserdatatagged_ref`] 的 `Option` 折算回 lua.h 约定的 `void*`/`NULL`。
///
/// # Safety
/// 同 [`lua_touserdatatagged_ref`]。
pub unsafe fn lua_touserdatatagged(l: *mut LuaState, idx: i32, tag: i32) -> *mut c_void {
  use core::ptr::null_mut;
  // Safety: 转发同契约的 `lua_touserdatatagged_ref`；仅把 None 折算回 NULL。
  unsafe { lua_touserdatatagged_ref(l, idx, tag).map_or(null_mut(), |r| r as *mut c_void) }
}
