use alloc::{
  collections::BTreeSet,
  format,
  string::{String, ToString},
};
use core::ptr::null_mut;

use ulua_ast::records::location::Location;
use ulua_common::fflag;

use crate::{
  enums::polarity::Polarity,
  functions::{
    get_mutable_type,
    get_mutable_type_function_runtime::{
      get_mutable_type_function_type_id, get_mutable_type_function_type_pack_id,
    },
    get_mutable_type_pack,
    get_type_function_runtime::{get_type_function_type_id, get_type_function_type_pack_id},
  },
  records::{
    any_type::AnyType, extern_type::ExternType, function_argument::FunctionArgument,
    function_type::FunctionType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, never_type::NeverType, primitive_type::PrimitiveType,
    property_type::Property, serialized_function_scope::SerializedFunctionScope,
    serialized_generic::SerializedGeneric, singleton_type::SingletonType,
    table_indexer::TableIndexer, table_type::TableType, r#type::Type,
    type_function_any_type::TypeFunctionAnyType,
    type_function_deserializer::TypeFunctionDeserializer,
    type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType, type_function_type::TypeFunctionType,
    type_function_type_pack::TypeFunctionTypePack, type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack, type_level::TypeLevel,
    type_pack::TypePack, type_pack_var::TypePackVar, union_type::UnionType,
    unknown_type::UnknownType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    type_function_kind::{TypeFunctionKind, TypeFunctionKindMember},
    type_function_type_id::TypeFunctionTypeId,
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
    type_function_type_variant::{TypeFunctionTypeVariant, TypeFunctionTypeVariantMember},
    type_id::TypeId,
    type_or_pack::{TypeOrPack, TypeOrPackMember},
    type_pack_id::TypePackId,
  },
};

/// runtime arena（`type_arena`）里的**源**负载：cpp `getMutable<TypeFunctionT>(tfti)`
/// 在本端口只读不写，故折成共享引用；变体不符时得 `None`，与原先「裸指针判空」
/// 同义。arena 解引用收口在被包装的 `unsafe fn` 内。
#[inline]
fn tf_source<T: TypeFunctionTypeVariantMember>(tfti: TypeFunctionTypeId) -> Option<&'static T> {
  // SAFETY: 本函数契约由 `deserialize_children_type_id` 保证——`tfti` 出自反序列化
  // 队列，即 runtime `type_arena`（chunked `TypedAllocator`，元素地址不再搬移）内
  // 存活节点的句柄；helper 只在非空时按 tag 探测一次，命中即返回该槽位内变体字段
  // 地址，与节点同寿命，且整个游程内源侧只读（写句柄都在目标 arena 一侧）。
  unsafe {
    get_mutable_type_function_type_id::<T>(tfti)
      .cast_const()
      .as_ref()
  }
}

/// 类型包侧的 [`tf_source`]，arena 为 `type_pack_arena`。
#[inline]
fn tf_pack_source<T: TypeFunctionTypePackVariantMember>(
  tftp: TypeFunctionTypePackId,
) -> Option<&'static T> {
  // SAFETY: 同 `tf_source`，句柄指向 runtime `type_pack_arena` 存活节点。
  unsafe {
    get_mutable_type_function_type_pack_id::<T>(tftp)
      .cast_const()
      .as_ref()
  }
}

/// 需要以可变句柄交给 `SerializedFunctionScope` 的源负载（cpp 侧同一
/// `getMutable`），契约同 [`tf_source`]。
#[inline]
fn tf_source_mut<T: TypeFunctionTypeVariantMember>(
  tfti: TypeFunctionTypeId,
) -> Option<&'static mut T> {
  // SAFETY: `tfti` 为 runtime `type_arena` 存活节点句柄；本轮反序列化由
  // `&mut self` 串行驱动，本步骤内该变体字段无其他在册借用（同源的其他臂按
  // tag 互斥，最多一条臂命中）。
  unsafe { get_mutable_type_function_type_id::<T>(tfti).as_mut() }
}

