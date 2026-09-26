use crate::{
  records::{
    apply_mapped_generics::ApplyMappedGenerics, arena_handle::Handle, builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter, subtyping_environment::SubtypingEnvironment,
    tarjan::SubstitutionVtable, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ApplyMappedGenerics {
  pub fn apply_mapped_generics(
    &mut self,
    builtin_types: Handle<BuiltinTypes>,
    arena: Handle<TypeArena>,
    env: &mut SubtypingEnvironment,
    ice_reporter: *mut InternalErrorReporter,
  ) {
    self.builtin_types = builtin_types;
    self.arena = arena;
    self.env = env as *mut _;
    self.ice_reporter = ice_reporter;
  }
}

// ---------------------------------------------------------------------------
// Substitution virtual-override dispatch for `ApplyMappedGenerics`.
//
// C++ `ApplyMappedGenerics` overrides `isDirty` / `clean` / `ignoreChildren`
// (it does NOT override `ignoreChildrenVisit`, so that defaults to
// `ignoreChildren`). The inherited `substitute` traversal dispatches into these
// at runtime. Each thunk casts the type-erased `owner` data pointer back to the
// concrete `*mut ApplyMappedGenerics` it was installed for and calls the inherent
// override. `found_dirty` is `Substitution`'s own (non-overridden) method, so its
// thunk forwards to `self.base.found_dirty_*`.
// ---------------------------------------------------------------------------

fn amg_is_dirty_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: `owner` 只可能由本模块 install_substitution_vtable 写入（`self as
  // *mut ApplyMappedGenerics as *mut ()`），所以回转类型即真实类型；该对象是
  // SubtypingEnvironment::apply_mapped_generics 里的栈上 `amg`，在本次 substitute
  // 遍历结束前不移动、不释放。被调 is_dirty_type_id 只经 `(*self.env)` 读
  // mapped_generics 映射表（env 指向调用方独占持有的 SubtypingEnvironment），不改写
  // Tarjan 侧字段，故与遍历代码对同一对象的访问按调用时序串接、无并存可变借用。
  unsafe { (*(owner as *mut ApplyMappedGenerics)).is_dirty_type_id(ty) }
}

fn amg_is_dirty_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: owner 的来源与类型一致性同 ty 版（本模块唯一装配点）；这里转发的
  // contains_mapped_pack 是对 env 内 pack 侧映射表的只读查询，tp 为 arena 存活句柄。
  unsafe { (*(owner as *mut ApplyMappedGenerics)).is_dirty_type_pack_id(tp) }
}

fn amg_clean_ty(owner: *mut (), ty: TypeId) -> TypeId {
  // Safety: `owner` 回转同上——动态类型由装配点唯一性保证。clean_type_id 是 unsafe fn，
  // 其入参契约（env 存活可读、ice_reporter 非空、builtin_types/arena 为会话内存活对象）
  // 全部由 apply_mapped_generics() 构造 amg 时注入的同一批指针满足；它只向 arena 追加
  // 上下界并集/交集节点，写入目标与遍历持有的 Tarjan 状态不重叠。
  unsafe { (*(owner as *mut ApplyMappedGenerics)).clean_type_id(ty) }
}

fn amg_clean_tp(owner: *mut (), tp: TypePackId) -> TypePackId {
  // Safety: 同一装配点给出的 owner 不变量；clean_type_pack_id 只借 `&*self.env` 查
  // lookup_generic_pack，兜底路径 Copy 出 builtin_types 的 any_type_pack 句柄。
  unsafe { (*(owner as *mut ApplyMappedGenerics)).clean_type_pack_id(tp) }
}

