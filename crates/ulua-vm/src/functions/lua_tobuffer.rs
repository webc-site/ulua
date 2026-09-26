use core::ffi::c_void;

use crate::{
  enums::value_view::ValueView, functions::index_2_addr::index_2_addr,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// cpp `lua_tobuffer`（`VM/src/lapi.cpp`）：`idx` 槽为 buffer 时返回其数据块首字节可变引用，
/// 并把数据长度写进 `len` 出参（`len` 可为 null，此时仅取址不写长度，与 cpp 传 `nullptr` 一致）；
/// 非 buffer 返回 `None`。
///
/// 空指针哨兵收口为 `Option<&'a mut c_void>`：数据块是 VM 持有的真实可写内存，引用寿命
/// `'a` 随槽解耦（同 `VmFrame::slots_mut`）。长度出参保持 cpp 的 `size_t*` C 签名（可选出参），
/// 仅 `Some` 路径写入，与 `None` 时不触碰 `*len` 的既有行为逐位一致。
///
/// # Safety
/// `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为栈内合法 StkId；`len` 须为可写 `usize`
/// 槽或 null。返回引用指向 buffer 自有内存，在该 buffer 存活期间有效。
pub unsafe fn lua_tobuffer<'a>(
  l: *mut LuaState,
  idx: i32,
  len: *mut usize,
) -> Option<&'a mut c_void> {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    match ValueView::from_tvalue(&*o) {
      ValueView::Buffer(b) => {
        if !len.is_null() {
          *len = (*b).len as usize;
        }
        (*b).data.as_ptr() as *mut c_void
      }
      _ => return None,
    }
    .as_mut()
  }
}