/// runtime 侧只读探测：cpp `get<TypeFunctionT>(ty)`，空句柄以 `None` 表达。
#[inline]
fn tf_read<T: TypeFunctionTypeVariantMember>(ty: TypeFunctionTypeId) -> Option<&'static T> {
  // SAFETY: `ty` 是队列里 `TypeFunctionTypeId`（`*const TypeFunctionType`）句柄，
  // 由 runtime bump arena 分配、本轮存活；helper 内部已先判空，未命中返回 null。
  unsafe { get_type_function_type_id::<T>(ty).as_ref() }
}

/// 类型包侧的 [`tf_read`]。
#[inline]
fn tf_pack_read<T: TypeFunctionTypePackVariantMember>(
  tp: TypeFunctionTypePackId,
) -> Option<&'static T> {
  // SAFETY: 同 `tf_read`，句柄指向 runtime `type_pack_arena` 存活节点。
  unsafe { get_type_function_type_pack_id::<T>(tp).as_ref() }
}

impl TypeFunctionDeserializer {
  /// 经 builder 注入的 `state->ctx.ice` 上报 ICE（cpp `ice(...)`）。
  fn report_ice(&mut self, message: &str) {
    // SAFETY: `self.state` 由 builder 入口 `type_function_deserializer()` 注入，
    // 整轮反序列化期间于 builder 栈上存活；`(*self.state).ctx` 为 Handle 字段（类型编码
    // 非空），物化后指向调用方持有的 `TypeFunctionContext`，其 `ice` 亦是 NonNull 报告器。本行
    // 只走上报路径，不触碰任何 arena 内存。
    unsafe { (*self.state).ctx.get().ice.as_ref().ice_string(message) };
  }

  /// cpp `deserializeChildren(TypeFunctionTypeId tfti, TypeId ty)`
  /// （`TypeFunctionRuntimeBuilder.cpp:870-902`）：源为 runtime arena 的类型函数
  /// 负载（只读），目标为反序列化所在 `TypeArena` 的槽位（可写），二者按同一
  /// 变体配对后逐臂下发。
  pub fn deserialize_children_type_id(&mut self, tfti: TypeFunctionTypeId, ty: TypeId) {
    // 各 arm 对照 C++ `if (auto [x1, x2] = tuple{...}; x1 && x2)` 链。
    if let (Some(t1), Some(t2)) = (
      get_mutable_type::get_mutable::<PrimitiveType>(ty),
      tf_source::<TypeFunctionPrimitiveType>(tfti),
    ) {
      self.deserialize_children_primitive_type(t2, t1);
      return;
    }

    if let (Some(t1), Some(t2)) = (
      get_mutable_type::get_mutable::<UnknownType>(ty),
      tf_source::<TypeFunctionUnknownType>(tfti),
    ) {
      self.deserialize_children_unknown_type(t2, t1);
      return;
    }

    if let (Some(t1), Some(t2)) = (
      get_mutable_type::get_mutable::<NeverType>(ty),
      tf_source::<TypeFunctionNeverType>(tfti),
    ) {
      self.deserialize_children_never_type(t2, t1);
      return;
    }

    if let (Some(t1), Some(t2)) = (
      get_mutable_type::get_mutable::<AnyType>(ty),
      tf_source::<TypeFunctionAnyType>(tfti),
    ) {
      self.deserialize_children_any_type(t2, t1);
      return;
    }

    if let (Some(t1), Some(t2)) = (
      get_mutable_type::get_mutable::<SingletonType>(ty),
      tf_source::<TypeFunctionSingletonType>(tfti),
    ) {
      self.deserialize_children_singleton_type(t2, t1);
      return;
    }

    if let (Some(t1), Some(t2)) = (
      get_mutable_type::get_mutable::<UnionType>(ty),
      tf_source::<TypeFunctionUnionType>(tfti),
    ) {
      self.deserialize_children_union_type(t2, t1);
      return;
    }

    if let (Some(t1), Some(t2)) = (
      get_mutable_type::get_mutable::<IntersectionType>(ty),
      tf_source::<TypeFunctionIntersectionType>(tfti),
    ) {
      self.deserialize_children_intersection_type(t2, t1);
      return;
    }

    if let (Some(t1), Some(t2)) = (
      get_mutable_type::get_mutable::<NegationType>(ty),
      tf_source::<TypeFunctionNegationType>(tfti),
    ) {
      self.deserialize_children_negation_type(t2, t1);
      return;
    }

    // C++: `t1 && t2 && !t2->metatable.has_value()`
    if let (Some(t1), Some(t2)) = (
      get_mutable_type::get_mutable::<TableType>(ty),
      tf_source::<TypeFunctionTableType>(tfti),
    ) && t2.metatable.is_none()
    {
      self.deserialize_children_table_type(t2, t1);
      return;
    }

    // C++: `m1 && m2 && m2->metatable.has_value()`
    if let (Some(m1), Some(m2)) = (
      get_mutable_type::get_mutable::<MetatableType>(ty),
      tf_source::<TypeFunctionTableType>(tfti),
    ) && m2.metatable.is_some()
    {
      self.deserialize_children_table_type_metatable_type(m2, m1);
      return;
    }

    if let (Some(t1), Some(t2)) = (
      get_mutable_type::get_mutable::<FunctionType>(ty),
      tf_source_mut::<TypeFunctionFunctionType>(tfti),
    ) {
      self.deserialize_children_function_type(t2, t1);
      return;
    }

    if let (Some(t1), Some(t2)) = (
      get_mutable_type::get_mutable::<ExternType>(ty),
      tf_source::<TypeFunctionExternType>(tfti),
    ) {
      self.deserialize_children_extern_type(t2, t1);
      return;
    }

    if let (Some(t1), Some(t2)) = (
      get_mutable_type::get_mutable::<GenericType>(ty),
      tf_source::<TypeFunctionGenericType>(tfti),
    ) {
      self.deserialize_children_generic_type(t2, t1);
      return;
    }

    self.report_ice(
      "Deserializing user defined type function arguments: mysterious type is being deserialized",
    );
  }

