//! `substitution` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::null;

use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  enums::tarjan_result::TarjanResult,
  functions::{
    follow_type, follow_type_pack, get_mutable_type, get_mutable_type_pack, get_type_pack,
    shallow_clone_substitution::shallow_clone_type_id_type_arena_txn_log,
  },
  records::{
    arena_handle::Handle,
    arena_id::ArenaId,
    extern_type::ExternType,
    function_type::FunctionType,
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    negation_type::NegationType,
    pending_expansion_type::PendingExpansionType,
    substitution::Substitution,
    table_type::TableType,
    tarjan::{SubstitutionVtable, Tarjan},
    txn_log::TxnLog,
    r#type::Type,
    type_arena::TypeArena,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_pack::TypePack,
    type_pack_var::TypePackVar,
    union_type::UnionType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Substitution {
  pub fn add_type<T>(&mut self, tv: T) -> TypeId
  where
    T: Into<Type>,
  {
    self.wired_arena_mut().add_tv(tv.into())
  }
}

impl Substitution {
  pub fn add_type_pack<T>(&mut self, tp: T) -> TypePackId
  where
    T: Into<TypePackVar>,
  {
    self.wired_arena_mut().add_type_pack_t(tp.into())
  }
}

impl Substitution {
  pub(crate) fn clone_type_id(&mut self, ty: TypeId) -> TypeId {
    // Safety: self.arena 对应构造期接线的 `NotNull<TypeArena>`，非空、比 Substitution 长寿；
    // 重建的 &mut 独占借用仅存活于本浅拷贝调用，单线程下此刻 arena 无其他借用故无别名冲突。
    let arena = self.wired_arena_mut();
    // Safety: 被调 unsafe fn 契约要求 dest/log 有效——arena 为上一步独占借用，self.base.log 为
    // Substitution 构造注入的非空 TxnLog（empty 单例或活动 log），ty 为遍历中存活的 arena 句柄。
    unsafe { shallow_clone_type_id_type_arena_txn_log(ty, arena, self.base.log) }
  }

  pub(crate) fn clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    // Safety: self.base.log 为 Substitution 构造注入的非空 TxnLog（empty 单例或会话活动 log），
    // follow_type_pack_id 取 &self 只读，返回遍历期内存活的 arena 类型包句柄。
    let mut tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

    // Safety: 同上 log 非空存活；pending_type_pack_id 取 &self 只读，返回指向 log 中存活
    // PendingTypePack 的裸指针（可能为 null，下一行判空）。
    let ptp = unsafe { (*self.base.log).pending_type_pack_id(tp) };
    if !ptp.is_null() {
      // Safety: `!ptp.is_null()` 已判空，ptp 指向 log 中存活的 PendingTypePack；共享读 .pending
      // 后取其地址作为 tp，log 节点在整段克隆遍历期内地址稳定、存活。
      tp = unsafe { &(*ptp).pending as *const TypePackVar };
    }

    if let Some(tpp) = get_type_pack::get::<TypePack>(tp) {
      return self.add_type_pack(TypePack::new(tpp.head.clone(), tpp.tail));
    }

    if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tp) {
      let clone = VariadicTypePack {
        ty: vtp.ty,
        hidden: vtp.hidden,
      };
      return self.add_type_pack(clone);
    }

    if let Some(tfitp) = get_type_pack::get::<TypeFunctionInstanceTypePack>(tp) {
      let clone = TypeFunctionInstanceTypePack {
        function: tfitp.function,
        type_arguments: tfitp.type_arguments.clone(),
        pack_arguments: tfitp.pack_arguments.clone(),
      };
      return self.add_type_pack(clone);
    }

    // Safety: tp 为 follow/pending 解析后的 *const TypePackVar；到达此兜底分支前，上方对
    // TypePack/VariadicTypePack/TypeFunctionInstanceTypePack 的 get_type_pack_id 已按同一地址
    // 成功解引用（RTTI 内部读 class_index），故 tp 非空且指向 arena/log 存活节点，clone 仅只读深拷贝。
    self.add_type_pack(unsafe { (*tp).clone() })
  }
}

impl Substitution {
  pub fn dont_traverse_into_type_id(&mut self, ty: TypeId) {
    self.no_traverse_types.insert(ty);
  }

  pub fn dont_traverse_into_type_pack_id(&mut self, tp: TypePackId) {
    self.no_traverse_type_packs.insert(tp);
  }
}

