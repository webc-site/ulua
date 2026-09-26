use core::{
  ptr::{addr_of_mut, null_mut},
  slice::from_raw_parts_mut,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{lua_m_freearray::luaM_freearray, lua_m_newarray::luaM_newarray},
  records::{lua_state::LuaState, stringtable::Stringtable, t_string::tstring},
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).global.strt` 有效；`newsize` 须为 2 的幂且 `>0`
/// （`bucket_of` 取模语义依赖），`luaM_newarray`/`luaM_freearray` 可 OOM 抛错；调用须在
/// GC 之外、所有内联/外联 `tstring` 仍挂在旧桶链上时进行（逐一 rehash 到新表）。
/// cpp `lstring.cpp:45`。
pub unsafe fn lua_s_resize(l: *mut LuaState, newsize: i32) {
  unsafe {
    let newhash = luaM_newarray!(l, newsize as usize, *mut tstring, 0);
    // Safety: luaM_newarray 契约为 newsize 个 *mut tstring 槽的已分配数组；
    // 新桶批量清零，替代 cpp `newhash[i] = NULL` 下标循环
    let fresh = from_raw_parts_mut(newhash, newsize as usize);
    fresh.fill(null_mut());

    let tb: *mut Stringtable = addr_of_mut!((*(*l).global).strt);
    // f_luaopen 首扩时 size==0 且 hash 未分配：buckets() 的 null 守卫给空视图。
    // 旧桶按序 rehash，链内仍按 cpp 以 next 驱动、头插新桶（遍历/落位次序逐位保持）。
    let old_buckets = (*tb).buckets();
    for &head in old_buckets {
      let mut p = head;
      while !p.is_null() {
        // Safety: 表不变式——链上节点皆存活 TString
        let next = (*p).next;
        let h = (*p).hash;
        let b = Stringtable::bucket_of(h, newsize);
        LUAU_ASSERT!((h % newsize as u32) as i32 == b.0 as i32);
        (*p).next = fresh[b.0];
        fresh[b.0] = p;
        p = next;
      }
    }

    luaM_freearray!(l, (*tb).hash, (*tb).size as usize, *mut tstring, 0);
    (*tb).size = newsize;
    (*tb).hash = newhash;
  }
}
