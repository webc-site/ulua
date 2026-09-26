use alloc::vec::Vec;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack},
  records::{
    any_type::AnyType, blocked_type::BlockedType, blocked_type_pack::BlockedTypePack,
    extern_type::ExternType, free_type::FreeType, free_type_pack::FreeTypePack,
    function_type::FunctionType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    intersection_type::IntersectionType, lazy_type::LazyType, metatable_type::MetatableType,
    negation_type::NegationType, never_type::NeverType, no_refine_type::NoRefineType,
    pending_expansion_type::PendingExpansionType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, table_type::TableType, type_cloner::TypeCloner,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    union_type::UnionType, unknown_type::UnknownType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    bound_type::BoundType,
    bound_type_pack::BoundTypePack,
    error_type::ErrorType,
    error_type_pack::ErrorTypePack,
    nominal_relation::NominalRelation,
    type_id::TypeId,
    type_or_pack::{TypeOrPack, TypeOrPackMember},
    type_pack_id::TypePackId,
    type_pack_variant::{TypePackVariant, TypePackVariantMember},
    type_variant::{TypeVariant, TypeVariantMember},
  },
};

/// 克隆节点变体的独占可写借用（cpp `asMutable(ty)->ty`）。
///
/// 句柄一律是 `TypeId`/`TypePackId` 索引句柄，负载改写以 `&mut` 形态下发，
/// 故 arena 解引用只在此处收口一次。
#[inline]
fn type_variant_mut(ty: TypeId) -> &'static mut TypeVariant {
  // SAFETY: `ty` 出自 `run()` 队列，即 `shallow_clone_type_id` 在目标 arena
  // （`self.arena`）`add_type` 新建节点的非空对齐句柄；arena 分配器按 chunk
  // 分配，克隆游程内不搬运、不释放既有节点。`run()` 由 `&mut self` 串行推进，
  // 且 `types` seen 表去重保证同一节点至多执行一次 cloneChildren（见
  // `type_cloner_run.rs`），各臂内的 `shallow_clone_*` 只追加新节点、不改写
  // 既有负载，故本借用是该节点变体的唯一活动借用。
  unsafe { &mut (*as_mutable_type_id(ty)).ty }
}

/// `TypePackVariant` 侧的同款收口，契约与 [`type_variant_mut`] 对称
/// （`packs` seen 表去重）。
#[inline]
fn type_pack_variant_mut(tp: TypePackId) -> &'static mut TypePackVariant {
  // SAFETY: 同上——`tp` 由 `shallow_clone_type_pack_id` 于目标 arena
  // `add_type_pack_*` 新建，非空对齐且地址稳定；本游程内该节点变体仅此一次
  // 独占可写借用。
  unsafe { &mut (*as_mutable_type_pack(tp)).ty }
}

impl TypeCloner {
  /// cpp `void TypeCloner::cloneChildren(TypeId ty)`（`Clone.cpp:225-234`）。
  ///
  /// C++：`visit([&](auto&& t){ return cloneChildren(&t); }, asMutable(ty)->ty);`
  /// —— 按 `TypeVariant` 分派到具体负载的可变引用。
  pub fn clone_children_type_id(&mut self, ty: TypeId) {
    let tv = type_variant_mut(ty);
    match tv {
      // `Bound<TypeId>` 负载在变体里以裸 `TypeId` 存放，其 repr(transparent)
      // 下转已收口于 `TypeVariantMember::get_if_mut`，此处直接拿安全借用。
      TypeVariant::Bound(_) => {
        if let Some(inner) = BoundType::get_if_mut(tv) {
          self.clone_children_bound_type(inner);
        }
      }
      TypeVariant::Error(inner) => self.clone_children_error_type(inner),
      TypeVariant::Free(inner) => self.clone_children_free_type(inner),
      TypeVariant::Generic(inner) => self.clone_children_generic_type(inner),
      TypeVariant::Primitive(inner) => self.clone_children_primitive_type(inner),
      TypeVariant::Singleton(inner) => self.clone_children_singleton_type(inner),
      TypeVariant::Blocked(inner) => self.clone_children_blocked_type(inner),
      TypeVariant::PendingExpansion(inner) => self.clone_children_pending_expansion_type(inner),
      TypeVariant::Function(inner) => self.clone_children_function_type(inner),
      TypeVariant::Table(inner) => self.clone_children_table_type(inner),
      TypeVariant::Metatable(inner) => self.clone_children_metatable_type(inner),
      TypeVariant::Extern(inner) => self.clone_children_extern_type(inner),
      TypeVariant::Any(inner) => self.clone_children_any_type(inner),
      TypeVariant::Union(inner) => self.clone_children_union_type(inner),
      TypeVariant::Intersection(inner) => self.clone_children_intersection_type(inner),
      TypeVariant::Lazy(inner) => self.clone_children_lazy_type(inner),
      TypeVariant::Unknown(inner) => self.clone_children_unknown_type(inner),
      TypeVariant::Never(inner) => self.clone_children_never_type(inner),
      TypeVariant::Negation(inner) => self.clone_children_negation_type(inner),
      TypeVariant::NoRefine(inner) => self.clone_children_no_refine_type(inner),
      TypeVariant::TypeFunctionInstance(inner) => {
        self.clone_children_type_function_instance_type(inner)
      }
    }
  }

