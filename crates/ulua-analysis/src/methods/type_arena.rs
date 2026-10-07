//! `type_arena` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::ptr::null_mut;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::variant::Variant2};

use crate::{
  enums::polarity::Polarity,
  functions::{as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack},
  records::{
    arena_handle::alias, builtin_types::BuiltinTypes, free_type::FreeType,
    free_type_pack::FreeTypePack, scope::Scope, singleton_type::SingletonType, r#type::Type,
    type_arena::TypeArena, type_function::TypeFunction,
    type_function_instance_type::TypeFunctionInstanceType, type_level::TypeLevel,
    type_pack::TypePack, type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_variant::TypeVariant},
};

impl TypeArena {
  pub fn add_tv(&mut self, tv: Type) -> TypeId {
    let allocated = self.types.allocate(tv);
    alias(as_mutable_type_id(allocated)).owning_arena = self.arena_id;
    allocated
  }
}

impl TypeArena {
  pub fn add_type<T>(&mut self, tv: T) -> TypeId
  where
    T: Into<Type> + 'static,
  {
    let tv_into: Type = tv.into();

    if let TypeVariant::Union(union_type) = &tv_into.ty {
      LUAU_ASSERT!(union_type.options.len() >= 2);
    }

    if let TypeVariant::Singleton(singleton_type) = &tv_into.ty
      && self.collect_singleton_stats
    {
      self.record_singleton_stats(singleton_type);
    }

    self.add_tv(tv_into)
  }
}

impl TypeArena {
  pub fn add_type_function_type_function_initializer_list_type_id(
    &mut self,
    function: &TypeFunction,
    types: &[TypeId],
  ) -> TypeId {
    let pack_arguments = Vec::new();
    self.add_type_function_type_function_vector_type_id_vector_type_pack_id(
      function,
      types.to_vec(),
      pack_arguments,
    )
  }

  pub fn add_type_function_type_function_vector_type_id_vector_type_pack_id(
    &mut self,
    function: &TypeFunction,
    type_arguments: Vec<TypeId>,
    pack_arguments: Vec<TypePackId>,
  ) -> TypeId {
    self.add_type(TypeFunctionInstanceType::new_with_pack_args(
      function,
      type_arguments,
      pack_arguments,
    ))
  }
}

impl TypeArena {
  pub fn add_type_pack_t<T>(&mut self, tp: T) -> TypePackId
  where
    T: Into<TypePackVar>,
  {
    self.add_type_pack_type_pack_var(tp.into())
  }

  pub fn add_type_pack_initializer_list_type_id(&mut self, types: &[TypeId]) -> TypePackId {
    let tp = TypePack::from_vec(types.to_vec());
    let allocated = self.type_packs.allocate(TypePackVar::from(tp));
    alias(as_mutable_type_pack(allocated)).owning_arena = self.arena_id;
    allocated
  }

  pub fn add_type_pack_vector_type_id_optional_type_pack_id(
    &mut self,
    types: Vec<TypeId>,
    tail: Option<TypePackId>,
  ) -> TypePackId {
    let tp = TypePack::new(types, tail);
    // 安全等价：转发到 safe 的 add_type_pack_type_pack_var（其内部完成
    // allocate + owning_arena 接线的同一证成路径），本方法不再需要 unsafe。
    self.add_type_pack_type_pack_var(TypePackVar::from(tp))
  }

  pub fn add_type_pack_type_pack(&mut self, tp: TypePackVar) -> TypePackId {
    // 安全等价：与 add_type_pack_type_pack_var 逐行同体（allocate + 写 owning_arena），
    // 收敛到同一 safe 实现，消除重复 unsafe 站点。
    self.add_type_pack_type_pack_var(tp)
  }

  pub fn add_type_pack_type_pack_var(&mut self, tp: TypePackVar) -> TypePackId {
    let allocated = self.type_packs.allocate(tp);
    alias(as_mutable_type_pack(allocated)).owning_arena = self.arena_id;
    allocated
  }
}

impl TypeArena {
  pub fn clear(&mut self) {
    self.types.clear();
    self.type_packs.clear();
  }
}

impl TypeArena {
  pub fn fresh_type_pack(&mut self, scope: *mut Scope, polarity: Polarity) -> TypePackId {
    // FreeTypePack{scope, polarity}
    let mut free = FreeTypePack {
      index: 0,
      level: TypeLevel::default(),
      scope: null_mut(),
      polarity: Polarity::None,
    };
    free.free_type_pack_scope_polarity(scope, polarity);

    let allocated = self.type_packs.allocate(TypePackVar::from(free));
    alias(as_mutable_type_pack(allocated)).owning_arena = self.arena_id;
    allocated
  }
}

impl TypeArena {
  pub fn fresh_type_not_null_builtin_types_type_level(
    &mut self,
    builtins: &BuiltinTypes,
    level: TypeLevel,
  ) -> TypeId {
    self.add_type(FreeType {
      level,
      lower_bound: builtins.never_type,
      upper_bound: builtins.unknown_type,
      ..FreeType::default()
    })
  }

  pub fn fresh_type_not_null_builtin_types_scope(
    &mut self,
    builtins: &BuiltinTypes,
    scope: *mut Scope,
  ) -> TypeId {
    self.add_type(FreeType {
      scope,
      lower_bound: builtins.never_type,
      upper_bound: builtins.unknown_type,
      ..FreeType::default()
    })
  }
}

impl TypeArena {
  pub fn record_singleton_stats(&mut self, singleton: &SingletonType) {
    match &singleton.variant {
      Variant2::V0(_bool_singleton) => {
        self.bool_singletons_minted += 1;
      }
      Variant2::V1(str_singleton) => {
        self.str_singletons_minted += 1;
        if !str_singleton.value.is_empty() {
          self
            .unique_str_singletons_minted
            .insert(Some(str_singleton.value.clone()));
        }
      }
    }
  }
}