impl Substitution {
  /// # Safety
  /// `ty` 须为指向存活类型 arena 节点的有效 `TypeId`，且 `self.base.log` 须为
  /// 非空并指向存活的 `TxnLog`——首行 `(*self.base.log).follow_type_id(ty)` 会
  /// 解引用二者。`self.base.vtable` 的 `owner` 及 `is_dirty_ty`/`clean_ty` 覆写
  /// 须已安装（否则下方 `expect` panic）。
  /// C++ `Substitution::foundDirty(TypeId)` (`cpp/Analysis/src/Substitution.cpp:723`).
  ///
  /// The `isDirty` / `clean` calls are virtual in C++ and dispatch into the
  /// concrete subclass; here they go through the subclass-installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable). The
  /// first `follow` is `log->follow`; the second is `Luau::follow` (the free
  /// function `follow_type_id`).
  pub(crate) unsafe fn found_dirty_type_id(&mut self, ty: TypeId) {
    // Safety: `self.base.log` 为构造 Substitution 时接线的非空 `*const TxnLog`（进程级
    // `TxnLog::empty()` 单例或调用方传入的活 log），比本 Substitution 长寿；`follow_type_id`
    // 为 `&self` 只读，`ty` 是存活类型句柄。单线程求解循环中重建共享引用无并发别名。
    let ty = unsafe { (*self.base.log).follow_type_id(ty) };

    if self.new_types.contains(&ty) {
      return;
    }

    let owner = self.base.vtable.owner;
    let is_dirty = self
      .base
      .vtable
      .is_dirty_ty
      .expect("Substitution::isDirty(TypeId) override not installed");

    let new_ty = if is_dirty(owner, ty) {
      let clean = self
        .base
        .vtable
        .clean_ty
        .expect("Substitution::clean(TypeId) override not installed");
      let cleaned = clean(owner, ty);
      follow_type::follow(cleaned)
    } else {
      let cloned = self.clone_type_id(ty);
      follow_type::follow(cloned)
    };

    *self.new_types.get_or_insert(ty) = new_ty;
  }

  /// # Safety
  /// `tp` 须为指向存活类型 arena 节点的有效 `TypePackId`，且 `self.base.log` 非空并指向存活
  /// 的 `TxnLog`（首行 follow 会解引用二者）；`self.base.vtable` 的 `owner` 与 dirty/clean 覆写
  /// 须已安装。单线程、无并发写别名。
  /// C++ `Substitution::foundDirty(TypePackId)` (`cpp/Analysis/src/Substitution.cpp:736`).
  ///
  /// See [`Substitution::found_dirty_type_id`] for the dispatch/`follow`
  /// details; this is the type-pack twin.
  pub(crate) unsafe fn found_dirty_type_pack_id(&mut self, tp: TypePackId) {
    // Safety: 同类型孪生——`self.base.log` 为非空存活的 `*const TxnLog`，`follow_type_pack_id`
    // 为 `&self` 只读，`tp` 是存活类型包句柄；单线程求解中无并发别名。
    let tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

    if self.new_packs.contains(&tp) {
      return;
    }

    let owner = self.base.vtable.owner;
    let is_dirty = self
      .base
      .vtable
      .is_dirty_tp
      .expect("Substitution::isDirty(TypePackId) override not installed");

    let new_tp = if is_dirty(owner, tp) {
      let clean = self
        .base
        .vtable
        .clean_tp
        .expect("Substitution::clean(TypePackId) override not installed");
      let cleaned = clean(owner, tp);
      follow_type_pack::follow(cleaned)
    } else {
      let cloned = self.clone_type_pack_id(tp);
      follow_type_pack::follow(cloned)
    };

    *self.new_packs.get_or_insert(tp) = new_tp;
  }
}