  pub fn clone_children_blocked_type_pack(&mut self, _t: &mut BlockedTypePack) {
    // TODO: In the new solver, we should ice.
  }

  /// cpp `cloneChildren(BoundTypePack&)`：改写别名目标。
  pub fn clone_children_bound_type_pack(&mut self, t: &mut BoundTypePack) {
    t.bound_to = self.shallow_clone_type_pack_id(t.bound_to);
  }

  pub fn clone_children_error_type_pack(&mut self, _t: &mut ErrorTypePack) {
    // noop.
  }

  pub fn clone_children_variadic_type_pack(&mut self, t: &mut VariadicTypePack) {
    t.ty = self.shallow_clone_type_id(t.ty);
  }

  pub fn clone_children_type_pack(&mut self, t: &mut TypePack) {
    for ty in t.head.iter_mut() {
      *ty = self.shallow_clone_type_id(*ty);
    }
    if let Some(tail) = t.tail {
      t.tail = Some(self.shallow_clone_type_pack_id(tail));
    }
  }

  pub fn clone_children_type_function_instance_type_pack(
    &mut self,
    t: &mut TypeFunctionInstanceTypePack,
  ) {
    for ty in t.type_arguments.iter_mut() {
      *ty = self.shallow_clone_type_id(*ty);
    }
    for tp in t.pack_arguments.iter_mut() {
      *tp = self.shallow_clone_type_pack_id(*tp);
    }
  }

  /// cpp `void TypeCloner::cloneChildren(TypePackId tp)`（`Clone.cpp:236-245`）。
  ///
  /// C++：`visit([&](auto&& t){ return cloneChildren(&t); }, asMutable(tp)->ty);`
  pub fn clone_children_type_pack_id(&mut self, tp: TypePackId) {
    let tv = type_pack_variant_mut(tp);
    match tv {
      // 同上：`Bound<TypePackId>` 的 repr(transparent) 下转收口在 rtti 侧。
      TypePackVariant::Bound(_) => {
        if let Some(inner) = BoundTypePack::get_if_mut(tv) {
          self.clone_children_bound_type_pack(inner);
        }
      }
      TypePackVariant::Error(inner) => self.clone_children_error_type_pack(inner),
      TypePackVariant::Free(inner) => self.clone_children_free_type_pack(inner),
      TypePackVariant::Generic(inner) => self.clone_children_generic_type_pack(inner),
      TypePackVariant::TypePack(inner) => self.clone_children_type_pack(inner),
      TypePackVariant::Variadic(inner) => self.clone_children_variadic_type_pack(inner),
      TypePackVariant::Blocked(inner) => self.clone_children_blocked_type_pack(inner),
      TypePackVariant::TypeFunctionInstance(inner) => {
        self.clone_children_type_function_instance_type_pack(inner)
      }
    }
  }

  pub fn clone_children_type_or_pack(&mut self, kind: TypeOrPack) {
    if let Some(ty) = TypeId::get_if(&kind) {
      self.clone_children_type_id(*ty);
    } else if let Some(tp) = TypePackId::get_if(&kind) {
      self.clone_children_type_pack_id(*tp);
    } else {
      LUAU_ASSERT!(false);
    }
  }

  pub fn clone_children_error_type(&mut self, _t: &mut ErrorType) {
    // noop
  }

  pub fn clone_children_bound_type(&mut self, t: &mut BoundType) {
    t.bound_to = self.shallow_clone_type_id(t.bound_to);
  }

  /// `lower_bound`/`upper_bound` 以 null 表示「无界」（镜像 cpp
  /// `std::optional`，字段形态属 arena 记录，本波不改）。
  pub fn clone_children_free_type(&mut self, t: &mut FreeType) {
    if !t.lower_bound.is_null() {
      t.lower_bound = self.shallow_clone_type_id(t.lower_bound);
    }
    if !t.upper_bound.is_null() {
      t.upper_bound = self.shallow_clone_type_id(t.upper_bound);
    }
  }

  pub fn clone_children_generic_type(&mut self, _t: &mut GenericType) {
    // TODO: clone upper bounds.
  }

  pub fn clone_children_primitive_type(&mut self, _t: &mut PrimitiveType) {
    // noop.
  }

  pub fn clone_children_blocked_type(&mut self, _t: &mut BlockedType) {
    // TODO: In the new solver, we should ice.
  }

  pub fn clone_children_pending_expansion_type(&mut self, _t: &mut PendingExpansionType) {
    // TODO: In the new solver, we should ice.
  }

  pub fn clone_children_singleton_type(&mut self, _t: &mut SingletonType) {
    // noop
  }

  pub fn clone_children_function_type(&mut self, t: &mut FunctionType) {
    for g in &mut t.generics {
      *g = self.shallow_clone_type_id(*g);
    }
    for gp in &mut t.generic_packs {
      *gp = self.shallow_clone_type_pack_id(*gp);
    }
    t.arg_types = self.shallow_clone_type_pack_id(t.arg_types);
    t.ret_types = self.shallow_clone_type_pack_id(t.ret_types);
  }

