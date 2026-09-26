use core::{
  ffi::c_void,
  fmt::{Debug, Formatter, Result},
  ptr, slice,
};

use crate::{
  checkliveness,
  enums::lua_type::LuaType,
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  records::{
    closure::Closure, gc_object::GCObject, global_state::global_State, lua_state::LuaState,
    lua_table::LuaTable, luau_buffer::LuaBuffer, t_string::TString, udata::Udata,
  },
  tagged_slot_accessors,
  type_aliases::value::Value,
};
#[derive(Clone, Copy)]
#[repr(C)]
#[derive(Default)]
pub struct lua_TValue {
  pub value: Value,
  pub extra: [i32; 1],
  pub tt: i32,
}

pub type TValue = lua_TValue;

/// 向量槽 `f32` 视图的 lane 数（编译期自 `LUA_VECTOR_SIZE` 求值；cpp `lobject.h:141-150`
/// `setvvalue` 的 3/4-lane 两形态）：3-lane 构建写满 `value`+`extra` 12 字节，
/// 4-lane 构建视图多覆盖的本槽尾字节随后被 `tt` 置写覆盖，与旧宏裸指针写逐位一致。
const VVALUE_LANES: usize = if LUA_VECTOR_SIZE == 4 { 4 } else { 3 };

impl Debug for lua_TValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("lua_TValue")
      .field("extra", &self.extra)
      .field("tt", &self.tt)
      .finish_non_exhaustive()
  }
}

/// 八个 cpp GC setter 公开壳（`setsvalue`/`sethvalue`/`setclvalue`/`setupvalue`/
/// `setthvalue`/`setbufvalue`/`setclassvalue`/`setobjectvalue`，lobject.h:168-249）
/// 中除锚点方法 `set_svalue` 外七件套的单源工厂：签名、`# Safety` 文档与转发
/// `set_gc_tagged` 共享核心的函数体逐行同形，唯余方法名、`LuaType` tag 常量与
/// 首行 doc——后者作为属性经 `#[doc = $doc]` 原样保留（含 cpp 出处锚点）。
/// `set_svalue` 首行另携带 `as *mut GCObject` 抹除与 `g` 实参纯度的补充论证，保留手写并
/// 继续充当各壳 `# Safety` 的引用锚点。
macro_rules! gc_value_setter {
  ($name:ident, $doc:expr, $tag:path) => {
    #[doc = $doc]
    ///
    /// # Safety
    /// 同 [`Self::set_svalue`]（含「屏障留调用点」红线与 `g` 有效性前提）。
    #[inline]
    pub unsafe fn $name(&mut self, gc: *mut GCObject, g: *mut global_State) {
      // Safety: 同 [`Self::set_svalue`]，转发共享核心 `set_gc_tagged`
      unsafe { self.set_gc_tagged(gc, g, $tag) }
    }
  };
}

impl lua_TValue {
  /// Tag accessor mirroring `TKey::tt()` so the C++ duck-typed tag macros
  /// (`ttype!`, `setttype!`, `iscollectable!`) work on values AND keys.
  #[inline]
  pub fn tt(&self) -> i32 {
    self.tt
  }

  #[inline]
  pub fn set_tt(&mut self, tt: i32) {
    self.tt = tt;
  }

  /// 判断是否为 nil 类型（cpp `ttisnil`，lobject.h:103）。
  #[inline]
  pub fn is_nil(&self) -> bool {
    self.tt == LuaType::Nil as i32
  }

  /// 判断是否为 boolean 类型（cpp `ttisboolean`，lobject.h:104）。
  #[inline]
  pub fn is_boolean(&self) -> bool {
    self.tt == LuaType::Boolean as i32
  }

  /// 判断是否为 number 类型（cpp `ttisnumber`，lobject.h:105）。
  #[inline]
  pub fn is_number(&self) -> bool {
    self.tt == LuaType::Number as i32
  }

