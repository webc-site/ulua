//! `SubstitutionVtable` 宿主「thunk 组 + 装表」的收口单点。
//!
//! 每个 `Substitution`/`Tarjan` 子类宿主（Anyification/Instantiation/…）都有一组
//! 逐字同构的模块级 thunk：把 `vtable.owner`（`self as *mut Host as *mut ()` 的类型
//! 擦除回指）还原为 `*mut Host`，转发到对应覆写方法；`install_substitution_vtable`
//! 再把这组 fn 指针连同 owner 装进内嵌基类的虚表。原实现该样板在 11 个宿主文件各抄
//! 一份（每份 8-10 个 fn、约 80 行），收口为本宏。
//!
//! 统一安全论证（宿主特有契约见各自覆写方法的文档）：`owner` 只可能由本宏展开的
//! `install_substitution_vtable` 以 `self as *mut $host as *mut ()` 写入同一对象的
//! `vtable.owner`，回转即恒等往返（非空、对齐、类型正确）；thunk 仅在宿主自身
//! `substitute_*` 遍历的同步回调栈内被派发，此刻宿主被调用栈经 `&mut self` 独占驱动，
//! 回调窗口内重建 `&mut $host` 无并存借用，crate 单线程不变量下不构成别名冲突；
//! `ty`/`tp` 为遍历传入的 arena 存活句柄，满足各覆写方法的既有契约。`found_dirty_*`
//! 未被子类覆写，经首字段 `.base`（地址偏移一致）落到基类 `Substitution`。
//!
//! 四个入口臂全部归一到 `@assemble` 单一骨架，差异只由两个选择子表达：
//! - TypePackId 侧三个覆写槽（isDirty/clean/ignoreChildren 的 pack 形态）：
//!   `real`（真实覆写：Anyification/AMG/ApplyTypeFunction/RefineTypeScrubber）、
//!   `false_tp`（ignoreChildren 无覆写、C++ 基类默认 false：
//!   Demoter/Instantiation2/ReplaceGenerics/Replacer/Widen）、
//!   `degen_tp`（Instantiation 退化形态——三槽全为基类默认：isDirty=false、
//!   clean 恒等透传、ignoreChildren=false）；
//! - ignoreChildren thunk 组开关 `ic_present` 与 visit 槽开关 `visit_ic`：
//!   常规宿主为 `thunks`/`alias`（visit 槽与 ignoreChildren 槽共用同一 thunk，
//!   C++ 两个虚槽指向同一覆写）；ClonePublicInterface 为 `none`/`distinct`
//!   （ignoreChildren* 槽位 None 继承基类默认，visit 槽转发到独立的
//!   `ignore_children_visit_*` 覆写）。此前 visit_only 臂手抄整份
//!   `svt_with`/六个共享 thunk/装表体，与 `@assemble` 逐字重复，已并入。
//!
//! §4 处置（伪 vtable 最终评估：**保留**，非 `dyn` 站点）：本宏生成的「thunk 组 +
//! 装表」是 C++ 基类虚函数覆写语义的回填，三种消除方案均已评估且不可行——
//! 1. **泛型单态化**（把宿主类型参数 `H` 贯穿共享遍历）：Tarjan/Substitution 公共
//!    遍历（methods/tarjan.rs + methods/substitution.rs，约 1200 行）会随 11 个宿主
//!    （Anyification/Instantiation/Instantiation2/Demoter/Replacer/Widen/
//!    ReplaceGenerics/ApplyMappedGenerics/ApplyTypeFunction/RefineTypeScrubber/
//!    ClonePublicInterface）膨胀 11 份，正是 §4 允许保留分发的事由；且覆写回调在
//!    遍历仍持有 `&mut Host.base.base` 期间经 owner 重建 `&mut Host`（clean 覆写还会
//!    重入 `substitute_*` 并改写遍历状态，见 ClonePublicInterface 的入口），这是基类
//!    持有器与宿主可变借用的重叠，安全 Rust 借用模型无法表达——泛型化只会把同样的
//!    别名逃逸摊进整个遍历，现状把 unsafe 收进每次宏展开唯一的 `svt_with` 分发器
//!    （§2 最小契约边界，thunk 体无 unsafe）。
//! 2. **`dyn Trait` 对象**：等价于把手写 fn 指针槽换成编译器生成的 vtable，间接调用
//!    开销不变；反而令 `Tarjan` 多出不变量生命周期、失去 `Copy`（表按值装填），且
//!    owner 自指别名依旧要绕过借用检查，严格更差。
//! 3. **`enum_dispatch`/收敛枚举**：分派方向是「基类 → 覆写它的宿主（含基类的上层
//!    类型）」，枚举变体须与遍历对 `Host.base` 的可变借用共存，双 `&mut` 冲突不可
//!    调和；enum_dispatch 的「对实现集静态分发」模型不适用此结构。
//!
//! 另：`@pack_slot false`/`identity` 臂未折叠为 `None` 槽位，因 `found_dirty_*` 的
//! `clean_*` 消费点对 `None` 走 `expect` panic（纯虚调用等价），语义并非「缺省即
//! false/恒等」；折叠需同步改消费点缺省语义，属行为面改动，不在消 dyn 轮内做。
//!
//! §2 边界收口：`owner` 还原原先在每支 thunk 各抄一个 `unsafe { *(owner as *mut $host) }`
//! （每展开 8 个 unsafe 点），现统一收进每次展开唯一的 `svt_with` 分发器——thunk 体
//! 全部为安全一行式，安全论证只写在分发器一处（即上方统一安全论证）。

