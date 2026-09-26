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
/// # Safety
/// `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为栈内合法 StkId。仅原样带出存储的地址值，
/// 不解引用。
pub unsafe fn lua_tolightuserdata_ref(l: *mut LuaState, idx: i32) -> Option<*mut c_void> {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

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
/// # Safety
/// 同 [`lua_tolightuserdata_ref`]。
pub unsafe fn lua_tolightuserdata(l: *mut LuaState, idx: i32) -> *mut c_void {
  // Safety: 转发同契约的 `lua_tolightuserdata_ref`；仅把 None 折算回 NULL。
  unsafe { lua_tolightuserdata_ref(l, idx).unwrap_or(null_mut()) }
}
