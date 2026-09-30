use core::{
  ffi::c_void,
  mem::{offset_of, size_of},
  ptr::addr_of_mut,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{lua_status::LuaStatus, lua_type::LuaType},
  functions::{lua_d_throw_ldo::lua_d_throw, newgcoblock::newgcoblock, newpage::newpage},
  macros::{asan_unpoison_memory_region::ASAN_UNPOISON_MEMORY_REGION, sizeclass::sizeclass},
  records::{g_cheader::GCheader, gc_object::GCObject, lua_page::lua_Page, lua_state::LuaState},
};

const K_GCO_LINK_OFFSET: usize =
  (size_of::<GCheader>() + size_of::<*mut u8>() - 1) & !(size_of::<*mut u8>() - 1);

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).global` 存活：`nsize` 须 ≥`K_GCO_LINK_OFFSET+size_of::<*mut u8>()`
/// （LUAU_ASSERT 前置），`memcat` 须为合法类别索引（`< memcatbytes 长度`，据此累加记账）；分配失败（block 空且 nsize>0）经
/// `lua_d_throw(ErrMem)` 抛错，调用方须处于受保护帧。返回的 GCObject 已由 `(*g).allgcopages`/page 链持有，尚未初始化头 tt/tag，
/// 调用方须随后 `luaC_init` 置类型方可被 GC 识别；`onallocate` 回调可再入。
/// cpp VM/src/lmem.cpp:564
pub unsafe fn lua_m_newgco(l: *mut LuaState, nsize: usize, memcat: u8) -> *mut GCObject {
  unsafe {
    LUAU_ASSERT!(nsize >= K_GCO_LINK_OFFSET + size_of::<*mut u8>());

    let g = (*l).global;
    let nclass = sizeclass!(nsize) as i32;

    let block = if nclass >= 0 {
      newgcoblock(l, nclass)
    } else {
      let page = &mut *newpage(
        l,
        addr_of_mut!((*g).allgcopages),
        (offset_of!(lua_Page, data) + nsize) as i32,
        nsize as i32,
        1,
      );

      let block = page.data.as_mut_ptr() as *mut u8;
      ASAN_UNPOISON_MEMORY_REGION!(block, page.block_size as usize);

      page.free_next -= page.block_size;
      page.busy_blocks += 1;
      block
    };

    if block.is_null() && nsize > 0 {
      lua_d_throw(l, LuaStatus::ErrMem as i32);
    }

    (*g).totalbytes = (*g).totalbytes.wrapping_add(nsize);
    let memcatbytes = &mut (*g).memcatbytes;
    memcatbytes[memcat as usize] = memcatbytes[memcat as usize].wrapping_add(nsize);

    if let Some(onallocate) = (*g).cb.onallocate {
      // DELIBERATE DEVIATION：cpp luaM_newgco_ 收 tt/tag 形参并透传
      // （lmem.cpp:598）；本实现分配器尚未线程化 tt/tag，以 LUA_T_ALL 哨兵
      // 占位，接入 per-type 统计时需贯穿签名
      onallocate(
        l,
        block.cast::<c_void>(),
        0,
        nsize,
        memcat,
        LuaType::ALL_SENTINEL,
        0,
      );
    }

    block as *mut GCObject
  }
}