macro_rules! substitution_vtable {
  (real, $vis:vis $host:ident, ic = $ic_ty:ident) => {
    substitution_vtable!(
      @assemble $vis $host,
      is_dirty_tp: real, clean_tp: real, ic_ty: $ic_ty, ic_tp: real,
      ic_present: thunks, visit_ic: alias);
  };
  (false_tp, $vis:vis $host:ident, ic = $ic_ty:ident) => {
    substitution_vtable!(
      @assemble $vis $host,
      is_dirty_tp: real, clean_tp: real, ic_ty: $ic_ty, ic_tp: false,
      ic_present: thunks, visit_ic: alias);
  };
  (degen_tp, $vis:vis $host:ident, ic = $ic_ty:ident) => {
    substitution_vtable!(
      @assemble $vis $host,
      is_dirty_tp: false, clean_tp: identity, ic_ty: $ic_ty, ic_tp: false,
      ic_present: thunks, visit_ic: alias);
  };
  // ClonePublicInterface 形态：ignoreChildren* 槽位 None（C++ 继承基类默认，
  // 替换仍须重写经脏内部子节点到达的已 clean 父节点的子树），visit 槽转发到
  // 独立的 `ignore_children_visit_*` 覆写。ic_ty/ic_tp 占位不消费（thunk 组
  // 由 ic_present: none 关闭）。
  (visit_only, $vis:vis $host:ident) => {
    substitution_vtable!(
      @assemble $vis $host,
      is_dirty_tp: real, clean_tp: real, ic_ty: unused, ic_tp: unused,
      ic_present: none, visit_ic: distinct);
  };
  (@assemble $vis:vis $host:ident,
      is_dirty_tp: $idt:ident, clean_tp: $clt:ident, ic_ty: $ict:ident, ic_tp: $icp:ident,
      ic_present: $icn:ident, visit_ic: $vv:ident) => {
    impl $host {
      /// 本宏每次展开唯一的 `owner` 还原边界：thunk 全部经此分发器把
      /// `vtable.owner`（`self as *mut $host as *mut ()` 的同址类型擦除回指）
      /// 还原为 `&mut Self` 并执行覆写调用。
      ///
      /// # Safety（收口后的统一契约，论证见模块文档）
      /// `owner` 只可能由同对象的 `install_substitution_vtable` 写入；thunk 仅在
      /// 宿主自身 `substitute_*` 遍历的同步回调栈内被派发，此刻宿主被调用栈经
      /// `&mut self` 独占驱动，回调窗口内重建 `&mut Self` 无并存借用（crate
      /// 单线程不变量）。
      #[inline]
      fn svt_with<R>(owner: *mut (), with: impl FnOnce(&mut Self) -> R) -> R {
        // SAFETY: `owner` 为非空、对齐、类型正确的存活 `$host` 地址（恒等往返），
        // 回调窗口内独占可变借用——前提即上方契约，由装表与遍历时序兑现。
        unsafe { with(&mut *(owner.cast::<Self>())) }
      }
    }

    fn svt_is_dirty_ty(owner: *mut (), ty: $crate::type_aliases::type_id::TypeId) -> bool {
      $host::svt_with(owner, |host| host.is_dirty_type_id(ty))
    }
    fn svt_is_dirty_tp(
      owner: *mut (),
      tp: $crate::type_aliases::type_pack_id::TypePackId,
    ) -> bool {
      substitution_vtable!(@pack_slot $idt, $host, owner, tp, is_dirty_type_pack_id)
    }
    fn svt_clean_ty(owner: *mut (), ty: $crate::type_aliases::type_id::TypeId)
    -> $crate::type_aliases::type_id::TypeId {
      $host::svt_with(owner, |host| host.clean_type_id(ty))
    }
    fn svt_clean_tp(
      owner: *mut (),
      tp: $crate::type_aliases::type_pack_id::TypePackId,
    ) -> $crate::type_aliases::type_pack_id::TypePackId {
      substitution_vtable!(@pack_slot $clt, $host, owner, tp, clean_type_pack_id)
    }
    fn svt_found_dirty_ty(owner: *mut (), ty: $crate::type_aliases::type_id::TypeId) {
      $host::svt_with(owner, |host| host.base.found_dirty_type_id(ty))
    }
    fn svt_found_dirty_tp(owner: *mut (), tp: $crate::type_aliases::type_pack_id::TypePackId) {
      $host::svt_with(owner, |host| host.base.found_dirty_type_pack_id(tp))
    }

    substitution_vtable!(@ic_thunks $icn, $host, $ict, $icp);
    substitution_vtable!(@visit_thunks $vv, $host);

    impl $host {
      $vis fn install_substitution_vtable(&mut self) {
        let owner = self as *mut $host as *mut ();
        self.base.base.vtable = $crate::records::tarjan::SubstitutionVtable {
          owner,
          is_dirty_ty: Some(svt_is_dirty_ty),
          is_dirty_tp: Some(svt_is_dirty_tp),
          clean_ty: Some(svt_clean_ty),
          clean_tp: Some(svt_clean_tp),
          found_dirty_ty: Some(svt_found_dirty_ty),
          found_dirty_tp: Some(svt_found_dirty_tp),
          ignore_children_ty: substitution_vtable!(@ic_ty_slot $icn),
          ignore_children_tp: substitution_vtable!(@ic_tp_slot $icn),
          ignore_children_visit_ty: substitution_vtable!(@visit_ty_slot $vv),
          ignore_children_visit_tp: substitution_vtable!(@visit_tp_slot $vv),
        };
      }
    }
  };
  // pack 侧槽位三形态：real = 转发宿主覆写；false = C++ 基类默认；
  // identity = 恒等透传（Instantiation::clean(TypePackId) 原样返回 tp）。
  (@pack_slot real, $host:ident, $owner:ident, $tp:ident, $m:ident) => {
    $host::svt_with($owner, |host| host.$m($tp))
  };
  (@pack_slot false, $host:ident, $owner:ident, $tp:ident, $m:ident) => {
    {
      let _ = ($owner, $tp);
      false
    }
  };
  (@pack_slot identity, $host:ident, $owner:ident, $tp:ident, $m:ident) => {
    {
      let _ = $owner;
      $tp
    }
  };
  // ignoreChildren thunk 组开关：常规宿主生成两支（ty 转发 $ict 覆写、tp 走
  // $icp 槽位形态）；visit_only 宿主不生成（装表落 None）。
  (@ic_thunks thunks, $host:ident, $ict:ident, $icp:ident) => {
    fn svt_ignore_children_ty(
      owner: *mut (),
      ty: $crate::type_aliases::type_id::TypeId,
    ) -> bool {
      $host::svt_with(owner, |host| host.$ict(ty))
    }
    fn svt_ignore_children_tp(
      owner: *mut (),
      tp: $crate::type_aliases::type_pack_id::TypePackId,
    ) -> bool {
      substitution_vtable!(@pack_slot $icp, $host, owner, tp, ignore_children_type_pack_id)
    }
  };
  (@ic_thunks none, $host:ident, $ict:ident, $icp:ident) => {};
  // visit 槽 thunk 组开关：alias = 与 ignoreChildren 槽共用同一 thunk（常规
  // 宿主，C++ 两个虚槽指向同一覆写）；distinct = visit_only 宿主转发到独立
  // 的 `ignore_children_visit_*` 覆写。
  (@visit_thunks distinct, $host:ident) => {
    fn svt_ignore_children_visit_ty(
      owner: *mut (),
      ty: $crate::type_aliases::type_id::TypeId,
    ) -> bool {
      $host::svt_with(owner, |host| host.ignore_children_visit_type_id(ty))
    }
    fn svt_ignore_children_visit_tp(
      owner: *mut (),
      tp: $crate::type_aliases::type_pack_id::TypePackId,
    ) -> bool {
      $host::svt_with(owner, |host| host.ignore_children_visit_type_pack_id(tp))
    }
  };
  (@visit_thunks alias, $host:ident) => {};
  // 装表槽位取值（表达式位宏）：thunks/none 与 alias/distinct 的四路组合。
  (@ic_ty_slot thunks) => { Some(svt_ignore_children_ty) };
  (@ic_ty_slot none) => { None };
  (@ic_tp_slot thunks) => { Some(svt_ignore_children_tp) };
  (@ic_tp_slot none) => { None };
  (@visit_ty_slot alias) => { Some(svt_ignore_children_ty) };
  (@visit_ty_slot distinct) => { Some(svt_ignore_children_visit_ty) };
  (@visit_tp_slot alias) => { Some(svt_ignore_children_tp) };
  (@visit_tp_slot distinct) => { Some(svt_ignore_children_visit_tp) };
}

pub(crate) use substitution_vtable;