  /// 判断是否为 integer 类型（本 fork 扩展类型，`ttisinteger`）。
  #[inline]
  pub fn is_integer(&self) -> bool {
    self.tt == LuaType::Integer as i32
  }

  /// 判断是否为 string 类型（cpp `ttisstring`，lobject.h:106）。
  #[inline]
  pub fn is_string(&self) -> bool {
    self.tt == LuaType::String as i32
  }

  /// 判断是否为 table 类型（cpp `ttistable`，lobject.h:107）。
  #[inline]
  pub fn is_table(&self) -> bool {
    self.tt == LuaType::Table as i32
  }

  /// 判断是否为 function 类型（cpp `ttisfunction`，lobject.h:108）。
  #[inline]
  pub fn is_function(&self) -> bool {
    self.tt == LuaType::Function as i32
  }

  /// 判断是否为 thread 类型（cpp `ttisthread`，lobject.h:109）。
  #[inline]
  pub fn is_thread(&self) -> bool {
    self.tt == LuaType::Thread as i32
  }

  /// 判断是否为 userdata 类型（cpp `ttisuserdata`，lobject.h:110）。
  #[inline]
  pub fn is_userdata(&self) -> bool {
    self.tt == LuaType::UserData as i32
  }

  /// 判断是否为 lightuserdata 类型（cpp `ttislightuserdata`，lobject.h:111）。
  #[inline]
  pub fn is_lightuserdata(&self) -> bool {
    self.tt == LuaType::LightUserData as i32
  }

  /// 判断是否为 vector 类型（cpp `ttisvector`，lobject.h:112）。
  #[inline]
  pub fn is_vector(&self) -> bool {
    self.tt == LuaType::Vector as i32
  }

  /// 判断是否为 buffer 类型（cpp `ttisbuffer`，lobject.h:113）。
  #[inline]
  pub fn is_buffer(&self) -> bool {
    self.tt == LuaType::Buffer as i32
  }

  /// 判断是否为 class 类型（本 fork 扩展类型，`ttisclass`）。
  #[inline]
  pub fn is_class(&self) -> bool {
    self.tt == LuaType::Class as i32
  }

  /// 判断是否为 object 类型（本 fork 扩展类型，`ttisobject`）。
  #[inline]
  pub fn is_object(&self) -> bool {
    self.tt == LuaType::Object as i32
  }

  /// 判断是否为 upval 类型（`ttisupval`）。
  #[inline]
  pub fn is_upval(&self) -> bool {
    self.tt == LuaType::Upval as i32
  }

