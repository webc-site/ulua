use core::ffi::c_void;

use crate::{
  records::{
    call_info::CallInfo, g_cheader::GCheader, gc_object::GcObject, global_state::global_State,
    lua_table::LuaTable, t_string::tstring, up_val::UpVal,
  },
  type_aliases::stk_id::StkId,
};

mod access;
mod error;
mod stack;
mod table;
mod thread;

#[repr(C)]
#[derive(Debug, Default)]
pub struct lua_State {
  pub hdr: GCheader,
  pub status: u8,
  pub activememcat: u8,
  pub isactive: bool,
  pub singlestep: bool,
  pub top: StkId,
  pub base: StkId,
  pub global: *mut global_State,
  pub ci: *mut CallInfo,
  pub stack_last: StkId,
  pub stack: StkId,
  pub end_ci: *mut CallInfo,
  pub base_ci: *mut CallInfo,
  pub stacksize: i32,
  pub size_ci: i32,
  pub n_ccalls: u16,
  pub base_ccalls: u16,
  pub cachedslot: i32,
  pub gt: *mut LuaTable,
  pub openupval: *mut UpVal,
  pub gclist: *mut GcObject,
  pub namecall: *mut tstring,
  pub userdata: *mut c_void,
}

pub type LuaState = lua_State;

impl LuaState {
  #[inline(always)]
  pub fn as_mut_ptr(&mut self) -> *mut Self {
    self as *mut Self
  }

  #[inline(always)]
  pub fn as_ptr(&self) -> *const Self {
    self as *const Self
  }

  /// 只读转发辅助：C-ABI 镜像函数族（`lua_type` 等 getter）的签名统一收
  /// `*mut Self`，即使被调方仅读取。本方法把 `*const → *mut` 的无写 provenance
  /// 抹除集中到这一处并附契约，替代散落的 `self.as_ptr() as *mut Self` 裸 cast。
  ///
  /// 使用契约：返回值仅可传给按其 `# Safety` 文档**不写穿**该指针、不将其逸出
  /// 保存的被调方；违约即别名 UB。
  #[inline(always)]
  pub(crate) fn read_ptr(&self) -> *mut Self {
    self as *const Self as *mut Self
  }

  // ---- r15-v1：global_State 入口句柄门面（唯一新造位）----
  //
  // 形制承 compiler `nn_alias::alias_ref` / 本 crate `stack.rs::top_slot` 判例：
  // 单 unsafe 体 + 契约上收，状态开场的 `global` 裸字段散点解引用自此有收口
  // 入口。票面二选形制取「gs_ref 只读件」：本波授权迁移面（gcmetrics 族 4
  // 文件）内不存在 &mut 消费位（三 helper 的场域句柄皆为入参形态，定性保留），
  // gs_mut 无消费即不造——造则超授权新造且触 dead_code 门禁。

  /// 入口句柄：`self.global` 裸字段的只读解引用收口，返回挂靠 `&self` 生命期
  /// 的 `&global_State` 视图。返回值刻意拉长寿命、与本族原散点裸解引用同构
  /// （借用模型等价 cpp `L->global` 自由读数）；代价以契约偿付——消费点仍须
  /// 语句内即取即用，禁跨栈/GC 重入持有、禁缓存（会令借用跨重入的点位不迁，
  /// 保留原形）。
  ///
  /// # Safety（契约由本方法调用方按文档保证）
  /// 1. `self.global` 在状态存活期恒定非空、对齐并指向存活 `global_State`
  ///    （本文件 `global` 字段结构不变量，cpp `lstate.h` 同款）；
  /// 2. 所得只读视图存活期内，经其它别名（含裸指针场域）对 global_State 的
  ///    写入不发生——视图窗内不得穿插重入调用（lua_* / GC step / 分配器回调）
  ///    或场域写点；
  /// 3. null 哨兵不进入本门面：可空处调用点先行判空或以 `Option` 表达。
  #[inline(always)]
  pub(crate) fn gs_ref(&self) -> &global_State {
    // SAFETY: 契约由调用点逐条承担（见上）；本方法为全仓本族 `global` 裸字段
    // 只读解引用的唯一收口体，与原散点 `&*(*l).global` 同形，零行为改动。
    unsafe { &*self.global }
  }
}
