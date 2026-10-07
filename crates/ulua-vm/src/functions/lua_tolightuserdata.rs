use core::{ffi::c_void, ptr::null_mut};

use crate::{
  enums::value_view::ValueView, functions::index_2_addr::index_2_addr,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// Rust 内部读取（§11 方向）：`idx` 槽为 light userdata 时返回其存储的裸指针，否则 `None`。
///
/// light userdata 的载荷是**存在 TValue 里的地址值**（无 backing 对象、可为 null），并非可借用
/// 的活内存，故用 `Option<*mut c_void>` 指针值而非 `&mut`：`Some(p)` 含 p 为 null 的情形，`None`
/// 严格表示"非 light userdata"。
///
/// r16-v4b 引用形前移取 `&LuaState`（纯读槽载荷，不改 VM；`l` 存活由类型承载），
/// `unsafe fn` 消亡转 safe fn，unsafe 内移到真实裸触点（StkId 解引用取 `&TValue`）。
/// 调用序契约（正确性，非内存安全）：`idx` 经 `index_2_addr` 解析为栈内合法 StkId
/// （越界得 `LUA_O_NILOBJECT` 哨兵亦可读），所得槽在使用点可读（栈未重分配）。
pub fn lua_tolightuserdata_ref(l: &LuaState, idx: i32) -> Option<*mut c_void> {
  let o: StkId = index_2_addr(l, idx);

  // SAFETY: 契约保证 `idx` 解析出的槽（含哨兵）在使用点可读；块内仅该槽一次只读
  // 视图构造（`from_tvalue` 按 tag 读载荷），原样带出存储的地址值、不解引用。
  unsafe {
    match ValueView::from_tvalue(&*o) {
      ValueView::LightUserdata { pointer, .. } => Some(pointer),
      // 内建迭代器「无更多值」标记槽本质是 null 载荷的 lightuserdata：与收敛前
      // （匹配 `LightUserdata { pointer: null }`）逐位一致，仍返回 `Some(null)`。
      // null 仅限本 lua_*.rs C-ABI 垫片层按协议带出，调用方不做解引用。
      ValueView::IteratorDone => Some(null_mut()),
      _ => None,
    }
  }
}

/// C-ABI 适配垫片：把 [`lua_tolightuserdata_ref`] 的 `Option` 折算回 lua.h 约定的 `void*`/`NULL`。
///
/// 保留 `*mut` 裸形透传（C-ABI 折算面，r16-v4b 裁决不收编）；对被调方的引用实参在
/// 转发处就地重建短借，借用窗于该调用语句即时结束。
///
/// # Safety
/// `l` 须为存活 LuaState（非空、对齐、调用期间单线程独占驱动——`&*l` 引用重建前提）；
/// `idx` 契约同 [`lua_tolightuserdata_ref`]。
pub unsafe fn lua_tolightuserdata(l: *mut LuaState, idx: i32) -> *mut c_void {
  // Safety: 契约保证 `l` 为存活 LuaState，本帧 `&*l` 重建只读短借即结束借用窗口后
  // 转调 safe 化的 `lua_tolightuserdata_ref`；仅把 None 折算回 NULL。
  // 既有约定（review.md §2）：c-API 边界签名折返——lua.h 约定返回 `void*`，Option::None/迭代器 done 折算为 NULL，本垫片层按协议带出、调用方不解引用
  lua_tolightuserdata_ref(unsafe { &*l }, idx).unwrap_or(null_mut())
}
