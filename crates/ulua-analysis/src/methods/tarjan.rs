//! `tarjan` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::ptr::null_mut;

use ulua_common::{fflag, fint, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::tarjan_result::TarjanResult,
  functions::{get_type, get_type_pack},
  records::{
    arena_handle::alias_ref,
    extern_type::ExternType,
    function_type::FunctionType,
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    negation_type::NegationType,
    pending_expansion_type::PendingExpansionType,
    table_type::TableType,
    tarjan::{Tarjan, TarjanEdge},
    tarjan_node::TarjanNode,
    tarjan_worklist_vertex::TarjanWorklistVertex,
    txn_log::TxnLog,
    r#type::Type,
    type_function_instance_type::TypeFunctionInstanceType,
    type_pack::TypePack,
    type_pack_var::TypePackVar,
    union_type::UnionType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{nominal_relation::NominalRelation, type_id::TypeId, type_pack_id::TypePackId},
};

impl Tarjan {
  pub fn clear_tarjan(&mut self, log: *const TxnLog) {
    let log = if log.is_null() { TxnLog::empty() } else { log };

    self.type_to_index.clear();
    self.pack_to_index.clear();

    self.nodes.clear();

    self.stack.clear();

    self.child_count = 0;

    self.log = log;

    self.edges.clear();
    self.worklist.clear();
  }
}

impl Tarjan {
  pub fn find_dirty_type_id(&mut self, ty: TypeId) -> TarjanResult {
    self.visit_root_type_id(ty)
  }

  pub fn find_dirty_type_pack_id(&mut self, tp: TypePackId) -> TarjanResult {
    self.visit_root_type_pack_id(tp)
  }
}

impl Tarjan {
  fn get_dirty(&self, index: i32) -> bool {
    let index_usize = index as usize;
    LUAU_ASSERT!(index_usize < self.nodes.len());
    self.nodes[index_usize].dirty
  }
}

impl Tarjan {
  /// C++ `Tarjan::ignoreChildren(TypeId)` (`Substitution.cpp:551-554`).
  ///
  /// The base returns `false`; concrete subclasses override it. Dispatches to
  /// the subclass override via the installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable) when
  /// present, else the base-class default.
  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    let owner = self.vtable.owner;
    match self.vtable.ignore_children_ty {
      Some(f) => f(owner, ty),
      None => false,
    }
  }

  /// C++ `Tarjan::ignoreChildren(TypePackId)` (`Substitution.cpp:556-559`).
  ///
  /// The base returns `false`; concrete subclasses override it. Dispatches to
  /// the subclass override via the installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable) when
  /// present, else the base-class default.
  pub fn ignore_children_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let owner = self.vtable.owner;
    match self.vtable.ignore_children_tp {
      Some(f) => f(owner, tp),
      None => false,
    }
  }
}

impl Tarjan {
  /// C++ `Tarjan::ignoreChildrenVisit(TypeId)` (`Substitution.cpp:562-565`).
  ///
  /// The base forwards to `ignoreChildren`; a few subclasses override the
  /// "visit" variant independently. Dispatches to the subclass override via
  /// the installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable) when
  /// present, else falls back to `ignoreChildren` (matching the base default,
  /// and fixing the previous self-recursive stub).
  pub fn ignore_children_visit_type_id(&mut self, ty: TypeId) -> bool {
    let owner = self.vtable.owner;
    match self.vtable.ignore_children_visit_ty {
      Some(f) => f(owner, ty),
      None => self.ignore_children_type_id(ty),
    }
  }

  /// C++ `Tarjan::ignoreChildrenVisit(TypePackId)` (`Substitution.cpp:567-570`).
  ///
  /// The base forwards to `ignoreChildren`; a few subclasses override the
  /// "visit" variant independently. Dispatches to the subclass override via
  /// the installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable) when
  /// present, else falls back to `ignoreChildren` (matching the base default,
  /// and fixing the previous self-recursive stub).
  pub fn ignore_children_visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let owner = self.vtable.owner;
    match self.vtable.ignore_children_visit_tp {
      Some(f) => f(owner, tp),
      None => self.ignore_children_type_pack_id(tp),
    }
  }
}