  /// cpp `deserializeChildren(TypeFunctionTypePackId tftp, TypePackId tp)`。
  pub fn deserialize_children_type_pack_id(
    &mut self,
    tftp: TypeFunctionTypePackId,
    tp: TypePackId,
  ) {
    // 各 arm 对照 C++ `if (auto [x1, x2] = tuple{...}; x1 && x2)` 链。
    if let (Some(t1), Some(t2)) = (
      get_mutable_type_pack::get_mutable::<TypePack>(tp),
      tf_pack_source::<TypeFunctionTypePack>(tftp),
    ) {
      self.deserialize_children_type_pack(t2, t1);
      return;
    }

    if let (Some(v1), Some(v2)) = (
      get_mutable_type_pack::get_mutable::<VariadicTypePack>(tp),
      tf_pack_source::<TypeFunctionVariadicTypePack>(tftp),
    ) {
      self.deserialize_children_variadic_type_pack(v2, v1);
      return;
    }

    if let (Some(g1), Some(g2)) = (
      get_mutable_type_pack::get_mutable::<GenericTypePack>(tp),
      tf_pack_source::<TypeFunctionGenericTypePack>(tftp),
    ) {
      self.deserialize_children_generic_type_pack(g2, g1);
      return;
    }

    self.report_ice(
      "Deserializing user defined type function arguments: mysterious type is being deserialized",
    );
  }

  /// cpp `deserializeChildren(TypeFunctionKind tfkind, TypeOrPack kind)`。
  pub fn deserialize_children_kind(&mut self, tfkind: TypeFunctionKind, kind: TypeOrPack) {
    if let Some(ty) = <TypeId as TypeOrPackMember>::get_if(&kind)
      && let Some(tfty) = <TypeFunctionTypeId as TypeFunctionKindMember>::get_if(&tfkind)
    {
      self.deserialize_children_type_id(*tfty, *ty);
      return;
    }

    if let Some(tp) = <TypePackId as TypeOrPackMember>::get_if(&kind)
      && let Some(tftp) = <TypeFunctionTypePackId as TypeFunctionKindMember>::get_if(&tfkind)
    {
      self.deserialize_children_type_pack_id(*tftp, *tp);
      return;
    }

    self.report_ice(
      "Deserializing user defined type function arguments: tfkind and kind do not represent the same type",
    );
  }

