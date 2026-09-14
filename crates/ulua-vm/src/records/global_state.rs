use core::ffi::{c_int, c_void};

#[cfg(feature = "luai_gcmetrics")]
use crate::records::gc_metrics::GCMetrics;
use crate::{
  records::{
    gc_object::GcObject, gc_stats::GCStats, lua_callbacks::LuaCallbacks,
    lua_execution_callback_storage::LuaExecutionCallbackStorage,
    lua_execution_callbacks::LuaExecutionCallbacks, lua_jmpbuf::lua_jmpbuf, lua_page::lua_Page,
    lua_state::LuaState, lua_t_value::TValue, lua_table::LuaTable,
    lua_udata_direct_access_data::LuaUdataDirectAccessData, stringtable::Stringtable,
    t_string::tstring, up_val::UpVal,
  },
  type_aliases::lua_alloc::LuaAlloc,
};

#[repr(C)]
#[derive(Debug)]
pub struct global_State {
  pub strt: Stringtable,
  pub frealloc: LuaAlloc,
  pub ud: *mut c_void,
  pub currentwhite: u8,
  pub gcstate: u8,
  pub gray: *mut GcObject,
  pub grayagain: *mut GcObject,
  pub weak: *mut GcObject,
  pub gc_threshold: usize,
  pub totalbytes: usize,
  pub gcgoal: c_int,
  pub gcstepmul: c_int,
  pub gcstepsize: c_int,
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
  pub registryfree: c_int,
  pub errorjmp: *mut lua_jmpbuf,
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