impl Tarjan {
  pub(crate) fn indexify_type_id(&mut self, ty: TypeId) -> (i32, bool) {
    // Safety: self.log 由 clear_tarjan/reset_state 在每轮 substitute 前接线，取值只
    // 可能是调用方存活的 &TxnLog 或 TxnLog::empty() 进程级单例指针，均非空且比本
    // 遍历长寿；follow_type_id 为 &self 只读解析，返回仍指向 arena 存活 Type。
    // 借用面仅覆盖本条只读解析调用，返回值是标量句柄，不与后续 self 字段写入交叠。
    let ty = alias_ref(self.log).follow_type_id(ty);

    if let Some(&index) = self.type_to_index.find(&ty) {
      (index, false)
    } else {
      let index = self.nodes.len() as i32;
      self.type_to_index.try_insert(ty, index);
      self.nodes.push(TarjanNode {
        ty,
        tp: null_mut(),
        on_stack: false,
        dirty: false,
        lowlink: index,
      });
      (index, true)
    }
  }

  pub(crate) fn indexify_type_pack_id(&mut self, mut tp: TypePackId) -> (i32, bool) {
    // Safety: 同 TypeId 版——self.log 为上轮接线保留的非空存活 TxnLog 句柄，
    // follow_type_pack_id 只读沿日志链解析，返回仍指向 arena 存活 TypePack。
    tp = alias_ref(self.log).follow_type_pack_id(tp);

    if let Some(&index) = self.pack_to_index.find(&tp) {
      (index, false)
    } else {
      let index = self.nodes.len() as i32;

      self.pack_to_index.try_insert(tp, index);

      self.nodes.push(TarjanNode {
        ty: null_mut(),
        tp,
        on_stack: false,
        dirty: false,
        lowlink: index,
      });

      (index, true)
    }
  }
}

impl Tarjan {
  pub fn loop_item(&mut self) -> TarjanResult {
    while !self.worklist.is_empty() {
      let (index, mut curr_edge, mut last_edge) = {
        let top = self
          .worklist
          .last()
          .expect("外层 !worklist.is_empty() 判据");
        (top.index, top.curr_edge, top.last_edge)
      };

      // First visit
      if curr_edge == -1 {
        self.child_count += 1;
        if self.child_limit > 0 && self.child_limit <= self.child_count {
          return TarjanResult::TooManyChildren;
        }

        self.stack.push(index);

        self.nodes[index as usize].on_stack = true;

        curr_edge = self.edges.len() as i32;

        // Fill in edge list of this vertex
        let ty = self.nodes[index as usize].ty;
        if !ty.is_null() {
          self.visit_children_type_id_i32(ty, index);
        } else {
          let tp = self.nodes[index as usize].tp;
          if !tp.is_null() {
            self.visit_children_type_pack_id_i32(tp, index);
          }
        }

        last_edge = self.edges.len() as i32;

        // Persist updated curr/last edge values back into worklist entry
        if let Some(top) = self.worklist.last_mut() {
          top.curr_edge = curr_edge;
          top.last_edge = last_edge;
        }
      }

      // Visit children
      let mut found_fresh = false;
      while curr_edge < last_edge {
        let (child_index, fresh) = match self.edges[curr_edge as usize] {
          TarjanEdge::Type(edge_ty) => self.indexify_type_id(edge_ty),
          TarjanEdge::Pack(edge_tp) => self.indexify_type_pack_id(edge_tp),
        };

        if fresh {
          // Original recursion point, update the parent continuation point and start the new element
          if let Some(top) = self.worklist.last_mut() {
            top.curr_edge = curr_edge + 1;
            // top.last_edge unchanged
          }
          self.worklist.push(TarjanWorklistVertex {
            index: child_index,
            curr_edge: -1,
            last_edge: -1,
          });
          found_fresh = true;
          break;
        } else if self.nodes[child_index as usize].on_stack {
          let ll = self.nodes[index as usize].lowlink;
          let other = child_index;
          if other < ll {
            self.nodes[index as usize].lowlink = other;
          }
        }

        self.visit_edge(child_index, index);

        curr_edge += 1;
      }

      if found_fresh {
        continue;
      }

      if self.nodes[index as usize].lowlink == index {
        self.visit_scc(index);

        while !self.stack.is_empty() {
          let popped = *self.stack.last().expect("外层 !stack.is_empty() 判据");
          self.stack.pop();

          self.nodes[popped as usize].on_stack = false;
          if popped == index {
            break;
          }
        }
      }

      self.worklist.pop();

      // Original return from recursion into a child
      if !self.worklist.is_empty() {
        let (parent_index, _parent_curr_edge, parent_end_edge) = {
          let top = self
            .worklist
            .last()
            .expect("外层 !worklist.is_empty() 判据");
          (top.index, top.curr_edge, top.last_edge)
        };

        // No need to keep child edges around
        let new_len = parent_end_edge as usize;
        self.edges.truncate(new_len);

        let child_lowlink = self.nodes[index as usize].lowlink;
        let parent_lowlink_ref = &mut self.nodes[parent_index as usize].lowlink;
        if child_lowlink < *parent_lowlink_ref {
          *parent_lowlink_ref = child_lowlink;
        }

        self.visit_edge(index, parent_index);
      }
    }

    TarjanResult::Ok
  }
}

