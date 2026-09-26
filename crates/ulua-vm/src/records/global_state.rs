use core::ffi::c_void;

#[cfg(feature = "luai_gcmetrics")]
use crate::records::gc_metrics::GCMetrics;
use crate::{
  records::{
    gc_object::GcObject, gc_stats::GCStats, lua_callbacks::LuaCallbacks,
    lua_execution_callback_storage::LuaExecutionCallbackStorage,
    lua_execution_callbacks::LuaExecutionCallbacks, lua_page::lua_Page, lua_state::LuaState,
    lua_t_value::TValue, lua_table::LuaTable,
    lua_udata_direct_access_data::LuaUdataDirectAccessData, stringtable::Stringtable,
    t_string::tstring, up_val::UpVal,
  },
  type_aliases::lua_alloc::LuaAlloc,
};

/// 全局 VM 状态。`#[repr(C)]` 与字段顺序是 code-gen JIT 的 ABI 契约
/// （`offset_of!(global_State, totalbytes/gc_threshold/cb/ecbdata/tmname)` 直接进机器码），不得重排。
///
/// 裸指针字段的所有权模型（§11 的 arena/`GcRef(u32)` 化是后续阶段，本阶段仅明确契约）：
/// - `strt`：串表访问已收拢为 `records/stringtable.rs` 的类型化句柄（`BucketIdx` +
///   切片视图 + `interned/link_front/unlink/detach_buckets` 方法），仅桶数组存储
///   本身仍是 frealloc arena 裸内存（受 `lua_newstate` 直建与 `lua_s_resize` 换阵契约约束）；
/// - `gray`/`grayagain`/`weak`：侵入式单链表的表头，指向 page 分配器持有的 GCObject，非拥有；
/// - `freepages`/`allpages` 等页链与 `mainthread`/`mt`/`ttname`/`tmname`/`udatamt` 等表：
///   指向经 `frealloc` 分配的存活内存，由 page/串表生命周期拥有，此处只作寻址句柄；
/// - `ecb`：C 形态回调表，见 [`LuaExecutionCallbacks`]。
#[repr(C)]
#[derive(Debug)]
pub struct global_State {
  pub strt: Stringtable,
  pub frealloc: LuaAlloc,
  /// `frealloc` 的用户数据（透传给分配器闭包），非空由宿主保证。
  pub ud: *mut c_void,
  pub currentwhite: u8,
  pub gcstate: u8,
  pub gray: *mut GcObject,
  pub grayagain: *mut GcObject,
  pub weak: *mut GcObject,
  pub gc_threshold: usize,
  pub totalbytes: usize,
  pub gcgoal: i32,
  pub gcstepmul: i32,
  pub gcstepsize: i32,
  pub freepages: [*mut lua_Page; 40],    // LUA_SIZECLASSES
  pub freegcopages: [*mut lua_Page; 40], // LUA_SIZECLASSES
  pub allpages: *mut lua_Page,
  pub allgcopages: *mut lua_Page,
  pub sweepgcopage: *mut lua_Page,
  pub mainthread: *mut LuaState,
  pub uvhead: UpVal,
  pub mt: [*mut LuaTable; 14], // LUA_T_COUNT = LUA_TDEADKEY
  pub ttname: [*mut tstring; 16],
  pub tmname: [*mut tstring; 21],
  pub pseudotemp: TValue,
  pub registry: TValue,
  pub registryfree: i32,
  pub rngstate: u64,
  pub ptrenckey: [u64; 4],
  pub cb: LuaCallbacks,
  pub ecb: LuaExecutionCallbacks,
  pub ecbdata: LuaExecutionCallbackStorage, // LUA_EXECUTION_CALLBACK_STORAGE
  pub udatadirect: [LuaUdataDirectAccessData; 130], // UTAG_INTERNAL_LIMIT
  pub memcatbytes: [usize; 256],
  pub udatagc: [Option<unsafe extern "C-unwind" fn(*mut LuaState, *mut c_void)>; 128],
  pub udatamt: [*mut LuaTable; 128],
  pub lightuserdataname: [*mut tstring; 128],
  pub udatadirectfields: [*mut LuaTable; 130], // UTAG_INTERNAL_LIMIT
  pub gcstats: GCStats,
  pub lastprotoid: u32,
  #[cfg(feature = "luai_gcmetrics")]
  pub gcmetrics: GCMetrics,
}
