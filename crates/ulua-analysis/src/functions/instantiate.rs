//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Instantiation.cpp:197:instantiate`
//! Source: `Analysis/src/Instantiation.cpp` (Instantiation.cpp:197-257, hand-ported)

use core::ptr::null;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  FFlag,
  enums::polarity::Polarity,
  functions::{
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id, fresh_type::fresh_type,
    get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
    get_type_pack::get_type_pack_id, shallow_clone_clone_alt_b::shallow_clone,
  },
  records::{
    builtin_types::BuiltinTypes, clone_state::CloneState, function_type::FunctionType,
    generic_type::GenericType, generic_type_pack::GenericTypePack, replacer::Replacer,
    scope::Scope, type_arena::TypeArena, type_check_limits::TypeCheckLimits,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
pub fn instantiate(
  builtin_types: &BuiltinTypes,
  arena: &mut TypeArena,
  limits: &TypeCheckLimits,
  scope: *mut Scope,
  ty: TypeId,
) -> Option<TypeId> {
  // ty = follow(ty);
  let ty = follow_type_id(ty);

  // const FunctionType* ft = get<FunctionType>(ty);
  // if (!ft) return ty;
  let Some(ft) = get_type_id::<FunctionType>(ty) else {
    return Some(ty);
  };

  // if (ft->generics.empty() && ft->genericPacks.empty()) return ty;
  if ft.generics.is_empty() && ft.generic_packs.is_empty() {
    return Some(ty);
  }

  // DenseHashMap<TypeId, TypeId> replacements{nullptr};
  // DenseHashMap<TypePackId, TypePackId> replacementPacks{nullptr};
  let mut replacements: DenseHashMap<TypeId, TypeId> = DenseHashMap::new(null());
  let mut replacement_packs: DenseHashMap<TypePackId, TypePackId> = DenseHashMap::new(null());

  if FFlag::LuauInstantiationUsesPolarity.get() {
    // for (TypeId g : ft->generics)
    //     if (auto r#gen = get<GenericType>(follow(g)))
    //         replacements[g] = freshType(arena, builtinTypes, scope, r#gen->polarity);
    for &g in &ft.generics {
      if let Some(r#gen) = get_type_id::<GenericType>(follow_type_id(g)) {
        *replacements.get_or_insert(g) = fresh_type(arena, builtin_types, scope, r#gen.polarity);
      }
    }

    // for (TypePackId g : ft->genericPacks)
    //     if (auto r#gen = get<GenericTypePack>(follow(g)))
    //         replacementPacks[g] = arena->freshTypePack(scope, r#gen->polarity);
    for &g in &ft.generic_packs {
      if let Some(r#gen) = get_type_pack_id::<GenericTypePack>(unsafe { follow_type_pack_id(g) }) {
        *replacement_packs.get_or_insert(g) = arena.fresh_type_pack(scope, r#gen.polarity);
      }
    }
  } else {
    // for (TypeId g : ft->generics)
    //     replacements[g] = freshType(arena, builtinTypes, scope);
    for &g in &ft.generics {
      *replacements.get_or_insert(g) = fresh_type(arena, builtin_types, scope, Polarity::None);
    }

    // for (TypePackId g : ft->genericPacks)
    //     replacementPacks[g] = arena->freshTypePack(scope);
    for &g in &ft.generic_packs {
      *replacement_packs.get_or_insert(g) = arena.fresh_type_pack(scope, Polarity::None);
    }
  }

  // Replacer r{arena, NotNull{&replacements}, NotNull{&replacementPacks}};
  let mut r = Replacer::new(arena, &mut replacements, &mut replacement_packs);

  // if (limits->instantiation_child_limit)
  //     r.childLimit = *limits->instantiation_child_limit;
  if let Some(child_limit) = limits.instantiation_child_limit {
    r.base.base.child_limit = child_limit;
  }

  // CloneState cs{builtinTypes};
  // SAFETY: CloneState 以裸指针持有 builtin_types（crate 惯例），此处仅读
  let mut cs = CloneState {
    builtin_types: builtin_types as *const BuiltinTypes as *mut BuiltinTypes,
    seen_types: DenseHashMap::new(null()),
    seen_type_packs: DenseHashMap::new(null()),
  };

  // auto clonedFunctionTypeId = shallowClone(ty, *arena, cs, /* clonePersistentTypes */ true);
  let cloned_function_type_id = unsafe { shallow_clone(ty, arena, &mut cs, true) };

  // FunctionType* ft2 = get_mutable<FunctionType>(clonedFunctionTypeId);
  // C++: shallowClone 克隆了 FunctionType，get_mutable 必命中
  let ft2 = get_mutable_type_id::<FunctionType>(cloned_function_type_id).unwrap();

  // ft2->generics.clear();
  // ft2->genericPacks.clear();
  ft2.generics.clear();
  ft2.generic_packs.clear();

  // return r.substitute(clonedFunctionTypeId);
  r.substitute_type_id(cloned_function_type_id)
}