impl Tarjan {
  fn set_dirty(&mut self, index: i32, d: bool) {
    let index_usize = index as usize;
    LUAU_ASSERT!(index_usize < self.nodes.len());
    self.nodes[index_usize].dirty = d;
  }
}

impl Tarjan {
  pub fn tarjan(&mut self) {
    // 对齐 cpp Substitution.cpp:10,161-171：预分配容量读取可调的
    // FInt::LuauTarjanPreallocationSize（默认 256），恢复其运行时可调语义；
    // cpp 断言桶数须为 2 的幂，此处向上取整到最近的 2 的幂以兼容任意取值。
    let raw = fint::LuauTarjanPreallocationSize.get().max(0) as usize;
    let preallocation_size = if raw == 0 { 0 } else { raw.next_power_of_two() };

    self.nodes.reserve(preallocation_size);
    self.stack.reserve(preallocation_size);
    self.edges.reserve(preallocation_size);
    self.worklist.reserve(preallocation_size);

    // cpp 在 Tarjan 构造时以同一 FInt 预分配 typeToIndex/packToIndex 两个
    // DenseHashMap 的桶；Rust 侧 map 先于本构造体创建，这里对两个空索引
    // map 做同容量的桶预留。
    self.type_to_index.reserve_buckets(preallocation_size);
    self.pack_to_index.reserve_buckets(preallocation_size);
  }
}

impl Tarjan {
  fn visit_child_optional_ty<Ty>(&mut self, ty: Option<Ty>)
  where
    Ty: Into<TypeId>,
  {
    if let Some(inner_ty) = ty {
      self.visit_child_type_id(inner_ty.into());
    }
  }

  pub(crate) fn visit_child_type_id(&mut self, ty: TypeId) {
    let ty = alias_ref(self.log).follow_type_id(ty);

    self.edges.push(TarjanEdge::Type(ty));
  }

  pub(crate) fn visit_child_type_pack_id(&mut self, tp: TypePackId) {
    let tp = alias_ref(self.log).follow_type_pack_id(tp);

    self.edges.push(TarjanEdge::Pack(tp));
  }
}