  // GC 引用三件套与标量读取族：与 `TKey` 逐字同形的双镜像收进单源宏
  // `tagged_slot_accessors!`（宏体见 `macros/tagged_slot_accessors.rs`），签名、
  // debug_assert、body 与手写版逐点等价；`as_vector`/`as_vector_ref`（跨
  // value+extra 裸写形状）与 `set_*` 族不在本宏范围，继续手写于下方。
  tagged_slot_accessors! {
    gc as_table, as_table_mut, as_table_ptr, LuaTable, *mut LuaTable, h, is_table,
      "借用为只读 Table 引用（cpp `hvalue`，lobject.h:93）。",
      "借用为可变 Table 引用。",
      "获取 Table 的裸指针（供需要 `*mut LuaTable` 的 C-ABI/内部例程调用）。";
    gc as_closure, as_closure_mut, as_closure_ptr, Closure, *mut Closure, cl, is_function,
      "借用为只读 Closure 引用（cpp `clvalue`，lobject.h:94）。",
      "借用为可变 Closure 引用。",
      "获取 Closure 的裸指针。";
    gc_ro as_string, as_string_ptr, TString, *mut TString, ts, is_string,
      "借用为只读 TString 引用（cpp `tsvalue`，lobject.h:91）。",
      "获取 TString 的裸指针。";
    gc_cptr as_userdata, as_userdata_mut, as_userdata_ptr, Udata, *const Udata, u,
      is_userdata,
      "借用为只读 Udata 引用（cpp `uvalue`，lobject.h:92）。",
      "借用为可变 Udata 引用。",
      "获取 Udata 的原始指针（`*const Udata`）。";
    gc as_thread, as_thread_mut, as_thread_ptr, LuaState, *mut LuaState, th, is_thread,
      "借用为只读 LuaState（Thread）引用（cpp `thvalue`，lobject.h:95）。",
      "借用为可变 LuaState 引用。",
      "获取 LuaState 的原始指针（`*mut LuaState`）。";
    gc as_buffer, as_buffer_mut, as_buffer_ptr, LuaBuffer, *mut LuaBuffer, buf, is_buffer,
      "借用为只读 LuaBuffer 引用（cpp `bufvalue`，lobject.h:96）。",
      "借用为可变 LuaBuffer 引用。",
      "获取 LuaBuffer 的原始指针（`*mut LuaBuffer`）。";
    scalar_bool as_boolean, b, is_boolean,
      "读取布尔值（cpp `bvalue`，lobject.h:160）。";
    scalar as_boolean_raw, i32, b, is_boolean,
      "读取布尔原始整型（cpp `bvalue` 原样 i32，true 必为 1）。";
    scalar as_number, f64, n, is_number,
      "读取浮点数值（cpp `nvalue`，lobject.h:116）。";
    scalar as_integer, i64, l, is_integer,
      "读取 64 位整型值（`lvalue`）。";
  }

  /// 读取 3 分量向量（cpp `vvalue`，lobject.h:134；`value + extra` 连续 12 字节）。
  #[inline]
  pub fn as_vector(&self) -> [f32; 3] {
    *self.as_vector_ref()
  }

  /// 借用为 3 分量向量切片引用。
  #[inline]
  pub fn as_vector_ref(&self) -> &[f32; 3] {
    debug_assert!(self.is_vector());
    unsafe { &*(ptr::from_ref(self).cast::<[f32; 3]>()) }
  }

  /// 置 nil tag（cpp `setnilvalue`，lobject.h:115）：仅写 `tt = LUA_TNIL`，
  /// 不清 payload——与 cpp 逐字一致，payload 残值由 tag 判定屏蔽。
  ///
  /// 本方法是 `setnilvalue!` 宏的唯一收口实现（宏体仅做 `(*$obj).set_nil()`
  /// 转发）；上方的 `set_tt` 则是 `setttype!` 宏的收口实现（其唯一消费点
  /// `removeentry` 作用于 `TKey`，故不另设 TValue 专属方法）。
  ///
  /// 方法体仅安全字段写；原 `# Safety` 前提（裸指针非空/对齐/已初始化/noalias
  /// 独占、槽在写入有效期内不被栈扩容 `lua_d_reallocstack` 移动）随宏收口下沉至
  /// 调用点对 `(*$obj)` 裸指针解引用处保证，语义不变。
  #[inline]
  pub fn set_nil(&mut self) {
    self.tt = LuaType::Nil as i32;
  }

  /// 写布尔载荷并置 tag（cpp `setbvalue`，lobject.h:161）：先 `value.b` 后
  /// `tt = LUA_TBOOLEAN`，写入次序与旧宏体逐位一致。`b` 已是 `($b) as i32`
  /// 收敛后的 i32（宏体保留该 cast 以兼容 bool/整型实参，cpp 同形：true 必为 1）。
  ///
  /// 方法体仅安全字段写（union 域写入本身安全）；`&mut` 独占来源的前提
  /// （裸指针非空/对齐/已初始化/noalias 独占，寿命不跨栈扩容）随宏收口
  /// 下沉至调用点解引用处保证，同 [`Self::set_nil`]。
  #[inline]
  pub fn set_bvalue(&mut self, b: i32) {
    self.value.b = b;
    self.tt = LuaType::Boolean as i32;
  }

