use ulua_common::fflag;

use crate::{
  functions::{follow_type, follow_type_pack, get_type, get_type_pack},
  records::{
    any_type::AnyType, arena_handle::alias, extern_type::ExternType, function_type::FunctionType,
    generic_type::GenericType, generic_type_pack::GenericTypePack,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, never_type::NeverType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, table_type::TableType,
    type_function_any_type::TypeFunctionAnyType, type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_property::TypeFunctionProperty, type_function_serializer::TypeFunctionSerializer,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_indexer::TypeFunctionTableIndexer,
    type_function_table_type::TypeFunctionTableType, type_function_type::TypeFunctionType,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_type_pack_var::TypeFunctionTypePackVar,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack, type_pack::TypePack,
    union_type::UnionType, unknown_type::UnknownType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    type_function_kind::TypeFunctionKind, type_function_type_id::TypeFunctionTypeId,
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariant,
    type_function_type_variant::TypeFunctionTypeVariant, type_id::TypeId, type_or_pack::TypeOrPack,
    type_pack_id::TypePackId,
  },
};

/// 目标节点（`TypeFunctionRuntime::type_arena` 中的槽位）变体的独占可写借用；
/// 空句柄以 `None` 表达，取代原「`is_null()` 提前返回」哨兵。
///
/// cpp 侧对应 `getMutable<TypeFunctionType>(tfti)->type_variant` 的 const_cast。
#[inline]
fn target_variant_mut(tfti: TypeFunctionTypeId) -> Option<&'static mut TypeFunctionTypeVariant> {
  (!tfti.is_null()).then(|| &mut alias(tfti as *mut TypeFunctionType).type_variant)
}

/// 类型包侧的同款收口，arena 为 `type_pack_arena`，契约同上。
#[inline]
fn target_pack_variant_mut(
  tftp: TypeFunctionTypePackId,
) -> Option<&'static mut TypeFunctionTypePackVariant> {
  (!tftp.is_null()).then(|| &mut alias(tftp as *mut TypeFunctionTypePackVar).type_variant)
}

impl TypeFunctionSerializer {
  /// cpp `serializeChildren(TypeId ty, TypeFunctionTypeId tfti)`：源取自模块
  /// 类型 arena（只读），目标为 runtime arena 里 `shallow_serialize` 新建的
  /// 槽位；源变体与目标变体一一配对后逐臂下发。
  pub fn serialize_children_type_id(&mut self, ty: TypeId, tfti: TypeFunctionTypeId) {
    let Some(target) = target_variant_mut(tfti) else {
      return;
    };

    let ty = follow_type::follow(ty);

    // 各 arm 对照 C++ `serializeChildren` 的 if-else 链。
    if let Some(source) = get_type::get::<PrimitiveType>(ty)
      && let TypeFunctionTypeVariant::Primitive(target) = target
    {
      self.serialize_children_primitive_type(source, target);
    } else if let Some(source) = get_type::get::<UnknownType>(ty)
      && let TypeFunctionTypeVariant::Unknown(target) = target
    {
      self.serialize_children_unknown_type(source, target);
    } else if let Some(source) = get_type::get::<NeverType>(ty)
      && let TypeFunctionTypeVariant::Never(target) = target
    {
      self.serialize_children_never_type(source, target);
    } else if let Some(source) = get_type::get::<AnyType>(ty)
      && let TypeFunctionTypeVariant::Any(target) = target
    {
      self.serialize_children_any_type(source, target);
    } else if let Some(source) = get_type::get::<SingletonType>(ty)
      && let TypeFunctionTypeVariant::Singleton(target) = target
    {
      self.serialize_children_singleton_type(source, target);
    } else if let Some(source) = get_type::get::<UnionType>(ty)
      && let TypeFunctionTypeVariant::Union(target) = target
    {
      self.serialize_children_union_type(source, target);
    } else if let Some(source) = get_type::get::<IntersectionType>(ty)
      && let TypeFunctionTypeVariant::Intersection(target) = target
    {
      self.serialize_children_intersection_type(source, target);
    } else if let Some(source) = get_type::get::<NegationType>(ty)
      && let TypeFunctionTypeVariant::Negation(target) = target
    {
      self.serialize_children_negation_type(source, target);
    } else if let Some(source) = get_type::get::<TableType>(ty)
      && let TypeFunctionTypeVariant::Table(target) = target
    {
      self.serialize_children_table_type(source, target);
    } else if let Some(source) = get_type::get::<MetatableType>(ty)
      && let TypeFunctionTypeVariant::Table(target) = target
    {
      self.serialize_children_metatable_type(source, target);
    } else if let Some(source) = get_type::get::<FunctionType>(ty)
      && let TypeFunctionTypeVariant::Function(target) = target
    {
      self.serialize_children_function_type(source, target);
    } else if let Some(source) = get_type::get::<ExternType>(ty)
      && let TypeFunctionTypeVariant::Extern(target) = target
    {
      self.serialize_children_extern_type(source, target);
    } else if let Some(source) = get_type::get::<GenericType>(ty)
      && let TypeFunctionTypeVariant::Generic(target) = target
    {
      self.serialize_children_generic_type(source, target);
    }
  }

