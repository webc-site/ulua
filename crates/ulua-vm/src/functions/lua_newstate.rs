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
/// `null` 表示分配失败，与 C API `lua_newstate` 的失败语义逐格一致。确属 C ABI
/// 交叉点（入参为 `lua_Alloc` 函数指针 + 不透明 `ud`），故保留 `unsafe` 签名；
/// 体内 unsafe 收敛为 7 处单行/单表达式最小岛（分配、块视图建立、preinit、
/// 回调表 zeroed 清零、保护帧执行、失败回收），其余初始化全部走安全字段写。
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

  // SAFETY: FFI 边界——契约保证 `falloc` 满足 cpp lua_Alloc 语义：null ptr + 0→size
  // 的请求即 malloc，失败返回 null；`ud` 仅透传，本侧不解引用
  let block = unsafe { falloc(ud, null_mut(), 0, size_of::<LG>()) };
  if block.is_null() {
    return null_mut();
  }

  // `l` 保持裸指针供 FFI 形接口与返回值使用；字段级读写经 `l_hdr`/`g` 两个引用视图
  let l = block.cast::<LuaState>();
  // SAFETY: FFI 边界——`block` 非空、按 LG 对齐且容纳整个 LG；`l_hdr`/`g` 是同一块
  // `LG { l, g }` 的两个不相交字段视图（与 cpp `LG* l; global_State* g = &l->g;` 的
  // 别名形态一致），`close_state` 负责失败路径把整块经 `falloc` 归还
  let (l_hdr, g) = unsafe { (&mut (*l).hdr, &mut (*block.cast::<LG>()).g) };

  l_hdr.tt = LuaType::Thread as u8;
  g.currentwhite = ((1 << WHITE0BIT) | (1 << FIXEDBIT)) as u8;
  l_hdr.marked = g.currentwhite;
  l_hdr.memcat = 0;
  // SAFETY: `preinit_state` 契约——`l` 为刚分配、尚未初始化的主线程，`from_mut(g)`
  // 即同块 global_State 的裸指针视图，二者在本函数构造期内存活
  unsafe { preinit_state(l, from_mut(g)) };
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

  g.freepages.fill(null_mut());
  g.freegcopages.fill(null_mut());

  g.allpages = null_mut();
  g.allgcopages = null_mut();
  g.sweepgcopage = null_mut();

  g.mt.fill(null_mut());
  g.udatagc.fill(None);
  g.udatamt.fill(null_mut());
  g.lightuserdataname.fill(null_mut());

  for direct in g.udatadirect.iter_mut() {
    direct.indextm.set_nil();
    direct.newindextm.set_nil();
    direct.namecalltm.set_nil();
    direct.index = None;
    direct.newindex = None;
    direct.namecall = None;
  }

  if fflag::LuauDirectFieldGet.get() {
    g.udatadirectfields.fill(null_mut());
  }

  g.memcatbytes.fill(0);
  g.memcatbytes[0] = size_of::<LG>();

  // SAFETY: `mem::zeroed` 为 unsafe fn（memset 语义）；lua_Callbacks/lua_ExecutionCallbacks
  // 为 #[repr(C)] 回调表，cpp 即 `memset` 清零——全零 = 全部回调未挂，位型合法
  g.cb = unsafe { zeroed() }; // lua_Callbacks()
  g.ecb = unsafe { zeroed() }; // lua_ExecutionCallbacks()
  g.ecbdata.bytes = [0; LUA_EXECUTION_CALLBACK_STORAGE];

  g.gcstats = Default::default(); // GCStats()
  g.lastprotoid = 1;

  // SAFETY: `lua_d_rawrunprotected` 契约——`l` 的 base_ci/ci 已由 preinit_state
  // 置备，`f_luaopen` 为在其保护帧内执行的初始化回调（可抛 ErrMem）
  if unsafe { lua_d_rawrunprotected(l, Some(f_luaopen), null_mut()) } != 0 {
    // memory allocation error: free partial state
    // SAFETY: `close_state` 契约——仅在本状态创建失败、f_luaopen 未完成的路径调用，
    // 经宿主分配器归还整块 LG
    unsafe { close_state(l) };
    return null_mut();
  }

  ulua_common::LUAU_ASSERT!(g.gc_threshold != 0);
  l
}
