//! `iterative_type_function_type_visitor` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_table::DenseDefault};

use crate::{
  records::{
    arena_handle::alias_ref,
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    type_function_any_type::TypeFunctionAnyType, type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_property::TypeFunctionProperty,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_indexer::TypeFunctionTableIndexer,
    type_function_table_type::TypeFunctionTableType, type_function_type_pack::TypeFunctionTypePack,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack, visit_key::VisitKeyRef,
    work_item_iterative_type_function_type_visitor::WorkItem,
  },
  type_aliases::{
    seen_set_iterative_type_visitor::SeenSet, type_function_type_id::TypeFunctionTypeId,
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
    type_function_type_variant::TypeFunctionTypeVariantMember,
  },
};

impl IterativeTypeFunctionTypeVisitor {
  fn cycle_type_function_type_id(&mut self, _ty: TypeFunctionTypeId) {
    // Empty implementation per source: void IterativeTypeFunctionTypeVisitor::cycle(TypeFunctionTypeId) {}
  }

  fn cycle_type_function_type_pack_id(&mut self, _tp: TypeFunctionTypePackId) {
    // Empty implementation per source: void IterativeTypeFunctionTypeVisitor::cycle(TypeFunctionTypePackId) {}
  }
}

impl IterativeTypeFunctionTypeVisitor {
  pub fn has_seen<T>(&mut self, tv: *const T) -> bool {
    if !self.visit_once {
      return false;
    }

    // C++ `bool isFresh = seen.insert(tv); return !isFresh;` — `Set::insert`
    // returns true when the element was newly inserted. The Rust `SeenSet`
    // (`DenseHashSet`) `insert` does not report freshness, so we probe
    // `contains` first to recover the same boolean.
    let key = VisitKeyRef::from_ptr(tv);
    let is_fresh = !self.seen.contains(&key);
    self.seen.insert(key);
    !is_fresh
  }
}

/// C++ 模板 `IterativeTypeFunctionTypeVisitor::isCyclic<TID>`
/// （IterativeTypeFunctionTypeVisitor.cpp:345-362）：沿 `parent` 链回溯，
/// 逐项 `*item == ty` 比对。enum 化后按变体内的 id 值直接比较指针。
impl IterativeTypeFunctionTypeVisitor {
  pub(crate) fn is_cyclic_type_id(&self, ty: TypeFunctionTypeId) -> bool {
    let mut cursor = self.work_cursor as i32;
    let mut item = &self.work_queue[self.work_cursor as usize];

    while item.parent() >= 0 {
      LUAU_ASSERT!(item.parent() < cursor);
      cursor = item.parent();
      item = &self.work_queue[cursor as usize];

      if matches!(item, WorkItem::Type(t, _) if *t == ty) {
        return true;
      }
    }

    false
  }

  pub(crate) fn is_cyclic_type_pack_id(&self, tp: TypeFunctionTypePackId) -> bool {
    let mut cursor = self.work_cursor as i32;
    let mut item = &self.work_queue[self.work_cursor as usize];

    while item.parent() >= 0 {
      LUAU_ASSERT!(item.parent() < cursor);
      cursor = item.parent();
      item = &self.work_queue[cursor as usize];

      if matches!(item, WorkItem::Pack(t, _) if *t == tp) {
        return true;
      }
    }

    false
  }
}

impl IterativeTypeFunctionTypeVisitor {
  pub fn iterative_type_function_type_visitor_string(visitor_name: String) -> Self {
    Self::iterative_type_function_type_visitor_string_seen_set_bool(
      visitor_name,
      SeenSet::new(VisitKeyRef::dense_default()),
      /*visitOnce*/ true,
    )
  }

  fn iterative_type_function_type_visitor_string_seen_set_bool(
    visitor_name: String,
    seen: SeenSet,
    visit_once: bool,
  ) -> Self {
    // Skip the first few doublings.  Almost all visits require less than 32 steps.
    let work_queue = Vec::with_capacity(32);

    IterativeTypeFunctionTypeVisitor {
      seen,
      work_queue,
      parent_cursor: -1,
      work_cursor: 0,
      visitor_name,
      visit_once,
    }
  }
}

impl IterativeTypeFunctionTypeVisitor {
  /// 表状节点（Table/Extern）共有的 props 子句柄入队：读类型先入队；
  /// In the case that the readType and the writeType are the same pointer, just traverse once.
  /// Traversing each property twice has pretty significant performance consequences.
  fn traverse_props(&mut self, props: &BTreeMap<String, TypeFunctionProperty>) {
    for prop in props.values() {
      if let Some(read_ty) = prop.read_ty {
        self.traverse_type_function_type_id(read_ty);
      }

      if let Some(write_ty) = prop.write_ty
        && !prop.is_shared()
      {
        self.traverse_type_function_type_id(write_ty);
      }
    }
  }

