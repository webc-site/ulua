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
//! 各臂差异只在 TypePackId 侧三个覆写槽（isDirty/clean/ignoreChildren 的 pack 形态）：
//! - `real`：pack 侧有真实覆写（Anyification/AMG/ApplyTypeFunction/RefineTypeScrubber）；
//! - `false_tp`：pack 侧 ignoreChildren 无覆写、C++ 基类默认 false
//!   （Demoter/Instantiation2/ReplaceGenerics/Replacer/Widen）；
//! - `degen_tp`：Instantiation 退化形态——pack 侧三槽全为基类默认（isDirty=false、
//!   clean 恒等透传、ignoreChildren=false）；
//! - `visit_only`：ClonePublicInterface 形态——ignoreChildren* 槽位 None（继承基类
//!   默认），visit 槽转发到独立的 `ignore_children_visit_*` 覆写。
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
//!    别名逃逸摊进整个遍历，现状把 unsafe 关在宏生成的 thunk 里（§2 最小契约边界）。
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

macro_rules! substitution_vtable {
  (real, $vis:vis $host:ident, ic = $ic_ty:ident) => {
    substitution_vtable!(
      @assemble $vis $host,
      is_dirty_tp: real, clean_tp: real, ic_ty: $ic_ty, ic_tp: real);
  };
  (false_tp, $vis:vis $host:ident, ic = $ic_ty:ident) => {
    substitution_vtable!(
      @assemble $vis $host,
      is_dirty_tp: real, clean_tp: real, ic_ty: $ic_ty, ic_tp: false);
  };
  (degen_tp, $vis:vis $host:ident, ic = $ic_ty:ident) => {
    substitution_vtable!(
      @assemble $vis $host,
      is_dirty_tp: false, clean_tp: identity, ic_ty: $ic_ty, ic_tp: false);
  };
  (@assemble $vis:vis $host:ident,
      is_dirty_tp: $idt:ident, clean_tp: $clt:ident, ic_ty: $ict:ident, ic_tp: $icp:ident) => {
    fn svt_is_dirty_ty(owner: *mut (), ty: $crate::type_aliases::type_id::TypeId) -> bool {
      unsafe { (*(owner as *mut $host)).is_dirty_type_id(ty) }
    }
    fn svt_is_dirty_tp(
      owner: *mut (),
      tp: $crate::type_aliases::type_pack_id::TypePackId,
    ) -> bool {
      substitution_vtable!(@pack_slot $idt, $host, owner, tp, is_dirty_type_pack_id)
    }
    fn svt_clean_ty(owner: *mut (), ty: $crate::type_aliases::type_id::TypeId)
    -> $crate::type_aliases::type_id::TypeId {
      unsafe { (*(owner as *mut $host)).clean_type_id(ty) }
    }
    fn svt_clean_tp(
      owner: *mut (),
      tp: $crate::type_aliases::type_pack_id::TypePackId,
    ) -> $crate::type_aliases::type_pack_id::TypePackId {
      substitution_vtable!(@pack_slot $clt, $host, owner, tp, clean_type_pack_id)
    }
    fn svt_found_dirty_ty(owner: *mut (), ty: $crate::type_aliases::type_id::TypeId) {
      unsafe { (*(owner as *mut $host)).base.found_dirty_type_id(ty) }
    }
    fn svt_found_dirty_tp(owner: *mut (), tp: $crate::type_aliases::type_pack_id::TypePackId) {
      unsafe { (*(owner as *mut $host)).base.found_dirty_type_pack_id(tp) }
    }
    fn svt_ignore_children_ty(
      owner: *mut (),
      ty: $crate::type_aliases::type_id::TypeId,
    ) -> bool {
      unsafe { (*(owner as *mut $host)).$ict(ty) }
    }
    fn svt_ignore_children_tp(
      owner: *mut (),
      tp: $crate::type_aliases::type_pack_id::TypePackId,
    ) -> bool {
      substitution_vtable!(@pack_slot $icp, $host, owner, tp, ignore_children_type_pack_id)
    }

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
          ignore_children_ty: Some(svt_ignore_children_ty),
          ignore_children_tp: Some(svt_ignore_children_tp),
          ignore_children_visit_ty: Some(svt_ignore_children_ty),
          ignore_children_visit_tp: Some(svt_ignore_children_tp),
        };
      }
    }
  };
  // pack 侧槽位三形态：real = 转发宿主覆写；false = C++ 基类默认；
  // identity = 恒等透传（Instantiation::clean(TypePackId) 原样返回 tp）。
  (@pack_slot real, $host:ident, $owner:ident, $tp:ident, $m:ident) => {
    unsafe { (*($owner as *mut $host)).$m($tp) }
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
  // ClonePublicInterface 形态：ignoreChildren* 槽位 None（C++ 继承基类默认，
  // 替换仍须重写经脏内部子节点到达的已 clean 父节点的子树），visit 槽转发到
  // 独立的 `ignore_children_visit_*` 覆写。
  (visit_only, $vis:vis $host:ident) => {
    fn svt_is_dirty_ty(owner: *mut (), ty: $crate::type_aliases::type_id::TypeId) -> bool {
      unsafe { (*(owner as *mut $host)).is_dirty_type_id(ty) }
    }
    fn svt_is_dirty_tp(
      owner: *mut (),
      tp: $crate::type_aliases::type_pack_id::TypePackId,
    ) -> bool {
      unsafe { (*(owner as *mut $host)).is_dirty_type_pack_id(tp) }
    }
    fn svt_clean_ty(owner: *mut (), ty: $crate::type_aliases::type_id::TypeId)
    -> $crate::type_aliases::type_id::TypeId {
      unsafe { (*(owner as *mut $host)).clean_type_id(ty) }
    }
    fn svt_clean_tp(
      owner: *mut (),
      tp: $crate::type_aliases::type_pack_id::TypePackId,
    ) -> $crate::type_aliases::type_pack_id::TypePackId {
      unsafe { (*(owner as *mut $host)).clean_type_pack_id(tp) }
    }
    fn svt_found_dirty_ty(owner: *mut (), ty: $crate::type_aliases::type_id::TypeId) {
      unsafe { (*(owner as *mut $host)).base.found_dirty_type_id(ty) }
    }
    fn svt_found_dirty_tp(owner: *mut (), tp: $crate::type_aliases::type_pack_id::TypePackId) {
      unsafe { (*(owner as *mut $host)).base.found_dirty_type_pack_id(tp) }
    }
    fn svt_ignore_children_visit_ty(
      owner: *mut (),
      ty: $crate::type_aliases::type_id::TypeId,
    ) -> bool {
      unsafe { (*(owner as *mut $host)).ignore_children_visit_type_id(ty) }
    }
    fn svt_ignore_children_visit_tp(
      owner: *mut (),
      tp: $crate::type_aliases::type_pack_id::TypePackId,
    ) -> bool {
      unsafe { (*(owner as *mut $host)).ignore_children_visit_type_pack_id(tp) }
    }

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
          ignore_children_ty: None,
          ignore_children_tp: None,
          ignore_children_visit_ty: Some(svt_ignore_children_visit_ty),
          ignore_children_visit_tp: Some(svt_ignore_children_visit_tp),
        };
      }
    }
  };
}

pub(crate) use substitution_vtable;