  /// 写数值载荷并置 tag（cpp `setnvalue`，lobject.h:117）：先 `value.n` 后
  /// `tt = LUA_TNUMBER`，写入次序与旧宏体逐位一致。
  ///
  /// 方法体仅安全字段写（union 域写入本身安全）；`&mut` 独占来源的前提
  /// （裸指针非空/对齐/已初始化/noalias 独占，寿命不跨栈扩容）随宏收口
  /// 下沉至调用点解引用处保证，同 [`Self::set_nil`]。
  #[inline]
  pub fn set_nvalue(&mut self, n: f64) {
    self.value.n = n;
    self.tt = LuaType::Number as i32;
  }

  /// 写整数载荷并置 tag（cpp `setlvalue`，lobject.h:124）：先 `value.l` 后
  /// `tt = LUA_TINTEGER`，写入次序与旧宏体逐位一致。
  ///
  /// 方法体仅安全字段写（union 域写入本身安全）；`&mut` 独占来源的前提
  /// （裸指针非空/对齐/已初始化/noalias 独占，寿命不跨栈扩容）随宏收口
  /// 下沉至调用点解引用处保证，同 [`Self::set_nil`]。
  #[inline]
  pub fn set_lvalue(&mut self, l: i64) {
    self.value.l = l;
    self.tt = LuaType::Integer as i32;
  }

  /// 写 lightuserdata 载荷与 user-tag（cpp `setpvalue`，lobject.h:153）：先
  /// `value.p`、再 `extra[0] = tag`、最后经 `set_tt` 置 `LUA_TLIGHTUSERDATA`，
  /// 写入次序与旧宏体逐位一致。`extra[0]` 与向量 lane2 共享字节（layout 冻结，
  /// 见结构体处注释），lightuserdata/vector 两 tag 互斥故同槽复用无冲突。
  ///
  /// 方法体仅安全字段写（union 域写入本身安全，且不解引用 `p`）；`&mut` 独占
  /// 来源的前提（裸指针非空/对齐/已初始化/noalias 独占，寿命不跨栈扩容）随宏
  /// 收口下沉至调用点解引用处保证，同 [`Self::set_nil`]。
  #[inline]
  pub fn set_pvalue(&mut self, p: *mut c_void, tag: i32) {
    self.value.p = p;
    self.extra[0] = tag;
    self.set_tt(LuaType::LightUserData as i32);
  }

  /// 向量四分写（cpp `setvvalue` inline-float 分支，lobject.h:141-150）：把本槽
  /// 起始的连续字节按 `f32` lane 依次写 0/1/2，第 3 lane 仅在
  /// `LUA_VECTOR_SIZE == 4` 时写（lane 数 [`VVALUE_LANES`] 为编译期常量，分支与
  /// 越界检查均在编译期折叠/消除），最后置 `tt = LUA_TVECTOR`。旧宏经
  /// `i_o as *mut f32` 直写，本方法经 `slice::from_raw_parts_mut` 从同一地址派生
  /// 的 `f32` 视图写入，逐位等价。`w` 以 `FnOnce` 惰性传入、只在 4-lane 门内被
  /// 调用：3-lane 构建下调用点的第 4 分量实参（形如 `*vb.add(3) + *vc.add(3)`，
  /// provenance 仅 `&[f32; 3]`）一旦求值即越界读、在 Rust 中是 UB（cpp 读 padding
  /// 无害故只 `(void)(w)`）——`macros/setvvalue.rs` 文件头的防御 rationale 原样
  /// 适用于此，不得把 `w` 退化为急切 `f32` 实参。求值次序（lane0/1/2 实参先于各
  /// 自写入、`w()` 仅在门内求值）与旧宏一致；调用点实参均为纯读（读他槽 lane，与
  /// 本槽写入 lane 严格不交叉），把 x/y/z 求值统一提前不改变可观察行为。
  ///
  /// # Safety
  /// `&mut self` 由调用点经裸指针解引用取得：非空、按 `TValue` 对齐、指向已初始化
  /// 槽、noalias 独占、槽寿命不跨栈扩容；写入跨 `value`+`extra` 字节段，与 `w()` 的
  /// 4-lane 越界防御共同受 `LUA_VECTOR_SIZE` 编译期常量约束。
  #[inline]
  pub unsafe fn set_vvalue<W: FnOnce() -> f32>(&mut self, x: f32, y: f32, z: f32, w: W) {
    // Safety: `lanes` 由 &mut self 独占派生（cast 至本槽起始地址的 f32 视图），
    // lane0..lane2 落在 value+extra 字节段内；4-lane 门内 lane3 亦在 TValue 尾
    // 字节内；随后经 self 写 tt 后 raw 视图不再使用（Stacked Borrows 顺序合规）
    unsafe {
      let lanes = slice::from_raw_parts_mut(ptr::from_mut(self).cast::<f32>(), VVALUE_LANES);
      lanes[0] = x;
      lanes[1] = y;
      lanes[2] = z;
      if VVALUE_LANES == 4 {
        lanes[3] = w();
      }
      self.tt = LuaType::Vector as i32;
    }
  }