impl Tarjan {
  pub fn visit_children_type_id_i32(&mut self, ty: TypeId, _index: i32) {
    let mut ty = ty;
    // Safety: self.log 由 clear_tarjan/reset_state 在每轮 substitute 前接线，取值只
    // 可能是调用方存活的 &TxnLog 或 TxnLog::empty() 进程级单例指针，均非空且比本次
    // 遍历长寿；follow_type_id 按 &self 只读。
    LUAU_ASSERT!(ty == alias_ref(self.log).follow_type_id(ty));

    if self.ignore_children_visit_type_id(ty) {
      return;
    }

    // Safety: 同上，self.log 非空存活；pending_type_id 沿 parent 链只读查找，返回值
    // 要么是日志映射中 Box<PendingType> 堆对象的指针、要么是 null。
    let pty = alias_ref(self.log).pending_type_id(ty);
    if !pty.is_null() {
      // Safety: pty 指向 Box<PendingType> 的堆对象——容器 rehash 只移动 Box 指针不
      // 移动堆内容，且本轮遍历期间不回滚该日志，pending 字段保持存活；仅取共享引用。
      ty = unsafe { &(*pty).pending as *const Type };
    }

    if let Some(ftv) = get_type::get::<FunctionType>(ty) {
      for generic in ftv.generics.iter() {
        self.visit_child_type_id(*generic);
      }
      for generic_pack in ftv.generic_packs.iter() {
        self.visit_child_type_pack_id(*generic_pack);
      }

      self.visit_child_type_pack_id(ftv.arg_types);
      self.visit_child_type_pack_id(ftv.ret_types);
      return;
    }

    if let Some(ttv) = get_type::get::<TableType>(ty) {
      LUAU_ASSERT!(ttv.bound_to.is_none());
      for prop in ttv.props.values() {
        self.visit_child_optional_ty(prop.read_ty);
        self.visit_child_optional_ty(prop.write_ty);
      }

      if let Some(ref indexer) = ttv.indexer {
        self.visit_child_type_id(indexer.index_type);
        self.visit_child_type_id(indexer.index_result_type);
      }

      for itp in ttv.instantiated_type_params.iter() {
        self.visit_child_type_id(*itp);
      }

      for itp in ttv.instantiated_type_pack_params.iter() {
        self.visit_child_type_pack_id(*itp);
      }
      return;
    }

    if let Some(mtv) = get_type::get::<MetatableType>(ty) {
      self.visit_child_type_id(mtv.table);
      self.visit_child_type_id(mtv.metatable);
      return;
    }

    if let Some(utv) = get_type::get::<UnionType>(ty) {
      for opt in utv.options.iter() {
        self.visit_child_type_id(*opt);
      }
      return;
    }

    if let Some(itv) = get_type::get::<IntersectionType>(ty) {
      for part in itv.parts.iter() {
        self.visit_child_type_id(*part);
      }
      return;
    }

    if let Some(petv) = get_type::get::<PendingExpansionType>(ty) {
      for a in petv.type_arguments.iter() {
        self.visit_child_type_id(*a);
      }
      for a in petv.pack_arguments.iter() {
        self.visit_child_type_pack_id(*a);
      }
      return;
    }

    if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(ty) {
      for a in tfit.type_arguments.iter() {
        self.visit_child_type_id(*a);
      }
      for a in tfit.pack_arguments.iter() {
        self.visit_child_type_pack_id(*a);
      }
      return;
    }

    if let Some(etv) = get_type::get::<ExternType>(ty) {
      for prop in etv.props.values() {
        if prop.read_ty.is_some() {
          self.visit_child_optional_ty(prop.read_ty);
        }
        if prop.write_ty.is_some() {
          self.visit_child_optional_ty(prop.write_ty);
        }
      }

      if let Some(parent) = etv.parent {
        self.visit_child_type_id(parent);
      }

      if let Some(metatable) = etv.metatable {
        self.visit_child_type_id(metatable);
      }

      if let Some(ref indexer) = etv.indexer {
        self.visit_child_type_id(indexer.index_type);
        self.visit_child_type_id(indexer.index_result_type);
      }

      if fflag::DebugLuauUserDefinedClasses.get()
        && let Some(ref relation) = etv.relation
      {
        match relation {
          NominalRelation::V0(obj) => {
            self.visit_child_type_id(obj.ty);
          }
          NominalRelation::V1(klass) => {
            self.visit_child_type_id(klass.ty);
          }
        }
      }
      return;
    }

    if let Some(ntv) = get_type::get::<NegationType>(ty) {
      self.visit_child_type_id(ntv.ty);
    }
  }