  pub fn deserialize_children_primitive_type(
    &mut self,
    _source: &TypeFunctionPrimitiveType,
    _target: &mut PrimitiveType,
  ) {
  }

  pub fn deserialize_children_unknown_type(
    &mut self,
    _source: &TypeFunctionUnknownType,
    _target: &mut UnknownType,
  ) {
  }

  pub fn deserialize_children_never_type(
    &mut self,
    _source: &TypeFunctionNeverType,
    _target: &mut NeverType,
  ) {
  }

  pub fn deserialize_children_any_type(
    &mut self,
    _source: &TypeFunctionAnyType,
    _target: &mut AnyType,
  ) {
    // noop.
  }

  pub fn deserialize_children_singleton_type(
    &mut self,
    _source: &TypeFunctionSingletonType,
    _target: &mut SingletonType,
  ) {
    // noop.
  }

  /// cpp `deserializeChildren(TypeFunctionUnionType* u2, UnionType* u1)`：读 runtime
  /// 侧 `components`、写目标 arena 侧 `options`，两 arena 内存不相交。
  pub fn deserialize_children_union_type(
    &mut self,
    source: &TypeFunctionUnionType,
    target: &mut UnionType,
  ) {
    // shallow_deserialize 只在目标 chunked arena 追加新 Type 并写 seen 表，既有
    // 元素地址不搬移，故遍历源 `components` 与写入目标 `options` 全程不别名。
    for ty in &source.components {
      let ty_id = self.shallow_deserialize_type_function_type_id(*ty);
      target.options.push(ty_id);
    }
  }

  pub fn deserialize_children_intersection_type(
    &mut self,
    source: &TypeFunctionIntersectionType,
    target: &mut IntersectionType,
  ) {
    for ty in &source.components {
      let ty_id = self.shallow_deserialize_type_function_type_id(*ty);
      target.parts.push(ty_id);
    }
  }

  pub fn deserialize_children_negation_type(
    &mut self,
    source: &TypeFunctionNegationType,
    target: &mut NegationType,
  ) {
    // 右值先于赋值求值：浅反序列化只追加目标 arena，不搬移 `target`。
    target.ty = self.shallow_deserialize_type_function_type_id(source.type_id);
  }

  pub fn deserialize_children_table_type(
    &mut self,
    source: &TypeFunctionTableType,
    target: &mut TableType,
  ) {
    for (k, p) in &source.props {
      let read_ty = p
        .read_ty
        .map(|ty| self.shallow_deserialize_type_function_type_id(ty));
      let write_ty = p
        .write_ty
        .map(|ty| self.shallow_deserialize_type_function_type_id(ty));

      let prop = match (read_ty, write_ty) {
        (Some(r), Some(w)) => Property::rw_type_id_type_id(r, w),
        (Some(r), None) => Property::readonly(r),
        (None, Some(w)) => Property::writeonly(w),
        (None, None) => Property::new(),
      };
      target.props.insert(k.clone(), prop);
    }

    if let Some(indexer) = &source.indexer {
      let key_ty = self.shallow_deserialize_type_function_type_id(indexer.key_type);
      let val_ty = self.shallow_deserialize_type_function_type_id(indexer.value_type);
      target.indexer = Some(TableIndexer {
        index_type: key_ty,
        index_result_type: val_ty,
        is_read_only: false,
      });
    }
  }

  /// cpp `deserializeChildren(TypeFunctionTableType* m2, MetatableType* m1)`：表体
  /// 先以「去掉元表的临时 TypeFunctionTableType」入 arena 再反序列化，元表另记。
  pub fn deserialize_children_table_type_metatable_type(
    &mut self,
    source: &TypeFunctionTableType,
    target: &mut MetatableType,
  ) {
    // `TypeFunctionTableType{m2->props, m2->indexer}` — the 2-arg constructor
    // (metatable defaults to std::nullopt).
    let table = TypeFunctionTableType {
      props: source.props.clone(),
      indexer: source.indexer.clone(),
      metatable: None,
    };
    // SAFETY: `self.type_function_runtime` 由构造器从 `state->ctx` 的 NonNull 字段
    // 捕获（即待反序列化 runtime 本体），整轮期间存活；`type_arena.allocate` 是
    // chunked 分配器追加新元素，既不搬移 `source` 指向的既有节点，`temp` 也是刚
    // 写入的新槽位。
    let temp: TypeFunctionTypeId = unsafe {
      (*self.type_function_runtime)
        .type_arena
        .allocate(TypeFunctionType::new(TypeFunctionTypeVariant::Table(table)))
    };

    target.table = self.shallow_deserialize_type_function_type_id(temp);

    if let Some(metatable) = source.metatable {
      target.metatable = self.shallow_deserialize_type_function_type_id(metatable);
    }
  }

