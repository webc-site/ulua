use core::ptr::null_mut;

use crate::{
  enums::follow_option::FollowOption,
  functions::{
    follow_type, follow_type_pack, get_mutable_type, get_mutable_type_pack,
    get_type::{documentation_symbol_of, is_persistent, type_variant_of},
    get_type_pack::{pack_is_persistent, type_pack_variant_of},
  },
  records::{
    arena_id::ArenaId, free_type::FreeType, free_type_pack::FreeTypePack,
    generic_type::GenericType, generic_type_pack::GenericTypePack, property_type::Property,
    table_type::TableType, r#type::Type, type_cloner::TypeCloner, type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId},
};

impl TypeCloner<'_> {
  pub(crate) fn shallow_clone_type_id(&mut self, ty: TypeId) -> TypeId {
    // We want to [`Luau::follow`] but without forcing the expansion of [`LazyType`]s.
    let ty = follow_type::follow_with_option(ty, FollowOption::DisableLazyTypeThunks);

    if let Some(clone) = self.find_type_id(ty) {
      return clone;
    } else
    // persistent 标志读取收口在 `is_persistent`（arena 节点契约同 C++ get）。
    if is_persistent(ty) && self.force_ty != Some(ty) {
      return ty;
    }

    // 变体克隆收口在 `type_variant_of`（arena 节点契约同 C++ get）；
    // `self.arena` 是 `Handle`，`get_mut` 为项目认可的最小内部可变边界。
    // documentation_symbol 在入 arena 前于本地值上写入（与 C++ Clone 复制
    // ty->documentationSymbol 同语义），源节点 symbol 读取收口在
    // `documentation_symbol_of`，全程无裸指针解引用。
    let variant = type_variant_of(ty).clone();
    let mut new_type = Type::new(variant);
    new_type.documentation_symbol = documentation_symbol_of(ty);
    let target = self.arena.get_mut().add_type(new_type);

    // 普通克隆无替换 scope（Clone.cpp:171-176，free/table scope -> null）；
    // `FragmentAutocompleteTypeCloner` 重载（Clone.cpp:508-513）带上其 fresh scope。
    // Generic 类型两路恒为 null。折算裸指针仅发生在写入契约字段处。
    let replacement_scope = self
      .replacement_for_null_scope
      .map_or(null_mut(), |scope| scope.as_ptr());
    if let Some(generic) = get_mutable_type::get_mutable::<GenericType>(target) {
      generic.scope = null_mut();
    } else if let Some(free) = get_mutable_type::get_mutable::<FreeType>(target) {
      free.scope = replacement_scope;
    } else if let Some(table) = get_mutable_type::get_mutable::<TableType>(target) {
      table.scope = replacement_scope;
    }

    self.types.insert(ty, target);
    self.queue.push(TypeOrPack::V0(target));
    target
  }

  pub(crate) fn shallow_clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let tp = follow_type_pack::follow(tp);

    if let Some(clone) = self.find_type_pack_id(tp) {
      return clone;
    } else
    // persistent 标志读取收口在 `pack_is_persistent`（arena 节点契约同 C++ get）。
    if pack_is_persistent(tp) && self.force_tp != Some(tp) {
      return tp;
    }

    // `self.arena` 为 `Handle`（get_mut 系最小内部可变边界）；new 节点的
    // owning_arena 为 ArenaId::NONE 与 C++ 新节点默认（nullptr）一致。
    // 变体克隆收口在 `type_pack_variant_of`（arena 节点契约同 C++ get）。
    let variant = type_pack_variant_of(tp).clone();
    let target = self
      .arena
      .get_mut()
      .add_type_pack_type_pack_var(TypePackVar {
        ty: variant,
        persistent: false,
        owning_arena: ArenaId::NONE,
      });

    // 普通克隆为 null（Clone.cpp:194-197），fragment 重载带上 fresh scope
    // （Clone.cpp:531-534）。Generic pack 恒 null。
    let replacement_scope = self
      .replacement_for_null_scope
      .map_or(null_mut(), |scope| scope.as_ptr());
    if let Some(generic) = get_mutable_type_pack::get_mutable::<GenericTypePack>(target) {
      generic.scope = null_mut();
    } else if let Some(free) = get_mutable_type_pack::get_mutable::<FreeTypePack>(target) {
      free.scope = replacement_scope;
    }

    self.packs.insert(tp, target);
    self.queue.push(TypeOrPack::V1(target));
    target
  }

  pub fn shallow_clone_property(&mut self, p: &Property) -> Property {
    let mut clone_read_ty: Option<TypeId> = None;
    if let Some(ty) = p.read_ty {
      clone_read_ty = Some(self.shallow_clone_type_id(ty));
    }

    let mut clone_write_ty: Option<TypeId> = None;
    if let Some(ty) = p.write_ty {
      clone_write_ty = Some(self.shallow_clone_type_id(ty));
    }

    let mut cloned = Property::create(clone_read_ty, clone_write_ty);
    cloned.deprecated = p.deprecated;
    cloned.deprecated_suggestion = p.deprecated_suggestion.clone();
    cloned.location = p.location;
    cloned.tags = p.tags.clone();
    cloned.documentation_symbol = p.documentation_symbol.clone();
    cloned.type_location = p.type_location;
    cloned
  }
}
