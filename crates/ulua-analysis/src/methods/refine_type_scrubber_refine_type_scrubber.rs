//! C++ `RefineTypeScrubber::RefineTypeScrubber(NotNull<TypeFunctionContext> ctx,
//! TypeId needle)` (BuiltinTypeFunctions.cpp:1083-1088). Base-inits the
//! `Substitution` with `ctx->arena`, then stores `ctx` and `needle`.

use crate::{
  macros::substitution_entry::substitution_entry,
  records::{
    arena_handle::Handle, refine_type_scrubber::RefineTypeScrubber, substitution::Substitution,
    tarjan::SubstitutionVtable, txn_log::TxnLog, type_function_context::TypeFunctionContext,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn refine_type_scrubber_is_dirty_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: `owner` 由本模块 install_substitution_vtable 以 `self as
  // *mut RefineTypeScrubber as *mut ()` 装入，且这组 fn 指针只写进该对象的 vtable，
  // 回转类型与动态类型必然一致；对象是 refine_type_function 里的栈上 `rts`，
  // 在 substitute_type_id 返回前不移动。is_dirty 只读 self.needle 与 arena 节点，无写入。
  unsafe { (*(owner as *mut RefineTypeScrubber)).is_dirty_type_id(ty) }
}

fn refine_type_scrubber_is_dirty_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: 同上——owner 回转合法；pack 侧覆写按 C++ 实现恒为 false，连 tp 都不解引用，
  // 故除 owner 存活外无额外前提。
  unsafe { (*(owner as *mut RefineTypeScrubber)).is_dirty_type_pack_id(tp) }
}

fn refine_type_scrubber_clean_ty(owner: *mut (), ty: TypeId) -> TypeId {
  // Safety: owner 指向装配期记录的自身地址（同 is_dirty_ty 的论证）。clean_type_id 会
  // `self.ctx.as_ref()` 再读 `builtins`/`arena`，二者都由 `new` 的 ctx 契约间接保证存活；
  // 写入只发生在 arena 节点追加，与遍历侧持有的 Tarjan 借用不重叠。
  unsafe { (*(owner as *mut RefineTypeScrubber)).clean_type_id(ty) }
}

fn refine_type_scrubber_clean_tp(owner: *mut (), tp: TypePackId) -> TypePackId {
  // Safety: pack 版覆写按 C++ 语义原样返回 tp（不解引用 ctx），因此只需 owner 回转出的
  // 自身引用有效——即上面同一条装配不变量。
  unsafe { (*(owner as *mut RefineTypeScrubber)).clean_type_pack_id(tp) }
}

fn refine_type_scrubber_found_dirty_ty(owner: *mut (), ty: TypeId) {
  // Safety: RTS 未覆写 foundDirty，回转 owner 后落到内嵌 `base: Substitution`。
  // 该 unsafe fn 要求 `base.log` 非空指向存活 TxnLog 且 vtable 已装配：前者由 `new`
  // 里的 `Substitution::substitution_new(TxnLog::empty(), arena)` 建立，后者由
  // substitute_type_id 在遍历前刚完成安装。
  unsafe {
    (*(owner as *mut RefineTypeScrubber))
      .base
      .found_dirty_type_id(ty)
  }
}

fn refine_type_scrubber_found_dirty_tp(owner: *mut (), tp: TypePackId) {
  // Safety: 与 ty 版同一基类转发链，同样的 TxnLog/已装配 vtable 前置条件；
  // 单线程独占驱动下遍历与回调对同一对象的访问严格串接。
  unsafe {
    (*(owner as *mut RefineTypeScrubber))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn refine_type_scrubber_ignore_children_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 回转合法性同前；ignore_children_type_id 用 get_type_id 只读判断
  // ty 是否 union/intersection，ty 由遍历侧从 nodes 快照取来，始终为 arena 内存活句柄。
  unsafe { (*(owner as *mut RefineTypeScrubber)).ignore_children_type_id(ty) }
}

fn refine_type_scrubber_ignore_children_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: pack 侧覆写为常量 false（不解引用 tp），只需 owner 指向存活的自身对象即可。
  unsafe { (*(owner as *mut RefineTypeScrubber)).ignore_children_type_pack_id(tp) }
}

impl RefineTypeScrubber {
  /// C++ `RefineTypeScrubber` ctor（BuiltinTypeFunctions.cpp:1083-1088）：以
  /// `ctx->arena` 基初始化 `Substitution`，并记下 `ctx` 与要剔除的 `needle`。
  ///
  /// # Safety
  /// - `ctx`：本次归约调用的独占借用，指向存活 [`TypeFunctionContext`]——本 ctor
  ///   立即读取其 `arena` 字段，且 `Substitution` 与后续所有覆写都会继续经它读写 arena。
  /// - `needle`：须是 `ctx.arena` 内一个存活的 `TypeId`（调用方传入的 instance 句柄）；
  ///   它只被按值比较，不被解引用。
  /// - 返回对象的 `ctx` 字段是 `Handle`（无生命周期追踪），其有效性完全继承自上面的
  ///   `ctx` 前提；使用该对象期间 `ctx` 所指记录不得失效、不得有并存改写。
  pub unsafe fn new(ctx: &mut TypeFunctionContext, needle: TypeId) -> Self {
    // Safety: 契约给出 ctx 为存活独占借用；此处只读其 arena 字段值（NonNull 拷贝）
    // 交给基类 ctor，不写该记录，也不延长这次共享再借用的生命周期。
    let ctx_ref = &*ctx;
    let base =
      Substitution::substitution_new(TxnLog::empty(), Some(Handle::from_nonnull(ctx_ref.arena)));
    RefineTypeScrubber {
      base,
      ctx: Handle::from_mut(ctx),
      needle,
    }
  }

  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut RefineTypeScrubber as *mut ();
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(refine_type_scrubber_is_dirty_ty),
      is_dirty_tp: Some(refine_type_scrubber_is_dirty_tp),
      clean_ty: Some(refine_type_scrubber_clean_ty),
      clean_tp: Some(refine_type_scrubber_clean_tp),
      found_dirty_ty: Some(refine_type_scrubber_found_dirty_ty),
      found_dirty_tp: Some(refine_type_scrubber_found_dirty_tp),
      ignore_children_ty: Some(refine_type_scrubber_ignore_children_ty),
      ignore_children_tp: Some(refine_type_scrubber_ignore_children_tp),
      ignore_children_visit_ty: Some(refine_type_scrubber_ignore_children_ty),
      ignore_children_visit_tp: Some(refine_type_scrubber_ignore_children_tp),
    };
  }

  substitution_entry!(id);
}
