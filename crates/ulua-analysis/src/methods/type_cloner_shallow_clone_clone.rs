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

impl TypeCloner {
  pub(crate) fn shallow_clone_type_id(&mut self, ty: TypeId) -> TypeId {
    // We want to [`Luau::follow`] but without forcing the expansion of [`LazyType`]s.
    let ty = follow_type::follow_with_option(ty, FollowOption::DisableLazyTypeThunks);

    if let Some(clone) = self.find_type_id(ty) {
      return clone;
    } else
    // persistent 标志读取收口在 `is_persistent`（arena 节点契约同 C++ get）。
    if is_persistent(ty) && ty != self.force_ty {
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

    // `replacement_for_null_scope` is null for ordinary clones (Clone.cpp:171-176,
    // free/table scope -> null) and carries the fragment cloner's fresh scope for
    // the `FragmentAutocompleteTypeCloner` override (Clone.cpp:508-513). Generic
    // types always get a null scope in both paths.
    if let Some(generic) = get_mutable_type::get_mutable::<GenericType>(target) {
      generic.scope = null_mut();
    } else if let Some(free) = get_mutable_type::get_mutable::<FreeType>(target) {
      free.scope = self.replacement_for_null_scope;
    } else if let Some(table) = get_mutable_type::get_mutable::<TableType>(target) {
      table.scope = self.replacement_for_null_scope;
    }

    // Safety: self.types 由 clone() 闭包从调用方 `tys: &mut HashMap` 借用接线为非空
    // 裸指针，HashMap 存活期覆盖整个 cloner 运行，且该 cloner 是唯一写入者；
    // ty/target 均为上方已确认的存活节点指针。
    unsafe { (*self.types).insert(ty, target) };
    self.queue.push(TypeOrPack::V0(target));
    target
  }

  pub(crate) fn shallow_clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let tp = follow_type_pack::follow(tp);

    if let Some(clone) = self.find_type_pack_id(tp) {
      return clone;
    } else
    // persistent 标志读取收口在 `pack_is_persistent`（arena 节点契约同 C++ get）。
    if pack_is_persistent(tp) && tp != self.force_tp {
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

    // null for ordinary clones (Clone.cpp:194-197), fresh scope for the
    // fragment cloner override (Clone.cpp:531-534). Generic packs always null.
    if let Some(generic) = get_mutable_type_pack::get_mutable::<GenericTypePack>(target) {
      generic.scope = null_mut();
    } else if let Some(free) = get_mutable_type_pack::get_mutable::<FreeTypePack>(target) {
      free.scope = self.replacement_for_null_scope;
    }

    // Safety: self.packs 由 clone() 闭包从调用方 `tps: &mut HashMap` 借用接线为非空
    // 裸指针，HashMap 存活期覆盖整个 cloner 运行且本 cloner 是唯一写入者；tp/target
    // 均为上方已确认的存活节点指针。
    unsafe { (*self.packs).insert(tp, target) };
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