  /// 表状节点共有的 indexer 键/值类型入队。
  fn traverse_indexer(&mut self, indexer: &TypeFunctionTableIndexer) {
    self.traverse_type_function_type_id(indexer.key_type);
    self.traverse_type_function_type_id(indexer.value_type);
  }

  /// 对应 C++ `IterativeTypeFunctionTypeVisitor::process(TypeFunctionTypeId)`
  /// （`IterativeTypeFunctionTypeVisitor.cpp`）。
  ///
  /// 前置条件：`ty` 为经 `traverse_type_function_type_id` 入队的句柄，即指向
  /// `TypeFunctionRuntime` type bump arena 内存活 `TypeFunctionType` 节点的
  /// 指针（遍历期间 arena 不回收、不迁移节点）；空句柄与 C++ `get<T>()` 的
  /// 入口判空同义，走断言分支返回，全程不被解引用。各判别分支按 C++ 原顺序
  /// Primitive→Any→Unknown→Never→Singleton→Union→Intersection→Negation→
  /// Function→Table→Extern→Generic 逐一匹配，互斥变体下与原 12 次 `get<T>()`
  /// 逐臂求值的可观察结果一致。
  pub(crate) fn process_type_function_type_id(&mut self, ty: TypeFunctionTypeId) {
    if self.has_seen(ty) {
      return;
    }

    if ty.is_null() {
      // C++ 侧对空句柄由 `get<T>(tv)` 内部的 `LUAU_ASSERT(!tv.is_null())`
      // 拒绝并令各臂落空、最终命中 not-exhaustive 断言；此处合并为一次同义
      // 断言后直接走收尾 `unsee`。
      LUAU_ASSERT!(!ty.is_null());
    } else {
      let node = alias_ref(ty);

      if let Some(tfpt) = TypeFunctionPrimitiveType::get_if(&node.type_variant) {
        self.visit_type_function_type_id_type_function_primitive_type(ty, tfpt);
      } else if let Some(tfat) = TypeFunctionAnyType::get_if(&node.type_variant) {
        self.visit_type_function_type_id_type_function_any_type(ty, tfat);
      } else if let Some(tfut_unknown) = TypeFunctionUnknownType::get_if(&node.type_variant) {
        self.visit_type_function_type_id_type_function_unknown_type(ty, tfut_unknown);
      } else if let Some(tfnt_never) = TypeFunctionNeverType::get_if(&node.type_variant) {
        self.visit_type_function_type_id_type_function_never_type(ty, tfnt_never);
      } else if let Some(tfst) = TypeFunctionSingletonType::get_if(&node.type_variant) {
        self.visit_type_function_type_id_type_function_singleton_type(ty, tfst);
      } else if let Some(tfut_union) = TypeFunctionUnionType::get_if(&node.type_variant) {
        if self.visit_type_function_type_id_type_function_union_type(ty, tfut_union) {
          for &component in &tfut_union.components {
            self.traverse_type_function_type_id(component);
          }
        }
      } else if let Some(tfit) = TypeFunctionIntersectionType::get_if(&node.type_variant) {
        if self.visit_type_function_type_id_type_function_intersection_type(ty, tfit) {
          for &component in &tfit.components {
            self.traverse_type_function_type_id(component);
          }
        }
      } else if let Some(tfnt_negation) = TypeFunctionNegationType::get_if(&node.type_variant) {
        if self.visit_type_function_type_id_type_function_negation_type(ty, tfnt_negation) {
          let inner = tfnt_negation.type_id;
          self.traverse_type_function_type_id(inner);
        }
      } else if let Some(tfft) = TypeFunctionFunctionType::get_if(&node.type_variant) {
        if self.visit_type_function_type_id_type_function_function_type(ty, tfft) {
          for &generic in &tfft.generics {
            self.traverse_type_function_type_id(generic);
          }

          for &generic in &tfft.generic_packs {
            self.traverse_type_function_type_pack_id(generic);
          }

          let arg_types = tfft.arg_types;
          self.traverse_type_function_type_pack_id(arg_types);
          let ret_types = tfft.ret_types;
          self.traverse_type_function_type_pack_id(ret_types);
        }
      } else if let Some(tftt) = TypeFunctionTableType::get_if(&node.type_variant) {
        if self.visit_type_function_type_id_type_function_table_type(ty, tftt) {
          self.traverse_props(&tftt.props);

          if let Some(metatable) = tftt.metatable {
            self.traverse_type_function_type_id(metatable);
          }

          if let Some(indexer) = &tftt.indexer {
            self.traverse_indexer(indexer);
          }
        }
      } else if let Some(tfet) = TypeFunctionExternType::get_if(&node.type_variant) {
        if self.visit_type_function_type_id_type_function_extern_type(ty, tfet) {
          self.traverse_props(&tfet.props);

          if let Some(metatable) = tfet.metatable {
            self.traverse_type_function_type_id(metatable);
          }

          if let Some(read_parent) = tfet.read_parent {
            self.traverse_type_function_type_id(read_parent);
          }
          if let Some(write_parent) = tfet.write_parent {
            self.traverse_type_function_type_id(write_parent);
          }

          if let Some(indexer) = &tfet.indexer {
            self.traverse_indexer(indexer);
          }
        }
      } else if let Some(tfgt) = TypeFunctionGenericType::get_if(&node.type_variant) {
        self.visit_type_function_type_id_type_function_generic_type(ty, tfgt);
      } else {
        LUAU_ASSERT!(
          false /* "GenericTypeFunctionTypeVisitor::traverse(TypeFunctionTypeId) is not exhaustive!" */
        );
      }
    }

    self.unsee(ty);
  }