  pub fn clone_children_table_type(&mut self, t: &mut TableType) {
    if let Some(indexer) = &mut t.indexer {
      indexer.index_type = self.shallow_clone_type_id(indexer.index_type);
      indexer.index_result_type = self.shallow_clone_type_id(indexer.index_result_type);
    }

    // `for (auto& [_, p] : t->props) p = shallowClone(p);`
    // Take ownership of each property, clone it, and write it back. We
    // collect the keys first because `shallow_clone_type_id` borrows
    // `self` mutably while we would otherwise hold a borrow on `props`.
    let keys: Vec<_> = t.props.keys().cloned().collect();
    for key in keys {
      let p = t.props[&key].clone();
      let cloned = self.shallow_clone_property(&p);
      t.props.insert(key, cloned);
    }

    for ty in t.instantiated_type_params.iter_mut() {
      *ty = self.shallow_clone_type_id(*ty);
    }

    for tp in t.instantiated_type_pack_params.iter_mut() {
      *tp = self.shallow_clone_type_pack_id(*tp);
    }
  }

  pub fn clone_children_metatable_type(&mut self, t: &mut MetatableType) {
    t.table = self.shallow_clone_type_id(t.table);
    t.metatable = self.shallow_clone_type_id(t.metatable);
  }

  /// `extern_ty` 指向源侧 builtin 类型，有意不克隆也不解引用。
  pub fn clone_children_extern_type(&mut self, t: &mut ExternType) {
    // `for (auto& [_, p] : t->props) p = shallowClone(p);`
    let keys: Vec<_> = t.props.keys().cloned().collect();
    for key in keys {
      let p = t.props[&key].clone();
      let cloned = self.shallow_clone_property(&p);
      t.props.insert(key, cloned);
    }

    if let Some(parent) = t.parent {
      t.parent = Some(self.shallow_clone_type_id(parent));
    }

    if let Some(metatable) = t.metatable {
      t.metatable = Some(self.shallow_clone_type_id(metatable));
    }

    if let Some(indexer) = &mut t.indexer {
      indexer.index_type = self.shallow_clone_type_id(indexer.index_type);
      indexer.index_result_type = self.shallow_clone_type_id(indexer.index_result_type);
    }

    if fflag::DebugLuauUserDefinedClasses.get()
      && let Some(relation) = &mut t.relation
    {
      match relation {
        NominalRelation::V0(obj) => {
          obj.ty = self.shallow_clone_type_id(obj.ty);
        }
        NominalRelation::V1(klass) => {
          klass.ty = self.shallow_clone_type_id(klass.ty);
        }
      }
    }
  }

  pub fn clone_children_any_type(&mut self, _t: &mut AnyType) {
    // noop.
  }

  pub fn clone_children_no_refine_type(&mut self, _t: &mut NoRefineType) {
    // noop.
  }

  pub fn clone_children_union_type(&mut self, t: &mut UnionType) {
    for ty in &mut t.options {
      *ty = self.shallow_clone_type_id(*ty);
    }
  }

  pub fn clone_children_intersection_type(&mut self, t: &mut IntersectionType) {
    for ty in t.parts.iter_mut() {
      *ty = self.shallow_clone_type_id(*ty);
    }
  }

  /// `unwrapped` 以 null 表示「尚未解包」（cpp 侧为
  /// `std::atomic<TypeId>`，本端口按单线程游程读写）。
  pub fn clone_children_lazy_type(&mut self, t: &mut LazyType) {
    // The `FragmentAutocompleteTypeCloner` override (Clone.cpp:541-544) does
    // not clone lazy types: it overrides `cloneChildren(LazyType*)` to a no-op.
    if self.skip_lazy_type_clone {
      return;
    }

    let unwrapped = t.unwrapped;
    if !unwrapped.is_null() {
      t.unwrapped = self.shallow_clone_type_id(unwrapped);
    }
  }

  pub fn clone_children_unknown_type(&mut self, _t: &mut UnknownType) {
    // noop.
  }

  pub fn clone_children_never_type(&mut self, _t: &mut NeverType) {
    // noop
  }

  pub fn clone_children_negation_type(&mut self, t: &mut NegationType) {
    t.ty = self.shallow_clone_type_id(t.ty);
  }

  /// `type_function` 本体（跨边界的类型函数定义）有意不克隆。
  pub fn clone_children_type_function_instance_type(&mut self, t: &mut TypeFunctionInstanceType) {
    for ty in t.type_arguments.iter_mut() {
      *ty = self.shallow_clone_type_id(*ty);
    }
    for tp in t.pack_arguments.iter_mut() {
      *tp = self.shallow_clone_type_pack_id(*tp);
    }
  }

  pub fn clone_children_free_type_pack(&mut self, _t: &mut FreeTypePack) {
    // TODO: clone lower and upper bounds.
    // TODO: In the new solver, we should ice.
  }

  pub fn clone_children_generic_type_pack(&mut self, _t: &mut GenericTypePack) {
    // TODO: clone upper bounds.
  }
}
