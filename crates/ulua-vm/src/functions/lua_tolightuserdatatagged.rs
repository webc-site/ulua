use core::{ffi::c_void, ptr::null_mut};

use crate::{
  enums::value_view::ValueView, functions::index_2_addr::index_2_addr,
  macros::lu_tag_iterator::LU_TAG_ITERATOR, records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// cpp `lua_tolightuserdatatagged`（`VM/src/lapi.cpp`）：`idx` 槽为 light userdata 且其 user tag
/// 等于 `tag` 时返回其存储的裸指针，否则 `None`。
///
/// 空指针哨兵收口为 `Option<*mut c_void>`：light userdata 的载荷是**存在 TValue 里的地址
/// 值**（无 backing 对象、可为 null），并非可借用的活内存，故用指针值而非 `&mut`。`Some(p)` 含
/// p 为 null 的情形（用户 push 了 null light userdata），`None` 严格表示"非 light userdata 或
/// tag 失配"——较 cpp 无法区分二者更精确，对以 null 判失败的既有调用点行为不变。
///
/// # Safety
/// `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为栈内合法 StkId。仅比对 tag、原样带出
/// 存储的地址值，不解引用。
pub unsafe fn lua_tolightuserdatatagged(
  l: *mut LuaState,
  idx: i32,
  tag: i32,
) -> Option<*mut c_void> {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    match ValueView::from_tvalue(&*o) {
      ValueView::LightUserdata { pointer, tag: t } if t == tag => Some(pointer),
      // 内建迭代器「无更多值」标记即 `LU_TAG_ITERATOR` + null 载荷的 lightuserdata：
      // tag 命中时与收敛前逐位一致地返回 `Some(null)`（null 仅限本 lua_*.rs 垫片层
      // 按协议带出）；tag 失配走下方 `None` 兜底，与原行为相同。
      ValueView::IteratorDone if tag == LU_TAG_ITERATOR => Some(null_mut()),
      _ => None,
    }
  }
}