  /// 对应 C++ `IterativeTypeFunctionTypeVisitor::process(TypeFunctionTypePackId)`
  /// （`IterativeTypeFunctionTypeVisitor.cpp`）。
  ///
  /// 前置条件：`tp` 为经 `traverse_type_function_type_pack_id` 入队的句柄，
  /// 指向 `TypeFunctionRuntime` type-pack bump arena 内的存活
  /// `TypeFunctionTypePackVar` 节点（遍历期间地址稳定）；空句柄同 C++
  /// `get<T>()` 入口判空，走断言分支返回不被解引用。分支顺序与 C++ 一致：
  /// TypePack→Variadic→GenericTypePack。
  pub(crate) fn process_type_function_type_pack_id(&mut self, tp: TypeFunctionTypePackId) {
    if self.has_seen(tp) {
      return;
    }

    if tp.is_null() {
      // 与 C++ `get<T>(tp)` 入口断言同义：空句柄被拒绝，随后照旧收尾 `unsee`。
      LUAU_ASSERT!(!tp.is_null());
    } else {
      let node = alias_ref(tp);

      if let Some(tftp) = TypeFunctionTypePack::get_if(&node.type_variant) {
        if self.visit_type_function_type_pack_id_type_function_type_pack(tp, tftp) {
          for &ty in &tftp.head {
            self.traverse_type_function_type_id(ty);
          }

          if let Some(tail) = tftp.tail {
            self.traverse_type_function_type_pack_id(tail);
          }
        }
      } else if let Some(tfvtp) = TypeFunctionVariadicTypePack::get_if(&node.type_variant) {
        if self.visit_type_function_type_pack_id_type_function_variadic_type_pack(tp, tfvtp) {
          let inner = tfvtp.type_id;
          self.traverse_type_function_type_id(inner);
        }
      } else if let Some(tfgtv) = TypeFunctionGenericTypePack::get_if(&node.type_variant) {
        self.visit_type_function_type_pack_id_type_function_generic_type_pack(tp, tfgtv);
      } else {
        LUAU_ASSERT!(
          false /* "GenericTypeFunctionTypeVisitor::traverse(TypeFunctionTypePackId) is not exhaustive!" */
        );
      }
    }

    self.unsee(tp);
  }
}

impl IterativeTypeFunctionTypeVisitor {
  pub fn process_work_queue(&mut self) {
    // work_cursor 是持久游标（parent_cursor 依其取值），且处理中 work_queue 会增长，
    // 迭代器借用无法表达，保留游标遍历。
    while self.work_cursor < self.work_queue.len() as u32 {
      let item = &self.work_queue[self.work_cursor as usize];
      self.parent_cursor = self.work_cursor as i32;

      // 经判别式 Option 取值，unsafe 解引用收敛到 WorkItem 内部的指针转换。
      if let Some(ty) = item.type_function_type_id() {
        if self.is_cyclic_type_id(ty) {
          self.cycle_type_function_type_id(ty);
        } else {
          self.process_type_function_type_id(ty);
        }
      } else if let Some(tp) = item.type_function_type_pack_id() {
        if self.is_cyclic_type_pack_id(tp) {
          self.cycle_type_function_type_pack_id(tp);
        } else {
          self.process_type_function_type_pack_id(tp);
        }
      } else {
        ulua_common::LUAU_ASSERT!(false);
      }

      self.work_cursor += 1;
    }
  }
}