fn amg_found_dirty_ty(owner: *mut (), ty: TypeId) {
  // Safety: 该覆写未被 AMG 覆盖，故先回转 owner 再走基类 `Substitution`（`.base` 是
  // 首字段，地址偏移一致）。found_dirty_type_id 要求 `base.log` 非空指向存活 TxnLog、
  // vtable 已装配——前者由 `Substitution::substitution_new(TxnLog::empty(), arena)`
  // 给出，后者正是 install_substitution_vtable 在 substitute 之前刚做的事。
  unsafe {
    (*(owner as *mut ApplyMappedGenerics))
      .base
      .found_dirty_type_id(ty)
  }
}

fn amg_found_dirty_tp(owner: *mut (), tp: TypePackId) {
  // Safety: pack 侧孪生分支——同样的 owner 装配不变量，同样的 TxnLog/`new_packs`
  // 前置条件（基类 traverse 期间独占驱动，无并发访问该对象）。
  unsafe {
    (*(owner as *mut ApplyMappedGenerics))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn amg_ignore_children_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 回转合法（唯一装配点）。ignore_children_type_id 会 `&*self.env`
  // 读 function 的 generics 界，并最终 `(*ty).persistent` 解引用入参——`ty` 由 Tarjan
  // 从 nodes/stack 快照取出，始终是 arena 内存活节点句柄。
  unsafe { (*(owner as *mut ApplyMappedGenerics)).ignore_children_type_id(ty) }
}

fn amg_ignore_children_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: 与 ty 版同源，仅把 `(*tp).persistent` 的读取换成 pack 节点；tp 同样取自
  // 遍历侧已拷贝出的句柄快照，指向 arena 内存活的 TypePackVar。
  unsafe { (*(owner as *mut ApplyMappedGenerics)).ignore_children_type_pack_id(tp) }
}

// AMG does not override ignoreChildrenVisit; the base default forwards to ignoreChildren.
fn amg_ignore_children_visit_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: 复刻 C++ 基类默认行为（不覆写 ignoreChildrenVisit 时落到 ignoreChildren），
  // 因此复用同一实现与其全部前置条件：owner 由本模块装配、env 存活、ty 为 arena 句柄。
  unsafe { (*(owner as *mut ApplyMappedGenerics)).ignore_children_type_id(ty) }
}

fn amg_ignore_children_visit_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: 上面 visit 变体的 pack 侧，转发目标与 ignore_children_tp 相同，
  // 前置条件（owner 类型匹配 + arena 存活 pack 节点）一并继承。
  unsafe { (*(owner as *mut ApplyMappedGenerics)).ignore_children_type_pack_id(tp) }
}

impl ApplyMappedGenerics {
  /// Point the embedded `Substitution`'s dispatch table at this object and its
  /// overrides. Must run with `self` at its final address (i.e. from a method
  /// called on a settled `&mut self`), so the `owner` pointer stays valid for
  /// the duration of the `substitute` traversal.
  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut ApplyMappedGenerics as *mut ();
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(amg_is_dirty_ty),
      is_dirty_tp: Some(amg_is_dirty_tp),
      clean_ty: Some(amg_clean_ty),
      clean_tp: Some(amg_clean_tp),
      found_dirty_ty: Some(amg_found_dirty_ty),
      found_dirty_tp: Some(amg_found_dirty_tp),
      ignore_children_ty: Some(amg_ignore_children_ty),
      ignore_children_tp: Some(amg_ignore_children_tp),
      ignore_children_visit_ty: Some(amg_ignore_children_visit_ty),
      ignore_children_visit_tp: Some(amg_ignore_children_visit_tp),
    };
  }

  /// Inherited `Substitution::substitute(TypeId)` with override dispatch wired.
  pub fn substitute_type_id(&mut self, ty: TypeId) -> Option<TypeId> {
    self.install_substitution_vtable();
    self.base.substitute_type_id(ty)
  }

  /// Inherited `Substitution::substitute(TypePackId)` with override dispatch wired.
  pub fn substitute_type_pack_id(&mut self, tp: TypePackId) -> Option<TypePackId> {
    self.install_substitution_vtable();
    self.base.substitute_type_pack_id(tp)
  }
}