impl Substitution {
  /// 对应 C++ `void Substitution::replaceChildren(TypeId ty)`
  /// （`cpp/Analysis/src/Substitution.cpp:769`）。降 safe：`ty` 为 arena `TypeId`
  /// 句柄（同 `get_mutable_type_id` 门面纪律，调用点传的是 Tarjan 产出的新类型），
  /// `self.base.log` 构造注入并 LUAU_ASSERT 非空（C++ 成员 `const TxnLog* log`）；
  /// 对二者的解引用收进下方窄 `unsafe` 块，本体改写下钻全部走安全 RTTI 门面。
  pub fn replace_children_type_id(&mut self, ty: TypeId) {
    // Safety: self.base.log 由 Substitution::substitution_new → substitution_txn_log_type_arena
    // 构造注入并 LUAU_ASSERT 非空，指向会话期内存活的 TxnLog；follow_type_id 取
    // &self 只读解链，仅在断言内使用，借用不跨越后续可变操作。
    unsafe {
      LUAU_ASSERT!(ty == (*self.base.log).follow_type_id(ty));
    }

    if self.base.ignore_children_type_id(ty) {
      return;
    }

    // 对应 C++ `ty->owningArena != arena`；探针窄块内证成句柄前提（上一行已断言
    // ty 为已展开的规范形态、非空存活）。
    if type_owning_arena(ty) != self.wired_arena_id() {
      return;
    }

    if let Some(ftv) = get_mutable_type::get_mutable::<FunctionType>(ty) {
      for generic in &mut ftv.generics {
        *generic = self.replace_type_id(*generic);
      }
      for generic_pack in &mut ftv.generic_packs {
        *generic_pack = self.replace_type_pack_id(*generic_pack);
      }
      ftv.arg_types = self.replace_type_pack_id(ftv.arg_types);
      ftv.ret_types = self.replace_type_pack_id(ftv.ret_types);
    } else if let Some(ttv) = get_mutable_type::get_mutable::<TableType>(ty) {
      LUAU_ASSERT!(ttv.bound_to.is_none());
      for prop in ttv.props.values_mut() {
        if let Some(read_ty) = prop.read_ty {
          prop.read_ty = Some(self.replace_type_id(read_ty));
        }
        if let Some(write_ty) = prop.write_ty {
          prop.write_ty = Some(self.replace_type_id(write_ty));
        }
      }
      if let Some(ref mut indexer) = ttv.indexer {
        indexer.index_type = self.replace_type_id(indexer.index_type);
        indexer.index_result_type = self.replace_type_id(indexer.index_result_type);
      }
      for itp in &mut ttv.instantiated_type_params {
        *itp = self.replace_type_id(*itp);
      }
      for itp in &mut ttv.instantiated_type_pack_params {
        *itp = self.replace_type_pack_id(*itp);
      }
    } else if let Some(mtv) = get_mutable_type::get_mutable::<MetatableType>(ty) {
      mtv.table = self.replace_type_id(mtv.table);
      mtv.metatable = self.replace_type_id(mtv.metatable);
    } else if let Some(utv) = get_mutable_type::get_mutable::<UnionType>(ty) {
      for opt in &mut utv.options {
        *opt = self.replace_type_id(*opt);
      }
    } else if let Some(itv) = get_mutable_type::get_mutable::<IntersectionType>(ty) {
      for part in &mut itv.parts {
        *part = self.replace_type_id(*part);
      }
    } else if let Some(petv) = get_mutable_type::get_mutable::<PendingExpansionType>(ty) {
      for a in &mut petv.type_arguments {
        *a = self.replace_type_id(*a);
      }
      for a in &mut petv.pack_arguments {
        *a = self.replace_type_pack_id(*a);
      }
    } else if let Some(tfit) = get_mutable_type::get_mutable::<TypeFunctionInstanceType>(ty) {
      for a in &mut tfit.type_arguments {
        *a = self.replace_type_id(*a);
      }
      for a in &mut tfit.pack_arguments {
        *a = self.replace_type_pack_id(*a);
      }
    } else if let Some(etv) = get_mutable_type::get_mutable::<ExternType>(ty) {
      for prop in etv.props.values_mut() {
        if let Some(read_ty) = prop.read_ty {
          prop.read_ty = Some(self.replace_type_id(read_ty));
        }
        if let Some(write_ty) = prop.write_ty {
          prop.write_ty = Some(self.replace_type_id(write_ty));
        }
      }
      if let Some(ref mut parent) = etv.parent {
        *parent = self.replace_type_id(*parent);
      }
      if let Some(ref mut metatable) = etv.metatable {
        *metatable = self.replace_type_id(*metatable);
      }
      if let Some(ref mut indexer) = etv.indexer {
        indexer.index_type = self.replace_type_id(indexer.index_type);
        indexer.index_result_type = self.replace_type_id(indexer.index_result_type);
      }
    } else if let Some(ntv) = get_mutable_type::get_mutable::<NegationType>(ty) {
      ntv.ty = self.replace_type_id(ntv.ty);
    }
  }