  pub fn visit_children_type_pack_id_i32(&mut self, tp: TypePackId, _index: i32) {
    let mut tp = tp;
    // Safety: 与 TypeId 版同源——self.log 每轮 substitute 前由 reset_state 接线为
    // 存活 &TxnLog 或 TxnLog::empty() 单例，非空且长寿；follow_type_pack_id 只读。
    LUAU_ASSERT!(tp == alias_ref(self.log).follow_type_pack_id(tp));

    if self.ignore_children_visit_type_pack_id(tp) {
      return;
    }

    // Safety: self.log 非空存活（同上）；pending_type_pack_id 沿 parent 链只读查找，
    // 返回 Box<PendingTypePack> 堆对象指针或 null。
    let ptp = alias_ref(self.log).pending_type_pack_id(tp);
    if !ptp.is_null() {
      // Safety: ptp 指向 Box 堆对象，容器重排不移动堆内容；遍历期间日志不回滚，
      // pending 字段存活，仅取共享引用。
      tp = unsafe { &(*ptp).pending as *const TypePackVar };
    }

    if let Some(tpp) = get_type_pack::get::<TypePack>(tp) {
      for tv in tpp.head.iter() {
        self.visit_child_type_id(*tv);
      }
      if let Some(tail) = tpp.tail {
        self.visit_child_type_pack_id(tail);
      }
      return;
    }

    if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tp) {
      self.visit_child_type_id(vtp.ty);
    }
  }
}

impl Tarjan {
  fn visit_edge(&mut self, index: i32, parent_index: i32) {
    let is_dirty = Tarjan::get_dirty(self, index);
    if is_dirty {
      Tarjan::set_dirty(self, parent_index, true);
    }
  }
}

impl Tarjan {
  pub(crate) fn visit_root_type_id(&mut self, ty: TypeId) -> TarjanResult {
    self.child_count = 0;
    if self.child_limit == 0 {
      self.child_limit = fint::LuauTarjanChildLimit.get();
    }

    let ty = alias_ref(self.log).follow_type_id(ty);

    let (index, _fresh) = self.indexify_type_id(ty);
    self.worklist.push(TarjanWorklistVertex {
      index,
      curr_edge: -1,
      last_edge: -1,
    });

    self.loop_item()
  }

  pub(crate) fn visit_root_type_pack_id(&mut self, tp: TypePackId) -> TarjanResult {
    self.child_count = 0;
    if self.child_limit == 0 {
      self.child_limit = fint::LuauTarjanChildLimit.get();
    }

    let tp = alias_ref(self.log).follow_type_pack_id(tp);

    let (index, _fresh) = self.indexify_type_pack_id(tp);
    self.worklist.push(TarjanWorklistVertex {
      index,
      curr_edge: -1,
      last_edge: -1,
    });

    self.loop_item()
  }
}

impl Tarjan {
  /// C++ `Tarjan::visitSCC(int)` (`Substitution.cpp:515-549`).
  ///
  /// `isDirty` and `foundDirty` are pure-virtual in C++; here they dispatch to
  /// the subclass through the installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable).
  fn visit_scc(&mut self, index: i32) {
    let mut d = self.get_dirty(index);

    let owner = self.vtable.owner;
    let is_dirty_ty = self.vtable.is_dirty_ty;
    let is_dirty_tp = self.vtable.is_dirty_tp;

    // Snapshot of the stack iterated rbegin..rend (the C++ reverse iterators).
    // Taken once: it is also reused for the foundDirty pass below, and copying
    // it avoids holding a borrow of `self.stack` across the `&mut self`
    // mutations / vtable callbacks.
    let stack_rev: Vec<i32> = self.stack.iter().copied().rev().collect();

    for &it in stack_rev.iter() {
      if d {
        break;
      }

      let node = &self.nodes[it as usize];
      let nty = node.ty;
      let ntp = node.tp;

      if !nty.is_null() {
        if let Some(f) = is_dirty_ty {
          d = f(owner, nty);
        }
      } else if !ntp.is_null()
        && let Some(f) = is_dirty_tp
      {
        d = f(owner, ntp);
      }

      if it == index {
        break;
      }
    }

    if !d {
      return;
    }

    let found_dirty_ty = self.vtable.found_dirty_ty;
    let found_dirty_tp = self.vtable.found_dirty_tp;

    for &it in stack_rev.iter() {
      self.set_dirty(it, true);

      let node = &self.nodes[it as usize];
      let nty = node.ty;
      let ntp = node.tp;

      if !nty.is_null() {
        if let Some(f) = found_dirty_ty {
          f(owner, nty);
        }
      } else if !ntp.is_null()
        && let Some(f) = found_dirty_tp
      {
        f(owner, ntp);
      }

      if it == index {
        return;
      }
    }
  }
}