  /// cpp `serializeChildren(TypePackId tp, TypeFunctionTypePackId tftp)`。
  pub(crate) fn serialize_children_type_pack_id(
    &mut self,
    tp: TypePackId,
    tftp: TypeFunctionTypePackId,
  ) {
    let Some(target) = target_pack_variant_mut(tftp) else {
      return;
    };

    let tp = follow_type_pack::follow(tp);

    if let Some(source) = get_type_pack::get::<TypePack>(tp)
      && let TypeFunctionTypePackVariant::V0(target) = target
    {
      self.serialize_children_type_pack(source, target);
    } else if let Some(source) = get_type_pack::get::<VariadicTypePack>(tp)
      && let TypeFunctionTypePackVariant::V1(target) = target
    {
      self.serialize_children_variadic_type_pack(source, target);
    } else if let Some(source) = get_type_pack::get::<GenericTypePack>(tp)
      && let TypeFunctionTypePackVariant::V2(target) = target
    {
      self.serialize_children_generic_type_pack(source, target);
    }
  }

  /// cpp `serializeChildren(TypeOrPack kind, TypeFunctionKind tfkind)`。
  pub fn serialize_children_kind(&mut self, kind: TypeOrPack, tfkind: TypeFunctionKind) {
    match (kind, tfkind) {
      (TypeOrPack::V0(ty), TypeFunctionKind::V0(tfti)) => {
        self.serialize_children_type_id(ty, tfti);
      }
      (TypeOrPack::V1(tp), TypeFunctionKind::V1(tftp)) => {
        self.serialize_children_type_pack_id(tp, tftp);
      }
      _ => {}
    }
  }

  pub fn serialize_children_primitive_type(
    &mut self,
    _source: &PrimitiveType,
    _target: &mut TypeFunctionPrimitiveType,
  ) {
  }

  pub fn serialize_children_unknown_type(
    &mut self,
    _source: &UnknownType,
    _target: &mut TypeFunctionUnknownType,
  ) {
  }

  pub fn serialize_children_never_type(
    &mut self,
    _source: &NeverType,
    _target: &mut TypeFunctionNeverType,
  ) {
  }

  pub fn serialize_children_any_type(
    &mut self,
    _source: &AnyType,
    _target: &mut TypeFunctionAnyType,
  ) {
  }

  pub fn serialize_children_singleton_type(
    &mut self,
    _source: &SingletonType,
    _target: &mut TypeFunctionSingletonType,
  ) {
  }

  /// cpp `serializeChildren(UnionType* u1, TypeFunctionUnionType* u2)`：源只读自
  /// 模块 arena，目标可写于 runtime arena，两 arena 地址不相交。
  pub fn serialize_children_union_type(
    &mut self,
    source: &UnionType,
    target: &mut TypeFunctionUnionType,
  ) {
    target.components.reserve(source.options.len());
    for &ty in &source.options {
      target.components.push(self.shallow_serialize_type_id(ty));
    }
  }

  pub fn serialize_children_intersection_type(
    &mut self,
    source: &IntersectionType,
    target: &mut TypeFunctionIntersectionType,
  ) {
    target.components.reserve(source.parts.len());
    for &ty in &source.parts {
      target.components.push(self.shallow_serialize_type_id(ty));
    }
  }

  pub fn serialize_children_negation_type(
    &mut self,
    source: &NegationType,
    target: &mut TypeFunctionNegationType,
  ) {
    target.type_id = self.shallow_serialize_type_id(source.ty);
  }