  /// 类型包孪生版。对应 C++ `void Substitution::replaceChildren(TypePackId tp)`
  /// （`cpp/Analysis/src/Substitution.cpp:871`）。降 safe 理由同上：log/句柄解引用
  /// 均在窄 `unsafe` 块内证成。
  pub fn replace_children_type_pack_id(&mut self, tp: TypePackId) {
    // Safety: self.base.log 由 Substitution::substitution_new → substitution_txn_log_type_arena
    // 构造注入并 LUAU_ASSERT 非空，指向会话期内存活的 TxnLog（对应 C++ 成员 `const TxnLog* log`）；
    // follow_type_pack_id 取 &self 只读解链，仅在断言内使用，借用不跨越后续可变操作。
    unsafe {
      LUAU_ASSERT!(tp == (*self.base.log).follow_type_pack_id(tp));
    }

    if self.base.ignore_children_type_pack_id(tp) {
      return;
    }

    // 同上：对应 C++ `tp->owningArena != arena`。
    if pack_owning_arena(tp) != self.wired_arena_id() {
      return;
    }

    if let Some(tpp) = get_mutable_type_pack::get_mutable::<TypePack>(tp) {
      for tv in tpp.head.iter_mut() {
        *tv = self.replace_type_id(*tv);
      }
      if let Some(tail) = tpp.tail {
        tpp.tail = Some(self.replace_type_pack_id(tail));
      }
    } else if let Some(vtp) = get_mutable_type_pack::get_mutable::<VariadicTypePack>(tp) {
      vtp.ty = self.replace_type_id(vtp.ty);
    } else if let Some(tfitp) =
      get_mutable_type_pack::get_mutable::<TypeFunctionInstanceTypePack>(tp)
    {
      for t in tfitp.type_arguments.iter_mut() {
        *t = self.replace_type_id(*t);
      }
      for t in tfitp.pack_arguments.iter_mut() {
        *t = self.replace_type_pack_id(*t);
      }
    }
  }
}
/// arena 句柄 `owning_arena` 只读探针，解引用收口在私有 helper（同
/// `clone_clone::type_is_persistent` 写法）；ty/tp 为 substitute 遍历传入的
/// arena 存活节点（bump 块地址不移动），仅拷贝值。
fn type_owning_arena(ty: TypeId) -> ArenaId {
  // SAFETY: 见函数 doc——存活对齐 arena 句柄，只读拷贝。
  unsafe { (*ty).owning_arena }
}
fn pack_owning_arena(tp: TypePackId) -> ArenaId {
  // SAFETY: 同上。
  unsafe { (*tp).owning_arena }
}

impl Substitution {
  pub(crate) fn replace_type_id(&mut self, ty: TypeId) -> TypeId {
    // Safety: self.base.log 是 Substitution 构造注入的非空 TxnLog 指针（进程级 empty 单例
    // 或会话活动 log，均比本 substitution 长寿）；follow_type_id 取 &self 只读遍历 parent 链，
    // 返回遍历期内存活的 arena TypeId，重建共享解引用不产生 &/&mut 冲突。
    let ty = unsafe { (*self.base.log).follow_type_id(ty) };
    match self.new_types.find(&ty) {
      Some(prev_ty) => *prev_ty,
      None => ty,
    }
  }

  pub(crate) fn replace_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    // Safety: 同上——self.base.log 为构造注入的非空 TxnLog（empty 单例或会话活动 log，
    // 比本对象长寿）；follow_type_pack_id 取 &self 只读遍历 parent 链，返回存活 arena 类型包句柄。
    let tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

    if let Some(prev_tp) = self.new_packs.find(&tp) {
      *prev_tp
    } else {
      tp
    }
  }
}

impl Substitution {
  pub fn reset_state(&mut self, log: *const TxnLog, arena: Handle<TypeArena>) {
    self.base.clear_tarjan(log);

    self.arena = Some(arena);

    self.new_types.clear();
    self.new_packs.clear();
    self.replaced_types.clear();
    self.replaced_type_packs.clear();

    self.no_traverse_types.clear();
    self.no_traverse_type_packs.clear();
  }
}

impl Substitution {
  pub(crate) fn substitute_type_id(&mut self, ty: TypeId) -> Option<TypeId> {
    // Safety: `self.base.log` 对应 C++ `NotNull<TxnLog>`，构造时以进程级单例
    // `TxnLog::empty()` 接线、恒非空且永生；`follow_type_id` 只读该事务日志。
    let ty = unsafe { (*self.base.log).follow_type_id(ty) };

    self.base.clear_tarjan(self.base.log);

    let result = self.base.find_dirty_type_id(ty);
    if result != TarjanResult::Ok {
      return None;
    }

    let new_types_clone = self.new_types.clone();
    for (old_ty, new_ty) in new_types_clone.iter() {
      if !self.base.ignore_children_type_id(*old_ty) && !self.replaced_types.contains(new_ty) {
        if !self.no_traverse_types.contains(new_ty) {
          // replace_children_type_id 已降 safe：log/arena 句柄前提在其窄块内证成。
          self.replace_children_type_id(*new_ty);
        }
        self.replaced_types.insert(*new_ty);
      }
    }

    let new_packs_clone = self.new_packs.clone();
    for (old_tp, new_tp) in new_packs_clone.iter() {
      if !self.base.ignore_children_type_pack_id(*old_tp)
        && !self.replaced_type_packs.contains(new_tp)
      {
        if !self.no_traverse_type_packs.contains(new_tp) {
          // 同上：replace_children_type_pack_id 已降 safe。
          self.replace_children_type_pack_id(*new_tp);
        }
        self.replaced_type_packs.insert(*new_tp);
      }
    }

    let new_ty = self.replace_type_id(ty);
    Some(new_ty)
  }