impl IterativeTypeFunctionTypeVisitor {
  pub fn run_type_function_type_id(&mut self, root_ty: TypeFunctionTypeId) {
    self.parent_cursor = -1;
    self.work_cursor = 0;
    self.work_queue.clear();
    self.traverse_type_function_type_id(root_ty);
    self.process_work_queue();
  }
}

impl IterativeTypeFunctionTypeVisitor {
  fn traverse_type_function_type_id(&mut self, ty: TypeFunctionTypeId) {
    self.work_queue.push(WorkItem::Type(ty, self.parent_cursor));
  }

  fn traverse_type_function_type_pack_id(&mut self, tp: TypeFunctionTypePackId) {
    self.work_queue.push(WorkItem::Pack(tp, self.parent_cursor));
  }
}

impl IterativeTypeFunctionTypeVisitor {
  pub fn unsee<T>(&mut self, _tv: *const T) {
    if !self.visit_once {
      // C++: `seen.erase(tv);`
      //
      // The C++ `SeenSet` is `Luau::Set` (DenseHashMap-backed, supports
      // erasure); here `SeenSet` is aliased to `DenseHashSet`, which has
      // no `erase`. This is sound regardless: `hasSeen` only inserts into
      // `seen` when `visitOnce` is true (it early-returns `false` before
      // any insert when `!visitOnce`), so in the `!visitOnce` branch the
      // set is always empty and the erase has nothing to remove. The
      // removal is therefore a no-op here, matching C++ behaviour.
    }
  }
}

/// 生成逐变体 visit 钩子的默认实现（cpp `GenericTypeFunctionTypeVisitor` 的
/// 虚函数默认值）：忽略变体负载、统一委托到裸 `visit(TypeFunctionTypeId)` /
/// `visit(TypeFunctionTypePackId)`，返回其布尔结果（true = 继续遍历子节点）。
macro_rules! default_visit_hooks {
  ($($name:ident($handle:ident : $handle_ty:ty, $payload:ident : $payload_ty:ty) -> $delegate:ident;)*) => {
    impl IterativeTypeFunctionTypeVisitor {
      $(
        pub fn $name(&mut self, $handle: $handle_ty, $payload: &$payload_ty) -> bool {
          let _ = $payload;
          self.$delegate($handle)
        }
      )*
    }
  };
}
default_visit_hooks! {
  visit_type_function_type_id_type_function_primitive_type(
    ty: TypeFunctionTypeId, tfpt: TypeFunctionPrimitiveType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_any_type(
    ty: TypeFunctionTypeId, tfat: TypeFunctionAnyType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_unknown_type(
    ty: TypeFunctionTypeId, tfut: TypeFunctionUnknownType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_never_type(
    ty: TypeFunctionTypeId, tfnt: TypeFunctionNeverType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_singleton_type(
    ty: TypeFunctionTypeId, tfst: TypeFunctionSingletonType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_union_type(
    ty: TypeFunctionTypeId, tfut: TypeFunctionUnionType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_intersection_type(
    ty: TypeFunctionTypeId, tfit: TypeFunctionIntersectionType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_negation_type(
    ty: TypeFunctionTypeId, tfnt: TypeFunctionNegationType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_function_type(
    ty: TypeFunctionTypeId, tfft: TypeFunctionFunctionType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_table_type(
    ty: TypeFunctionTypeId, tftt: TypeFunctionTableType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_extern_type(
    ty: TypeFunctionTypeId, tfet: TypeFunctionExternType
  ) -> visit_type_function_type_id;
  visit_type_function_type_id_type_function_generic_type(
    ty: TypeFunctionTypeId, tfgt: TypeFunctionGenericType
  ) -> visit_type_function_type_id;
  visit_type_function_type_pack_id_type_function_type_pack(
    tp: TypeFunctionTypePackId, tftp: TypeFunctionTypePack
  ) -> visit_type_function_type_pack_id;
  visit_type_function_type_pack_id_type_function_variadic_type_pack(
    tp: TypeFunctionTypePackId, tfvtp: TypeFunctionVariadicTypePack
  ) -> visit_type_function_type_pack_id;
  visit_type_function_type_pack_id_type_function_generic_type_pack(
    tp: TypeFunctionTypePackId, tfgtp: TypeFunctionGenericTypePack
  ) -> visit_type_function_type_pack_id;
}
impl IterativeTypeFunctionTypeVisitor {
  fn visit_type_function_type_pack_id(&mut self, _tp: TypeFunctionTypePackId) -> bool {
    true
  }
}
