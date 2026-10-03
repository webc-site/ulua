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
/// r16-v21 收形：首参转 `&mut LuaState`——`l` 的存活与独占由类型承载；解耦的借出寿命 `'a` 仍是
/// 类型表达不了的内存契约，故本核心的 `# Safety` 契约位保留，调用前提见下（形制对齐
/// `lua_tobuffer_bytes_ref`）。
///
/// # Safety
/// `l` 的存活与独占已由 `&mut LuaState` 承载；`idx` 须经 `index_2_addr` 解析为栈内合法 StkId，
/// 命中 full userdata 时才读 `(*u).tag` 与 `data`（`uvalue` 解引用的 Udata 须存活）。借出寿命
/// `'a` 有效期内该 userdata 值须始终为存活栈槽值，否则悬垂借用。
/// 不抛错/不分配。cpp/VM/src/lapi.cpp:622 lua_touserdatatagged。
pub(crate) unsafe fn lua_touserdatatagged_ref<'a>(
  l: &mut LuaState,
  idx: i32,
  tag: i32,
) -> Option<&'a mut c_void> {
  // SAFETY: 契约即本函数 `# Safety` 所列——`l` 存活由 `&mut` 承载，`idx` 解得栈内合法槽，
  // 命中的 Udata 在借出窗内存活；`index_2_addr` 只读换算、本窗不穿插任何写点。
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
/// 消费位实测在 ulua-analysis（`is_type_user_data`/`get_type_user_data`）与 ulua-conformance
/// 测试门面，皆以裸 `LuaState*` 传形（跨 crate 收形属出范围），故本垫片按 lua.h 镜像契约保留
/// `*mut LuaState` 形，仅在转调收形后的 ref 核心处一次 `&mut *l` 重建引用、不跨调用持有。
///
/// # Safety
/// 同 [`lua_touserdatatagged_ref`]，另 `l` 须为指向存活 `LuaState` 的非空指针。
pub unsafe fn lua_touserdatatagged(l: *mut LuaState, idx: i32, tag: i32) -> *mut c_void {
  use core::ptr::null_mut;
  // SAFETY: 契约保证 `l` 非空且指向存活 `LuaState`；`&mut *l` 一次性重借用即 ref 核心期望的
  // 接收者形，本帧不再解引用该指针；仅把 None 折算回 NULL。
  unsafe { lua_touserdatatagged_ref(&mut *l, idx, tag).map_or(null_mut(), |r| r as *mut c_void) }
}
