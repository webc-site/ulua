use alloc::{collections::BTreeMap, string::String};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
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
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
    type_function_type_variant::TypeFunctionTypeVariantMember,
  },
};

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
    if self.has_seen(ty as *const ()) {
      return;
    }

    if ty.is_null() {
      // C++ 侧对空句柄由 `get<T>(tv)` 内部的 `LUAU_ASSERT(!tv.is_null())`
      // 拒绝并令各臂落空、最终命中 not-exhaustive 断言；此处合并为一次同义
      // 断言后直接走收尾 `unsee`。
      LUAU_ASSERT!(!ty.is_null());
    } else {
      // Safety: `ty` 非空（上方判空分支排除）且按前置条件对齐、指向
      // `TypeFunctionRuntime` bump arena 内的存活 `TypeFunctionType` 节点；
      // arena 在访问者生存期内不释放、不重定位内存。本函数及其调用的各
      // `visit_*` 仅持有节点的共享借用（`get_if` 派生的 `&T`），子节点只被
      // `traverse_*` 原样拷入指针入队（写入的是 `self.work_queue`），不回写
      // arena 节点，故该共享借用存续期间不存在并存的 `&mut` 借用。
      let node = unsafe { &*ty };

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

    self.unsee(ty as *const ());
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
    if self.has_seen(tp as *const ()) {
      return;
    }

    if tp.is_null() {
      // 与 C++ `get<T>(tp)` 入口断言同义：空句柄被拒绝，随后照旧收尾 `unsee`。
      LUAU_ASSERT!(!tp.is_null());
    } else {
      // Safety: `tp` 非空（上方判空分支排除）且按前置条件对齐、指向
      // `TypeFunctionRuntime` type-pack arena 内存活节点；arena 不回收、不
      // 迁移。各 `visit_*` 与 head/tail 子句柄的读取只产生共享借用，
      // `traverse_*` 仅把指针值拷入 `self.work_queue`，不回写 arena 节点，
      // 故此借用期间无并存的可变借用。
      let node = unsafe { &*tp };

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

    self.unsee(tp as *const ());
  }
}
