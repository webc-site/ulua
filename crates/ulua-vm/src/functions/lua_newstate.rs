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
    global_state::PTRENCKEY_INIT, lg::LG,
    lua_execution_callback_storage::LUA_EXECUTION_CALLBACK_STORAGE, lua_state::LuaState,
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
  g.ptrenckey = PTRENCKEY_INIT;
  g.strt.size = 0;
  g.strt.nuse = 0;
  // 既有约定（review.md §2）：LG 堆对象 POD 字段初值 + GC 页/块链表与灰/弱链表头结构哨兵（gray/grayagain/weak/freepages/allpages/mt 等）；alloc 回调返回 null 的契约见上方 SAFETY 注（46-47 行），此处裸指针均作空/未挂链哨兵，勿改 Option；strt.hash 已按 §2 规则 1 终态改型为 Option<NonNull>（None=未分配，与 size==0 同现，读写契约收拢于 records/stringtable.rs 字段 doc）
  g.strt.hash = None;
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

  // r16-v12 根修：LG 块经宿主分配器取得、逐字段初始化，cpp lstate.cpp:255-257 在
  // 此处 `g->gcmetrics = GCMetrics();`（全零），本移植此前漏写该字段——
  // luai_gcmetrics 点亮时 gcmetrics 残留堆上垃圾位，record_gc_state_step 的
  // usize 累加（assistwork += work 等）在 debug 偶发「attempt to add with
  // overflow」红点。GCMetrics 派生 Default，各字段零值与 cpp 成员默认初始化
  // （lstate.h:145-155 全 = 0）逐位同值。
  #[cfg(feature = "luai_gcmetrics")]
  {
    g.gcmetrics = Default::default(); // GCMetrics()
  }

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

// r16-v12 定向回归（d1 形制：夹具+边界+值域门）：钉住「LG 经宿主分配器取得后
// gcmetrics 字段必须显式零初始化」这一操作数域。夹具确定性地把一块预先涂满
// 0xFF 脏位的等大小分配块交予 `lua_newstate` 作 LG 块，令「漏初始化」暴露为
// 可见脏值而非碰运气的零页；边界臂断言开态即全零；驱动臂跑一整轮 assist GC
// 步进，断言工作量累加器停留在堆字节值域（debug 下脏基值会先在
// `record_gc_state_step` 的 usize `+=` 处 add-with-overflow 爆出）。
// 选址注记：被测面为 `lua_newstate` 体内字段初始化序，`gcmetrics`/`GCMetrics`
// 字段为 pub(crate)/crate 内可见，且本用例不走公开脚本面（无 conformance
// 可达路径需要），随 getheaptrigger.rs 先例置本文件尾块。
#[cfg(all(test, feature = "luai_gcmetrics"))]
mod tests {
  use core::{
    ffi::c_void,
    mem::size_of,
    ptr::{null_mut, write_bytes},
  };

  use crate::{
    functions::{
      l_alloc::l_alloc, lua_c_step::lua_c_step, lua_close::lua_close, lua_newstate::lua_newstate,
    },
    macros::gc_spause::GCSPAUSE,
    records::{gc_cycle_metrics::GCCycleMetrics, lg::LG},
  };

  /// 夹具状态：一块已涂脏的 LG 等大小分配块 + 一次性交割旗标。
  struct DirtyLgFixture {
    buf: *mut u8,
    nsize: usize,
    handed: bool,
  }

  /// 首次「null→size_of::<LG>()」纯分配请求交割脏块，其余请求原样转发 `l_alloc`。
  ///
  /// # Safety
  /// `ud` 必须是存活的 `DirtyLgFixture` 盒子指针（lua_Alloc 契约允许宿主任意
  /// 透传，本测试独占构造）；块生命周期随测试内的 VM 交接。
  unsafe extern "C-unwind" fn handout_alloc(
    ud: *mut c_void,
    ptr: *mut u8,
    osize: usize,
    nsize: usize,
  ) -> *mut u8 {
    // SAFETY: `ud` 由本测试夹具保证指向存活的 `DirtyLgFixture`
    let fixture = unsafe { &mut *(ud.cast::<DirtyLgFixture>()) };
    if ptr.is_null() && osize == 0 && nsize == fixture.nsize && !fixture.handed {
      fixture.handed = true;
      return fixture.buf;
    }
    // SAFETY: 其余请求按 lua_Alloc 契约原样转发 `l_alloc`（其契约接受同一 ud/块谱系）
    unsafe { l_alloc(ud, ptr, osize, nsize) }
  }