  pub fn serialize_children_table_type(
    &mut self,
    source: &TableType,
    target: &mut TypeFunctionTableType,
  ) {
    for (name, prop) in &source.props {
      let read_ty = prop
        .read_ty
        .map(|read_ty| self.shallow_serialize_type_id(read_ty));
      let write_ty = prop
        .write_ty
        .map(|write_ty| self.shallow_serialize_type_id(write_ty));

      target
        .props
        .insert(name.clone(), TypeFunctionProperty { read_ty, write_ty });
    }

    if let Some(indexer) = &source.indexer {
      let key_type = self.shallow_serialize_type_id(indexer.index_type);
      let value_type = self.shallow_serialize_type_id(indexer.index_result_type);
      target.indexer = Some(TypeFunctionTableIndexer::new(key_type, value_type));
    }
  }

  /// cpp `serializeChildren(MetatableType*, TypeFunctionTableType*)`：表体由
  /// `follow` 后的内层 `TableType` 复用表分支序列化，元表另记一句柄。
  pub fn serialize_children_metatable_type(
    &mut self,
    source: &MetatableType,
    target: &mut TypeFunctionTableType,
  ) {
    let table = follow_type::follow(source.table);

    if let Some(table) = get_type::get::<TableType>(table) {
      self.serialize_children_table_type(table, target);
    }

    target.metatable = Some(self.shallow_serialize_type_id(source.metatable));
  }

  pub fn serialize_children_function_type(
    &mut self,
    source: &FunctionType,
    target: &mut TypeFunctionFunctionType,
  ) {
    target.generics.reserve(source.generics.len());
    for &ty in &source.generics {
      let t = self.shallow_serialize_type_id(ty);
      target.generics.push(t);
    }
    target.generic_packs.reserve(source.generic_packs.len());
    for &tp in &source.generic_packs {
      let t = self.shallow_serialize_type_pack_id(tp);
      target.generic_packs.push(t);
    }
    target.arg_types = self.shallow_serialize_type_pack_id(source.arg_types);
    target.ret_types = self.shallow_serialize_type_pack_id(source.ret_types);

    if fflag::LuauTypeFunctionSerializeArgNames.get() {
      target.arg_names.reserve(source.arg_names.len());
      for arg_name in &source.arg_names {
        if let Some(arg_name) = arg_name {
          target.arg_names.push(Some(arg_name.name.clone()));
        } else {
          target.arg_names.push(None);
        }
      }
    }
  }

  pub fn serialize_children_extern_type(
    &mut self,
    source: &ExternType,
    target: &mut TypeFunctionExternType,
  ) {
    for (k, p) in &source.props {
      let read_ty = p
        .read_ty
        .as_ref()
        .map(|read_ty| self.shallow_serialize_type_id(*read_ty));

      let write_ty = p
        .write_ty
        .as_ref()
        .map(|write_ty| self.shallow_serialize_type_id(*write_ty));

      target
        .props
        .insert(k.clone(), TypeFunctionProperty { read_ty, write_ty });
    }

    if let Some(indexer) = &source.indexer {
      let key_type = self.shallow_serialize_type_id(indexer.index_type);
      let value_type = self.shallow_serialize_type_id(indexer.index_result_type);
      target.indexer = Some(TypeFunctionTableIndexer::new(key_type, value_type));
    }

    if let Some(metatable) = &source.metatable {
      target.metatable = Some(self.shallow_serialize_type_id(*metatable));
    }

    if let Some(parent) = &source.parent {
      let parent_ty = self.shallow_serialize_type_id(*parent);
      target.read_parent = Some(parent_ty);
      target.write_parent = Some(parent_ty);
    }
  }

  pub fn serialize_children_generic_type(
    &mut self,
    _source: &GenericType,
    _target: &mut TypeFunctionGenericType,
  ) {
  }

  pub fn serialize_children_type_pack(
    &mut self,
    source: &TypePack,
    target: &mut TypeFunctionTypePack,
  ) {
    for &ty in &source.head {
      target.head.push(self.shallow_serialize_type_id(ty));
    }
    if let Some(tail) = source.tail {
      target.tail = Some(self.shallow_serialize_type_pack_id(tail));
    }
  }

  pub fn serialize_children_variadic_type_pack(
    &mut self,
    source: &VariadicTypePack,
    target: &mut TypeFunctionVariadicTypePack,
  ) {
    target.type_id = self.shallow_serialize_type_id(source.ty);
  }

  pub fn serialize_children_generic_type_pack(
    &mut self,
    _source: &GenericTypePack,
    _target: &mut TypeFunctionGenericTypePack,
  ) {
  }
}
