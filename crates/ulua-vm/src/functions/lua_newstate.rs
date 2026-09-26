//! Source: `VM/src/lstate.cpp:184-290` (hand-ported)

use core::{
  ffi::c_void,
  mem::{size_of, zeroed},
  ptr::{from_mut, null_mut},
};

use ulua_common::fflag;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    close_state::close_state, f_luaopen::f_luaopen,
    lua_d_rawrunprotected_ldo::lua_d_rawrunprotected, preinit_state::preinit_state,
  },
  macros::{
    fixedbit::FIXEDBIT, gc_spause::GCSPAUSE, luai_gcgoal::LUAI_GCGOAL,
    luai_gcstepmul::LUAI_GCSTEPMUL, luai_gcstepsize::LUAI_GCSTEPSIZE, white_0_bit::WHITE0BIT,
  },
  records::{
    lg::LG, lua_execution_callback_storage::LUA_EXECUTION_CALLBACK_STORAGE, lua_state::LuaState,
  },
  type_aliases::lua_alloc::LuaAlloc,
};

/// 以宿主分配器 `f` 创建全新 VM 状态：分配 `LG`（`global_State` + 主
/// `lua_State`）并逐格初始化，随后在保护帧内执行 `f_luaopen`。
///
/// 本函数是 VM 侧唯一的宿主分配器跨界点（cpp `lstate.cpp:184` 同款），返回
/// `null` 表示分配失败，与 C API `lua_newstate` 的失败语义逐格一致。
///
/// # Safety
///
/// `f` 为空或满足 cpp `lua_Alloc` 契约的函数：`ptr` 为 null 或此前由它返回且
/// 未被转交的块；`nsize == 0` 时返回 null；任意时刻可从被分配块内回调。
/// `ud` 原样透传给 `f` 与后续全部重分配（宿主须保证其存活于状态整个生命周期）。
pub unsafe fn lua_newstate(f: LuaAlloc, ud: *mut c_void) -> *mut LuaState {
  let Some(falloc) = f else {
    return null_mut();
  };

  // Safety: FFI 边界——契约保证 `falloc` 满足 cpp lua_Alloc 语义：null ptr + 0→size
  // 的请求即 malloc，失败返回 null；`ud` 仅透传，本侧不解引用
  let block = unsafe { falloc(ud, null_mut(), 0, size_of::<LG>()) };
  if block.is_null() {
    return null_mut();
  }

  // Safety: FFI 边界——`block` 非空、按 LG 对齐且容纳整个 LG；此后的全部读写经
  // `l`/`g` 两个视图抵达同一块内存（与 cpp `LG* l; global_State* g = &l->g;` 的
  // 别名形态一致），`close_state` 负责失败路径把整块经 `falloc` 归还
  unsafe {
    let l = block as *mut LuaState;
    let g = &mut (*block.cast::<LG>()).g;

    (*l).hdr.tt = LuaType::Thread as u8;
    g.currentwhite = ((1 << WHITE0BIT) | (1 << FIXEDBIT)) as u8;
    (*l).hdr.marked = g.currentwhite;
    (*l).hdr.memcat = 0;
    preinit_state(l, from_mut(g));
    g.frealloc = f;
    g.ud = ud;
    g.mainthread = l;
    g.uvhead.u.open.prev = &raw mut g.uvhead;
    g.uvhead.u.open.next = &raw mut g.uvhead;
    g.gc_threshold = 0; // mark it as unfinished state
    g.registryfree = 0;
    g.rngstate = 0;
    g.ptrenckey = [1, 0, 0, 0];
    g.strt.size = 0;
    g.strt.nuse = 0;
    g.strt.hash = null_mut();
    g.pseudotemp.set_nil();
    g.registry.set_nil();
    g.gcstate = GCSPAUSE as u8;
    g.gray = null_mut();
    g.grayagain = null_mut();
    g.weak = null_mut();
    g.totalbytes = size_of::<LG>();
    g.gcgoal = LUAI_GCGOAL;
    g.gcstepmul = LUAI_GCSTEPMUL;
    g.gcstepsize = LUAI_GCSTEPSIZE << 10;

    for page in g.freepages.iter_mut() {
      *page = null_mut();
    }
    for page in g.freegcopages.iter_mut() {
      *page = null_mut();
    }

    g.allpages = null_mut();
    g.allgcopages = null_mut();
    g.sweepgcopage = null_mut();

    for mt in g.mt.iter_mut() {
      *mt = null_mut();
    }

    g.udatagc.iter_mut().for_each(|gc| *gc = None);
    g.udatamt.iter_mut().for_each(|mt| *mt = null_mut());

    for direct in g.udatadirect.iter_mut() {
      direct.indextm.set_nil();
      direct.newindextm.set_nil();
      direct.namecalltm.set_nil();
      direct.index = None;
      direct.newindex = None;
      direct.namecall = None;
    }

    g.lightuserdataname
      .iter_mut()
      .for_each(|name| *name = null_mut());

    if fflag::LuauDirectFieldGet.get() {
      g.udatadirectfields
        .iter_mut()
        .for_each(|field| *field = null_mut());
    }

    g.memcatbytes.fill(0);
    g.memcatbytes[0] = size_of::<LG>();

    // Safety: lua_Callbacks/lua_ExecutionCallbacks 为 #[repr(C)] 回调表，
    // cpp 即 `memset` 清零；全零 = 全部回调未挂，位型合法
    g.cb = zeroed(); // lua_Callbacks()
    g.ecb = zeroed(); // lua_ExecutionCallbacks()
    g.ecbdata.bytes = [0; LUA_EXECUTION_CALLBACK_STORAGE];

    g.gcstats = Default::default(); // GCStats()
    g.lastprotoid = 1;

    if lua_d_rawrunprotected(l, Some(f_luaopen), null_mut()) != 0 {
      // memory allocation error: free partial state
      close_state(l);
      return null_mut();
    }

    ulua_common::LUAU_ASSERT!(g.gc_threshold != 0);
    l
  }
}