  /// 八个 cpp GC setter（`setsvalue`/`sethvalue`/`setclvalue`/`setupvalue`/
  /// `setthvalue`/`setbufvalue`/`setclassvalue`/`setobjectvalue`，lobject.h
  /// :168-249）的共享核心：方法体逐字同形——先写 `value.gc`、再置 `tt`、末尾
  /// `checkliveness!(g, i_o)`（debug-only 存活断言）——唯一差异是 `LuaType`
  /// tag，各公开方法只携带自己的 tag 转发到这里，安全论证单点收敛。
  ///
  /// # Safety
  /// 同 [`Self::set_svalue`]（裸指针非空/对齐/已初始化/noalias 独占，寿命不跨栈
  /// 扩容）；`g` 须为 `(*L).global` 所指的有效 `global_State`（仅 debug 断言路径
  /// 解引用）。**gc 指针写入方仍须自行负责写屏障**：cpp 中 SETOBJ/SET*SVALUE 与
  /// `luaC_barriert`/`barrierfast` 是分步的（lvmexecute.cpp:442/711/723/815/870、
  /// ltable.cpp:975/1026/1050），本方法绝不触发屏障，调用点屏障时序一律不动。
  #[inline]
  unsafe fn set_gc_tagged(&mut self, gc: *mut GCObject, g: *mut global_State, tt: LuaType) {
    self.value.gc = gc;
    self.tt = tt as i32;
    // Safety: `i_o` 经 `ptr::from_mut` 从 `&mut self` 独占权源出、与本槽同址
    // （与旧宏体传入 checkliveness 的 `i_o` 同值），仅做只读存活断言
    unsafe {
      let i_o = ptr::from_mut(self);
      checkliveness!(g, i_o);
    }
  }

  /// 写 GC 载荷并置 string tag（cpp `setsvalue`，lobject.h:168）：写入次序与旧
  /// 宏体逐位一致。`x as *mut GCObject` 的类型抹除保留在宏壳（继续兼容
  /// 各 GC 指针实参类型），方法只收已擦除的 `gc`；`g` 即旧宏体 checkliveness 的
  /// `(*L).global` 实参（纯指针字段加载，提前于载荷写入求值不改变可观察行为——
  /// 与 [`Self::set_vvalue`] 的实参纯度论证同形）。
  ///
  /// # Safety
  /// 同共享核心 `set_gc_tagged`（含「屏障留调用点」红线与 `g` 有效性前提）。
  #[inline]
  pub unsafe fn set_svalue(&mut self, gc: *mut GCObject, g: *mut global_State) {
    // Safety: 契约见共享核心 `set_gc_tagged`
    unsafe { self.set_gc_tagged(gc, g, LuaType::String) }
  }