  /// 造一块涂满 0xFF 的等大小分配块并包成夹具（buf 所有权随交割转给 VM，
  /// 测试尾由 `lua_close` 经同一分配器归还）。
  fn dirty_fixture() -> Box<DirtyLgFixture> {
    let nsize = size_of::<LG>();
    // SAFETY: `l_alloc` 契约——null ptr + osize 0 + nsize 为纯分配请求
    let buf = unsafe { l_alloc(null_mut(), null_mut(), 0, nsize) };
    assert!(!buf.is_null(), "夹具块分配失败");
    // SAFETY: `buf` 为刚分配的 `nsize` 字节可读块，全字节初涂合法
    unsafe { write_bytes(buf, 0xFF, nsize) };
    Box::new(DirtyLgFixture {
      buf,
      nsize,
      handed: false,
    })
  }

  /// 边界臂：脏块开态后 `gcmetrics` 必须逐字段全零（对齐 cpp lstate.cpp:256
  /// `g->gcmetrics = GCMetrics();`）。修复前本臂读到 0xFF 基脏值。
  #[test]
  fn fresh_state_zeroes_gcmetrics_on_dirty_lg_block() {
    let mut fixture = dirty_fixture();
    let ud: *mut c_void = (&raw mut *fixture).cast::<c_void>();
    // SAFETY: `handout_alloc` 满足 lua_Alloc 契约（见其 Safety 注），`ud` 存活至本测试尾
    let l = unsafe { lua_newstate(Some(handout_alloc), ud) };
    assert!(!l.is_null(), "带毒夹具下状态创建必须成功");
    // 夹具必须真被交割——否则操作数域未被污染，本用例失去钉子意义。
    assert!(fixture.handed, "LG 分配请求未经夹具交割");
    // SAFETY: `l` 为刚创建的存活状态，`global` 指向其 LG 内同块 `global_State`
    let gm = unsafe { &(*(*l).global).gcmetrics };
    assert_eq!(gm.completedcycles, 0);
    assert!(gm.stepexplicittimeacc == 0.0 && gm.stepassisttimeacc == 0.0);
    assert_eq!(gm.currcycle, GCCycleMetrics::default());
    assert_eq!(gm.lastcycle, GCCycleMetrics::default());
    // SAFETY: `l` 为本测试独占持有且尚未关闭的主线程状态
    unsafe { lua_close(l) };
  }

  /// 驱动臂：在脏块开态上跑一整轮 assist GC 步进至回到 GCSpause，累加器
  /// （含首发爆点 `assistwork`）必须停留在堆字节值域——修复前 debug 直接
  /// add-with-overflow panic，release 则脏基值越出本值域被本臂捕获。
  #[test]
  fn full_assist_gc_cycle_keeps_work_accumulators_in_heap_domain() {
    let mut fixture = dirty_fixture();
    let ud: *mut c_void = (&raw mut *fixture).cast::<c_void>();
    // SAFETY: 同 `fresh_state_zeroes_gcmetrics_on_dirty_lg_block`
    let l = unsafe { lua_newstate(Some(handout_alloc), ud) };
    assert!(!l.is_null());
    assert!(fixture.handed);
    let mut steps = 0usize;
    loop {
      // SAFETY: `l` 存活；置 debt=0 满足 `lua_c_step` 的 totalbytes≥gc_threshold
      // 前提（对齐 VM 内分配点驱动形态）。
      unsafe {
        let g = (*l).global;
        (*g).gc_threshold = (*g).totalbytes;
        lua_c_step(l, true);
      }
      steps += 1;
      assert!(steps < 1_000_000, "GC 周期未推进，防挂起护栏");
      // SAFETY: `l`/`global` 存活，本行仅读 gcstate
      if unsafe { (*(*l).global).gcstate as i32 } == GCSPAUSE {
        break;
      }
    }
    // SAFETY: `l` 存活，只读取 gcmetrics 快照做值域断言
    let gm = unsafe { &(*(*l).global).gcmetrics };
    assert!(gm.completedcycles >= 1);
    // 周期归档在 lastcycle：全 assist 驱动下 assistwork 覆盖 mark/sweep/atomic
    // 全部工作量，故 assistwork>0 且 ≥ markwork+sweepwork。
    assert!(gm.lastcycle.assistwork > 0);
    assert!(gm.lastcycle.assistwork >= gm.lastcycle.markwork + gm.lastcycle.sweepwork);
    // 值域门：单 VM 堆 ≪ 2^48 字节；脏基值（0xFF 谱系 ≈ 2^64）必越此界。
    let ceiling = 1usize << 48;
    assert!(gm.lastcycle.markwork < ceiling);
    assert!(gm.lastcycle.sweepwork < ceiling);
    assert!(gm.lastcycle.assistwork < ceiling);
    assert_eq!(gm.lastcycle.explicitwork, 0); // 全程 assist，显式面零累加
    assert_eq!(gm.currcycle, GCCycleMetrics::default()); // 新周期已复位
    assert!(gm.stepassisttimeacc.is_finite());
    // SAFETY: `l` 为本测试独占持有且尚未关闭的主线程状态
    unsafe { lua_close(l) };
  }
}
