use core::{ffi::c_void, ptr::null};

use crate::{
  enums::value_view::ValueView,
  functions::index_2_addr::index_2_addr,
  macros::{gcvalue::gcvalue, iscollectable::iscollectable},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为栈内合法 StkId（越界返回可读的 `LUA_O_NILOBJECT`）；
/// 按类型分支解引用 union：full userdata 取 `(*uvalue).data`、light userdata 取 `pvalue`、可回收对象取 `gcvalue`，
/// 返回的 `*const c_void` 仅在对应对象存活期间有效，仅用于 identity 比较。cpp/VM/src/lapi.cpp:665 lua_topointer。
pub unsafe fn lua_topointer(l: *mut LuaState, idx: i32) -> *const c_void {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    match ValueView::from_tvalue(&*o) {
      ValueView::Userdata(u) => (*u).data.as_ptr() as *const c_void,
      ValueView::LightUserdata { pointer, .. } => pointer as *const c_void,
      // 其余可回收 tag（string/table/function/thread/buffer 及 Class/Object 等
      // Other 逃生舱）与 C 的 iscollectable 兜底一致：取 GCObject 指针仅作
      // identity 比较。
      _ if iscollectable!(o) => gcvalue!(o) as *const c_void,
      _ => null(),
    }
  }
}