  gc_value_setter!(
    set_hvalue,
    "写 GC 载荷并置 table tag（cpp `sethvalue`，lobject.h:208）。",
    LuaType::Table
  );

  gc_value_setter!(
    set_clvalue,
    "写 GC 载荷并置 function tag（cpp `setclvalue`，lobject.h:200）。",
    LuaType::Function
  );

  gc_value_setter!(
    set_upvalue,
    "写 GC 载荷并置 upval tag（cpp `setupvalue`，lobject.h:224）。",
    LuaType::Upval
  );

  gc_value_setter!(
    set_thvalue,
    "写 GC 载荷并置 thread tag（cpp `setthvalue`，lobject.h:184；旧宏体的 `as i32` 与 `as i32` 同型，逐位一致）。",
    LuaType::Thread
  );

  gc_value_setter!(
    set_bufvalue,
    "写 GC 载荷并置 buffer tag（cpp `setbufvalue`，lobject.h:192）。",
    LuaType::Buffer
  );

  gc_value_setter!(
    set_classvalue,
    "写 GC 载荷并置 class tag（cpp `setclassvalue`，lobject.h:240）。",
    LuaType::Class
  );

  gc_value_setter!(
    set_objectvalue,
    "写 GC 载荷并置 object tag（cpp `setobjectvalue`，lobject.h:249）。",
    LuaType::Object
  );

  /// 整槽复制（cpp `setobj`，lobject.h:232）：把 `other` 所指 TValue 全量拷入本
  /// 槽后做 `checkliveness!(g, i_o)`（debug-only 存活断言），与旧宏体
  /// `*o1 = *o2; checkliveness(L->global, o1)` 逐位一致。写用 `ptr::copy`
  /// （memmove）而非 `*self = *other_ref`：cpp 允许 `o1`/`o2` 同槽自拷贝（如
  /// 赋值收窄路径）与栈区间相邻重叠，memmove 语义覆盖 C 裸指针 load-store 的全
  /// 部合法形态，而共享引用别名 `&mut self` 在 Stacked Borrows 下即为 UB。
  /// `TValue` 为 repr(C) 无 padding 的 Copy POD（value+extra+tt 共 16 字节），
  /// 整字节拷贝与逐字段拷贝等价。`g` 即旧宏体 checkliveness 的 `(*L).global`
  /// 实参（纯指针字段加载，提前于拷贝求值不改变可观察行为，同
  /// [`Self::set_svalue`] 的论证）。
  ///
  /// # Safety
  /// 本槽侧 `&mut self` 由调用点经裸指针解引用取得：非空、按 `TValue` 对齐、指向
  /// 已初始化槽、noalias 独占、寿命不跨栈扩容；`other` 须为指向已初始化 TValue 的有效指针（非空、按 `TValue`
  /// 对齐，允许与本槽同址——自拷贝合法）。**gc 指针写入方仍须自行负责屏障**：
  /// cpp 中 SETOBJ 与 `luaC_barriert`/`barriert`/`back` 分步（lvmexecute.cpp:
  /// 442/711/723/815/870、ltable.cpp:975/1026/1050），本方法绝不触发屏障，调用
  /// 点屏障时序（lua_v_settable.rs:56-60、lua_rawset.rs、lua_rawsetfield.rs、
  /// luau_execute.rs:465/741/752/839/898 等）一律不动。
  #[inline]
  pub unsafe fn set_obj(&mut self, other: *const TValue, g: *mut global_State) {
    // Safety: `i_o` 经 `ptr::from_mut` 从 `&mut self` 独占权源出、与本槽同址；
    // `ptr::copy` 允许与 `other` 重叠/同址（memmove 语义，等价 cpp 裸指针整槽
    // 赋值），随后仅做只读存活断言
    unsafe {
      let i_o = ptr::from_mut(self);
      ptr::copy(other, i_o, 1);
      checkliveness!(g, i_o);
    }
  }
}