  /// cpp `deserializeChildren(TypeFunctionFunctionType* f2, FunctionType* f1)`：
  /// 先把源节点压入 `function_scopes`（队列回到本帧水位时由
  /// `close_function_scope` 弹出泛型登记），再把泛型参数引入作用域并逐项反序列化。
  ///
  /// 源侧取 `&mut`：`SerializedFunctionScope::function` 是 runtime arena 的可变
  /// 槽位句柄（cpp 侧同为 `getMutable` 的结果），本函数自身只读它。
  pub fn deserialize_children_function_type(
    &mut self,
    source: &mut TypeFunctionFunctionType,
    target: &mut FunctionType,
  ) {
    // SAFETY: `self.state` 由 builder 入口注入并在本轮反序列化期间于 builder
    // 栈上存活；`(*self.state).ctx` 为 Handle 字段（类型编码非空），物化后指向调用方持有的
    // `TypeFunctionContext`，其 `arena`/`scope` 亦为 NonNull，指向的
    // `TypeArena`/`Scope` 由调用方持有到反序列化结束。本块只把两个 NonNull 折回
    // 原生句柄，不解引用空值；其后 `add_tv`/`add_type_pack_t` 只向目标 arena
    // 追加、不搬移既有元素，故 `source` 与循环里派生的 `gty`/`gtp` 引用全程有效。
    // deserializer 自有的 queue/generic_types 等 Vec 与 arena 内存无关，借用互不冲突。
    let (arena, scope) = unsafe {
      (
        (*self.state).ctx.get().arena.as_ptr(),
        (*self.state).ctx.get().scope.as_ptr(),
      )
    };
    // SAFETY: 依上，`arena` 指向调用方持有、本轮存活的 `TypeArena`；本函数内只以
    // 它调用 `add_tv`/`add_type_pack_t`（bump 追加，不搬移既有元素），且该 arena
    // 与 `self`（deserializer 自有状态）不重叠，故可变借用无在册别名。
    let arena = unsafe { &mut *arena };

    self.function_scopes.push(SerializedFunctionScope {
      old_queue_size: self.queue.len(),
      function: source as *mut _,
    });
    let mut generic_names: BTreeSet<(bool, String)> = BTreeSet::new();

    // Introduce generic function parameters into scope
    for &ty in &source.generics {
      let Some(gty) = tf_read::<TypeFunctionGenericType>(ty) else {
        self.push_runtime_error("Encountered unexpected generic".to_string());
        return;
      };
      if gty.is_pack() {
        self.push_runtime_error("Encountered unexpected generic".to_string());
        return;
      }

      let name_key = (gty.is_named(), gty.name().to_string());

      // Duplicates are not allowed
      if generic_names.contains(&name_key) {
        self.push_runtime_error(format!("Duplicate type parameter '{}'", gty.name()));
        return;
      }

      generic_names.insert(name_key);

      let mapping = if gty.is_named() {
        arena.add_tv(Type::from(GenericType::generic_type_scope_name(
          scope,
          &gty.name().to_string(),
        )))
      } else {
        arena.add_tv(Type::from(GenericType::new()))
      };
      self.generic_types.push(SerializedGeneric {
        is_named: gty.is_named(),
        name: gty.name().to_string(),
        r#type: mapping,
      });
    }

    for &tp in &source.generic_packs {
      let Some(gtp) = tf_pack_read::<TypeFunctionGenericTypePack>(tp) else {
        self.push_runtime_error("Encountered unexpected generic type pack".to_string());
        return;
      };

      let name_key = (gtp.is_named(), gtp.name().to_string());

      // Duplicates are not allowed
      if generic_names.contains(&name_key) {
        self.push_runtime_error(format!("Duplicate type parameter '{}'", gtp.name()));
        return;
      }

      generic_names.insert(name_key);

      let mapping = if gtp.is_named() {
        let r#gen = GenericTypePack {
          index: 0,
          level: TypeLevel::default(),
          scope,
          name: gtp.name().to_string(),
          explicit_name: true,
          polarity: Polarity::Unknown,
        };
        arena.add_type_pack_t(TypePackVar::from(r#gen))
      } else {
        let mut r#gen = GenericTypePack {
          index: 0,
          level: TypeLevel::default(),
          scope: null_mut(),
          name: String::new(),
          explicit_name: false,
          polarity: Polarity::Unknown,
        };
        r#gen.generic_type_pack();
        arena.add_type_pack_t(TypePackVar::from(r#gen))
      };
      self.generic_packs.push(SerializedGeneric {
        is_named: gtp.is_named(),
        name: gtp.name().to_string(),
        r#type: mapping,
      });
    }

    target.generics.reserve(source.generics.len());
    for &ty in &source.generics {
      let g = self.shallow_deserialize_type_function_type_id(ty);
      target.generics.push(g);
    }

    target.generic_packs.reserve(source.generic_packs.len());
    for &tp in &source.generic_packs {
      // SAFETY: `shallow_deserialize_type_function_type_pack_id` 的前提是
      // `self.state`/`state->ctx` 构造期接线非空且本轮存活（本函数入口已按同一
      // 契约取用），与入参句柄无关；它只向目标 arena 追加槽位。
      let g = unsafe { self.shallow_deserialize_type_function_type_pack_id(tp) };
      target.generic_packs.push(g);
    }

    if !source.arg_types.is_null() {
      // SAFETY: 同上——state 链条有效，浅反序列化只在目标 arena 追加。
      target.arg_types =
        unsafe { self.shallow_deserialize_type_function_type_pack_id(source.arg_types) };
    }

    if !source.ret_types.is_null() {
      // SAFETY: 同上。
      target.ret_types =
        unsafe { self.shallow_deserialize_type_function_type_pack_id(source.ret_types) };
    }

    if fflag::LuauTypeFunctionSerializeArgNames.get() {
      target.arg_names.reserve(source.arg_names.len());
      for name in &source.arg_names {
        if let Some(name) = name {
          target.arg_names.push(Some(FunctionArgument {
            name: name.clone(),
            location: Location::default(),
          }));
        } else {
          target.arg_names.push(None);
        }
      }
    }
  }

  pub fn deserialize_children_extern_type(
    &mut self,
    _source: &TypeFunctionExternType,
    _target: &mut ExternType,
  ) {
  }

  pub fn deserialize_children_generic_type(
    &mut self,
    _source: &TypeFunctionGenericType,
    _target: &mut GenericType,
  ) {
  }

  pub fn deserialize_children_type_pack(
    &mut self,
    source: &TypeFunctionTypePack,
    target: &mut TypePack,
  ) {
    // `source` 位于 runtime chunked arena，浅反序列化只追加目标 arena 与 seen 表，
    // 不搬移该区域，故遍历期间的只读借用与对 `target` 的写入互不别名。
    for &ty in &source.head {
      let deserialized = self.shallow_deserialize_type_function_type_id(ty);
      target.head.push(deserialized);
    }

    if let Some(tail) = source.tail {
      // SAFETY: `shallow_deserialize_type_function_type_pack_id` 只要求
      // `self.state`/`state->ctx` 链条非空存活（本轮 builder 已保证）。
      target.tail = Some(unsafe { self.shallow_deserialize_type_function_type_pack_id(tail) });
    }
  }

  pub fn deserialize_children_variadic_type_pack(
    &mut self,
    source: &TypeFunctionVariadicTypePack,
    target: &mut VariadicTypePack,
  ) {
    let deserialized = self.shallow_deserialize_type_function_type_id(source.type_id);
    target.ty = deserialized;
  }

  pub fn deserialize_children_generic_type_pack(
    &mut self,
    _source: &TypeFunctionGenericTypePack,
    _target: &mut GenericTypePack,
  ) {
  }
}
