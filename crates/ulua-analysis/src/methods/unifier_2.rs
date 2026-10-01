//! `unifier_2` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::ptr::NonNull;

use crate::{
  enums::polarity::Polarity,
  functions::{
    follow_type, fresh_type::fresh_type, get_mutable_type_pack,
    simplify_intersection_simplify::simplify_intersection, simplify_union::simplify_union,
  },
  records::{
    arena_handle::{Handle, alias},
    free_type_pack::FreeTypePack,
    replacer::Replacer,
    scope::Scope,
    unifier_2::Unifier2,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Unifier2 {
  pub fn fresh_type(&mut self, scope: NonNull<Scope>, polarity: Polarity) -> TypeId {
    // Safety: arena/builtin_types 为构造期 NonNull 接线的存活分配；`scope`
    // 同为 NonNull<Scope>（cpp NotNull 直译），`as_ref` 只读共享借用。
    let result = fresh_type(
      // Safety: 见上——重建 &mut 仅覆盖本次调用（只追加 FreeType 节点）。
      unsafe { self.arena.as_mut() },
      unsafe { self.builtin_types.as_ref() },
      Some(unsafe { scope.as_ref() }),
      polarity,
    );
    self.new_fresh_types.push(result);
    result
  }
}

impl Unifier2 {
  pub fn fresh_type_pack(&mut self, scope: NonNull<Scope>, polarity: Polarity) -> TypePackId {
    let result = alias(self.arena.as_ptr()).fresh_type_pack(scope.as_ptr(), polarity);

    // C++ Unifier2.cpp:958 LUAU_ASSERT(ftp)：result 刚由 arena freshTypePack 分配，必为 FreeTypePack
    if let Some(ftp) = get_mutable_type_pack::get_mutable::<FreeTypePack>(result) {
      ftp.polarity = polarity;
    }

    self.new_fresh_type_packs.push(result);
    result
  }
}

impl Unifier2 {
  pub fn instantiate_with_bound_types(&mut self, ty: TypeId) -> TypeId {
    let mut r = Replacer::new(
      Handle::from_nonnull(self.arena),
      NonNull::from(&mut self.generic_substitutions).as_ptr(),
      NonNull::from(&mut self.generic_pack_substitutions).as_ptr(),
    );
    if let Some(new_ty) = r.substitute_type_id(ty) {
      return new_ty;
    }
    ty
  }
}

impl Unifier2 {
  pub fn mk_intersection(&mut self, left: TypeId, right: TypeId) -> TypeId {
    let left = follow_type::follow(left);
    let right = follow_type::follow(right);
    simplify_intersection(
      Handle::from_nonnull(self.builtin_types),
      Handle::from_nonnull(self.arena),
      left,
      right,
    )
    .result
  }
}

impl Unifier2 {
  pub fn mk_union(&mut self, left: TypeId, right: TypeId) -> TypeId {
    let left = follow_type::follow(left);
    let right = follow_type::follow(right);
    simplify_union(
      Handle::from_nonnull(self.builtin_types),
      Handle::from_nonnull(self.arena),
      left,
      right,
    )
    .result
  }
}
