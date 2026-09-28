use core::ffi::c_void;

use crate::{
  enums::value_view::ValueView, functions::index_2_addr::index_2_addr,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// cpp `lua_touserdata`（`VM/src/lapi.cpp`）：`idx` 槽为 userdata（full 或 light）时返回其载荷
/// 地址，否则 `None`。
///
/// 空指针哨兵收口为 `Option<&'a mut c_void>`（§11 B 方向）：载荷区是 VM 持有的真实可写
/// 内存（full userdata 柔性数组 `data`、或 light userdata 存下的地址值），引用寿命 `'a` 随槽
/// 解耦（同 `VmFrame::slots_mut`）。light userdata 存的地址本身可为 null——`.as_mut()` 将其归一
/// 为 `None`，与所有调用点既有的 `NonNull::new`/`is_null` 语义逐位一致。
///
/// # Safety
/// `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为栈内合法 StkId。返回引用仅在 userdata
/// 载荷存活期间有效，调用方须保证期间无 GC 移动/释放该对象。
pub unsafe fn lua_touserdata<'a>(l: *mut LuaState, idx: i32) -> Option<&'a mut c_void> {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    match ValueView::from_tvalue(&*o) {
      // Udata 的 data 是结构体尾部的 char[1]柔性数组，取其首字节可变引用。
      ValueView::Userdata(u) => (*u).data.as_ptr() as *mut c_void,
      ValueView::LightUserdata { pointer, .. } => pointer,
      _ => return None,
    }
    .as_mut()
  }
}
