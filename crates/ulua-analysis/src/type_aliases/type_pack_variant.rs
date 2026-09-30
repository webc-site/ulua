use crate::{
  macros::variant_member,
  records::{
    blocked_type_pack::BlockedTypePack, free_type_pack::FreeTypePack,
    generic_type_pack::GenericTypePack,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    unifiable::Error, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{bound_type_pack::BoundTypePack, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub enum TypePackVariant {
  Bound(TypePackId),
  Error(Error<TypePackId>),
  Free(FreeTypePack),
  Generic(GenericTypePack),
  TypePack(TypePack),
  Variadic(VariadicTypePack),
  Blocked(BlockedTypePack),
  TypeFunctionInstance(TypeFunctionInstanceTypePack),
}

// `Bound` 由下方 `BoundTypePack` 的手工实现承接（槽位存裸 `TypePackId`），不在表内。
variant_member! {
  /// Mirror of `TypeVariantMember` for packs: the Rust shape of C++
  /// `get_if<T>(&TypePackVariant)`.
  enum TypePackVariantMember: TypePackVariant {
    Error => Error<TypePackId>,
    Free => FreeTypePack,
    Generic => GenericTypePack,
    TypePack => TypePack,
    Variadic => VariadicTypePack,
    Blocked => BlockedTypePack,
    TypeFunctionInstance => TypeFunctionInstanceTypePack,
  }
}

/// BoundTypePack = Bound<TypePackId>; the Bound alternative stores the bare id.
impl TypePackVariantMember for BoundTypePack {
  fn get_if(v: &TypePackVariant) -> Option<&Self> {
    match v {
      TypePackVariant::Bound(inner) => {
        // Safety: 唯一命中臂保证 `inner` 借用自 `v` 内存活的 `Bound(TypePackId)`
        // 字段；`Bound<Id>` 为 `#[repr(transparent)]` 单字段结构（`bound_to: Id`），
        // 与 `TypePackId` 布局相同、基址重合，转写为 `&BoundTypePack` 不改变
        // 借用的生命周期与别名集。
        Some(unsafe { &*(inner as *const TypePackId as *const Self) })
      }
      _ => None,
    }
  }
  fn get_if_mut(v: &mut TypePackVariant) -> Option<&mut Self> {
    match v {
      TypePackVariant::Bound(inner) => {
        // Safety: 同上——repr(transparent) 保证与 `TypePackId` 基址重合；此处
        // `&mut` 借自入参 `v` 的独占借用，命中臂即动态类型确认，无并发别名。
        Some(unsafe { &mut *(inner as *mut TypePackId as *mut Self) })
      }
      _ => None,
    }
  }
}
