use core::{ffi::c_uint, ptr::copy_nonoverlapping};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::lua_m_newgco, lua_m_toobig::lua_m_toobig, lua_s_resize::lua_s_resize},
  macros::{
    atom_undef::ATOM_UNDEF, lua_c_init::luaC_init, maxssize::MAXSSIZE, sizestring::sizestring,
  },
  records::{lua_state::LuaState, t_string::tstring},
};

/// # Safety
/// 调用方须保证：`l` 存活且处于受保护帧（分配 OOM、串表扩容均经 `l` 抛错）；`str_.len() <= MAXSSIZE`
/// （超限抛 string too long）；`h` 必须是 lua_s_hash(str_) 的结果（错配会使串落错桶、破坏去重并使
/// 后续查找失败）。cpp lstring.cpp:71 `newlstr`
pub(crate) unsafe fn newlstr(l: *mut LuaState, str_: &[u8], h: c_uint) -> *mut tstring {
  // Safety: 契约保证 `l` 存活；新串先挂桶链再发布，写入仅限刚分配对象自身
  unsafe {
    let len = str_.len();
    if len > MAXSSIZE as usize {
      lua_m_toobig(l);
    }

    let ts = lua_m_newgco(l, sizestring(len), (*l).activememcat) as *mut tstring;

    luaC_init!(l, ts, LuaType::String as i32);
    // Safety: `ts` 为刚分配的存活对象，头域与 data 区 sizestring(len) 可写
    let s = &mut *ts;
    s.atom = ATOM_UNDEF as i16;
    s.hash = h;
    s.len = len as c_uint;

    // len==0 时跳过拷贝（零长度 copy_nonoverlapping 是 UB）；终止 NUL 仍写入，
    // TString 的 C 布局（data[len]=0）由 JIT 偏移访问与 capi 侧依赖，保持不变
    if len != 0 {
      // Safety: `str_` 切片 len 字节可读，data 区 len+1 可写（同上契约）
      copy_nonoverlapping(str_.as_ptr(), s.data.as_mut_ptr() as *mut u8, len);
    }
    *s.data.as_mut_ptr().add(len) = 0;

    let tb = &mut (*(*l).global).strt;
    tb.link_front(h, ts);
    if tb.wants_growth() {
      let target = tb.doubled_size();
      lua_s_resize(l, target); // too crowded（cpp：经 l 受保护抛错的翻倍扩容）
    }

    ts
  }
}
