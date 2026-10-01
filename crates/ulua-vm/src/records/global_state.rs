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

// —— 数组字段长度具名常量（review.md §6：消灭魔法数字）。数值即 repr(C)/
// JIT offset_of 冻结布局的一部分，严禁改动；语义来源为 cpp `lstate.h`
// `global_State` 注释与 cpp 同名宏。

/// cpp `LUA_SIZECLASSES`（luaconf.h:111）：页分配器 size class 数，
/// `freepages`/`freegcopages` 按类各挂一条空闲页链（lstate.h:211-212）；
/// 同值常量另见 `size_class_config::K_SIZE_CLASSES`。
const SIZE_CLASSES: usize = 40;
/// cpp `LUA_T_COUNT = LUA_TDEADKEY`（lua.h:113）：TValue 类型 tag 计数，
/// `mt` 按基本类型各存一个元表。
const BASIC_TYPE_TAGS: usize = 14;
/// `ttname` 槽数：消费点按 `ttype!`/键 tag 索引，覆盖至内部 Proto tag
/// （0..=LUA_TPROTO=15）；cpp 同位是 `LUA_T_COUNT`（14），本移植布局取 16，
/// 多出的 DEADKEY/PROTO 槽无人 intern、恒为 null。
const TTNAME_TAGS: usize = 16;
/// cpp `TM_N`（ltm.h:39）：元方法名计数，索引即 `TMS` 枚举序号
/// （`lua_t_init.rs` 编译期断言 EVENTNAMES 与之同步）。
const TM_NAMES: usize = 21;
/// cpp `UTAG_INTERNAL_LIMIT`（ludata.h:14）= `LUA_UTAG_LIMIT + 2`：含内建
/// UTAG_IDTOR/UTAG_PROXY 的 userdata tag 空间，`udatadirect`（lstate.h:240）
/// 与 `udatadirectfields`（lstate.h:256）按 tag 各一槽。
const UDATA_INTERNAL_TAGS: usize = 130;
/// cpp `LUA_MEMORY_CATEGORIES`（luaconf.h:116）：内存分类计数，
/// `memcatbytes` 按类累计在用字节。
const MEMORY_CATEGORIES: usize = 256;
/// cpp `LUA_UTAG_LIMIT`（luaconf.h:101）：宿主注册的 userdata tag 上限，
/// `udatagc`/`udatamt` 按 tag 各一槽（lstate.h:244-246）。
const UDATA_TAGS: usize = 128;
/// cpp `LUA_LUTAG_LIMIT`（luaconf.h:106）：lightuserdata tag 上限，
/// `lightuserdataname` 按 tag 各一名槽（lstate.h:253）。
const LIGHTUSERDATA_TAGS: usize = 128;
/// cpp `lstate.h:237` `uint64_t ptrenckey[4]`：指针混淆密钥槽数；
/// `lua_encodepointer` 按「两段乘数 + 两段加数」乘加后异或。
pub(crate) const PTRENCKEY_LEN: usize = 4;
/// 建态时的 `ptrenckey` 初值（唯一写点在 `lua_newstate`）：key[0]=1 为乘法单位元、
/// 其余 0 为加法单位元；密钥轮换移植前编码退化为恒等透传。
pub(crate) const PTRENCKEY_INIT: [u64; PTRENCKEY_LEN] = [1, 0, 0, 0];

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
  pub freepages: [*mut lua_Page; SIZE_CLASSES],
  pub freegcopages: [*mut lua_Page; SIZE_CLASSES],
  pub allpages: *mut lua_Page,
  pub allgcopages: *mut lua_Page,
  pub sweepgcopage: *mut lua_Page,
  pub mainthread: *mut LuaState,
  pub uvhead: UpVal,
  pub mt: [*mut LuaTable; BASIC_TYPE_TAGS], // LUA_T_COUNT = LUA_TDEADKEY
  pub ttname: [*mut tstring; TTNAME_TAGS],
  pub tmname: [*mut tstring; TM_NAMES],
  pub pseudotemp: TValue,
  pub registry: TValue,
  pub registryfree: i32,
  pub rngstate: u64,
  pub ptrenckey: [u64; PTRENCKEY_LEN],
  pub cb: LuaCallbacks,
  pub ecb: LuaExecutionCallbacks,
  pub ecbdata: LuaExecutionCallbackStorage, // LUA_EXECUTION_CALLBACK_STORAGE
  pub udatadirect: [LuaUdataDirectAccessData; UDATA_INTERNAL_TAGS],
  pub memcatbytes: [usize; MEMORY_CATEGORIES],
  pub udatagc: [Option<unsafe extern "C-unwind" fn(*mut LuaState, *mut c_void)>; UDATA_TAGS],
  pub udatamt: [*mut LuaTable; UDATA_TAGS],
  pub lightuserdataname: [*mut tstring; LIGHTUSERDATA_TAGS],
  pub udatadirectfields: [*mut LuaTable; UDATA_INTERNAL_TAGS],
  pub gcstats: GCStats,
  pub lastprotoid: u32,
  #[cfg(feature = "luai_gcmetrics")]
  pub gcmetrics: GCMetrics,
}