  pub(crate) fn substitute_type_pack_id(&mut self, tp: TypePackId) -> Option<TypePackId> {
    // Safety: `self.base.log` 为进程级单例 `TxnLog::empty()` 接线的非空、永生指针；
    // `follow_type_pack_id` 只读该事务日志。
    let tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

    self.base.clear_tarjan(self.base.log);

    let result = self.base.find_dirty_type_pack_id(tp);
    if result != TarjanResult::Ok {
      return None;
    }

    let new_types_clone = self.new_types.clone();
    for (old_ty, new_ty) in new_types_clone.iter() {
      if !self.base.ignore_children_type_id(*old_ty) && !self.replaced_types.contains(new_ty) {
        if !self.no_traverse_types.contains(new_ty) {
          // replace_children_type_id 已降 safe。
          self.replace_children_type_id(*new_ty);
        }
        self.replaced_types.insert(*new_ty);
      }
    }

    let new_packs_clone = self.new_packs.clone();
    for (old_tp, new_tp) in new_packs_clone.iter() {
      if !self.base.ignore_children_type_pack_id(*old_tp)
        && !self.replaced_type_packs.contains(new_tp)
      {
        if !self.no_traverse_type_packs.contains(new_tp) {
          // 同上：replace_children_type_pack_id 已降 safe。
          self.replace_children_type_pack_id(*new_tp);
        }
        self.replaced_type_packs.insert(*new_tp);
      }
    }

    Some(self.replace_type_pack_id(tp))
  }
}

impl Substitution {
  /// C++ `Substitution::Substitution(const TxnLog* log_, TypeArena* arena)`
  /// (`Substitution.cpp`). Builds a fresh `Substitution` value with empty
  /// containers and the given log/arena; the C++ base `Tarjan()` constructor
  /// reserves space for its worklists, mirrored by `Tarjan::tarjan`.
  pub fn substitution_new(log_: *const TxnLog, arena: Option<Handle<TypeArena>>) -> Self {
    let mut base = Tarjan {
      type_to_index: DenseHashMap::default(),
      pack_to_index: DenseHashMap::default(),
      nodes: Vec::new(),
      stack: Vec::new(),
      child_count: 0,
      child_limit: 0,
      log: null(),
      edges_ty: Vec::new(),
      edges_tp: Vec::new(),
      worklist: Vec::new(),
      vtable: SubstitutionVtable::null(),
    };
    base.tarjan();

    let mut this = Substitution {
      base,
      arena,
      new_types: DenseHashMap::default(),
      new_packs: DenseHashMap::default(),
      replaced_types: DenseHashSet::default(),
      replaced_type_packs: DenseHashSet::default(),
      no_traverse_types: DenseHashSet::default(),
      no_traverse_type_packs: DenseHashSet::default(),
    };
    this.substitution_txn_log_type_arena(log_, arena);
    this
  }

  pub fn substitution_txn_log_type_arena(
    &mut self,
    log_: *const TxnLog,
    arena: Option<Handle<TypeArena>>,
  ) {
    self.arena = arena;
    self.base.log = log_;
    LUAU_ASSERT!(!log_.is_null());
  }

  /// 已接线的 arena 可变视图：遍历期（`reset_state` 之后）恒非空；
  /// 构造占位期的 null 属契约违例，确定性 panic 而非 UB。
  pub(crate) fn wired_arena_mut(&self) -> &mut TypeArena {
    self
      .arena
      .expect("Substitution.arena 使用前必须已由 reset_state 接线")
      .get_mut()
  }

  /// 已接线 arena 的身份值，供与节点 `owning_arena` 做归属比较。
  pub(crate) fn wired_arena_id(&self) -> ArenaId {
    self.wired_arena_handle().get().arena_id
  }

  /// 已接线 arena 的句柄形态（判空即 panic，与 `wired_arena_id` 同一契约），
  /// 供以 `Handle<TypeArena>` 为形参的下游接口直传，避免句柄↔裸指针往返。
  pub(crate) fn wired_arena_handle(&self) -> Handle<TypeArena> {
    self
      .arena
      .expect("Substitution.arena 使用前必须已由 reset_state 接线")
  }
}
