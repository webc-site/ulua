use crate::{
  enums::{pack_field::PackField, type_field::TypeField},
  records::{
    generic_pack_mapping::GenericPackMapping, generic_type_pack::GenericTypePack, nothing::Nothing,
    path::Path, scope::Scope, subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{component::Component, lookup_result::LookupResult, type_pack_id::TypePackId},
};

impl Subtyping {
  pub fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_variadic_type_pack_type_pack_id_variadic_type_pack(
    &mut self,
    env: &mut SubtypingEnvironment,
    scope: &Scope,
    _sub_tp: TypePackId,
    sub: &VariadicTypePack,
    _super_tp: TypePackId,
    super_variadic: &VariadicTypePack,
  ) -> SubtypingResult {
    self
      .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub.ty,
        super_variadic.ty,
        scope,
      )
      .with_both_component(Component::TypeField(TypeField::Variadic))
      .with_both_component(Component::PackField(PackField::Tail))
      .to_owned()
  }

  /// # Safety 说明
  /// 形参全为受检类型/引用：`scope` 为非空 `&Scope`（NotNull<Scope> 的类型化等价），
  /// `sub_tp`/`super_tp` 为 arena 包句柄，`env` 为调用方独占借用——契约由类型承载。
  pub(crate) fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_type_pack_id_generic_type_pack(
    &mut self,
    env: &mut SubtypingEnvironment,
    scope: &Scope,
    sub_tp: TypePackId,
    _sub: &GenericTypePack,
    super_tp: TypePackId,
    _super: &GenericTypePack,
  ) -> SubtypingResult {
    let sub_lookup_result = env.lookup_generic_pack(sub_tp);
    let super_lookup_result = env.lookup_generic_pack(super_tp);

    match sub_lookup_result {
      LookupResult::V0(curr_mapping) => {
        let mut result = self
          .is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
            env,
            curr_mapping,
            super_tp,
            scope,
          );
        result.with_sub_path(Path::from_components(alloc::vec![
          Component::PackField(PackField::Tail),
          Component::GenericPackMapping(GenericPackMapping {
            mapped_type: curr_mapping,
          }),
        ]));
        result.with_super_component(Component::PackField(PackField::Tail));
        result
      }
      LookupResult::V1(_) => {
        let ok = env
          .current_mut()
          .mapped_generic_packs
          .bind_generic(sub_tp, super_tp);
        let mut result = SubtypingResult::uncacheable(ok);
        result.with_both_component(Component::PackField(PackField::Tail));
        result
      }
      LookupResult::V2(_) => match super_lookup_result {
        LookupResult::V0(curr_mapping) => {
          let mut result = self
            .is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
              env,
              sub_tp,
              curr_mapping,
              scope,
            );
          result.with_sub_component(Component::PackField(PackField::Tail));
          result.with_super_path(Path::from_components(alloc::vec![
            Component::PackField(PackField::Tail),
            Component::GenericPackMapping(GenericPackMapping {
              mapped_type: curr_mapping,
            }),
          ]));
          result
        }
        LookupResult::V1(_) => {
          let ok = env
            .current_mut()
            .mapped_generic_packs
            .bind_generic(super_tp, sub_tp);
          let mut result = SubtypingResult::uncacheable(ok);
          result.with_both_component(Component::PackField(PackField::Tail));
          result
        }
        LookupResult::V2(_) => {
          let mut result = SubtypingResult::uncacheable(sub_tp == super_tp);
          result.with_both_component(Component::PackField(PackField::Tail));
          result
        }
      },
    }
  }

  /// # Safety 说明
  /// 形参全为受检类型/引用（`scope` 非空 `&Scope`、arena 包句柄、`env` 独占借用），
  /// 契约由类型承载。
  pub(crate) fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_variadic_type_pack_type_pack_id_generic_type_pack(
    &mut self,
    env: &mut SubtypingEnvironment,
    scope: &Scope,
    sub_tp: TypePackId,
    _sub: &VariadicTypePack,
    super_tp: TypePackId,
    _super: &GenericTypePack,
  ) -> SubtypingResult {
    let lookup_result = env.lookup_generic_pack(super_tp);
    match lookup_result {
      LookupResult::V0(curr_mapping) => {
        let mut result = self
          .is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
            env,
            sub_tp,
            curr_mapping,
            scope,
          );
        result.with_sub_component(Component::PackField(PackField::Tail));
        result.with_super_path(Path::from_components(alloc::vec![
          Component::PackField(PackField::Tail),
          Component::GenericPackMapping(GenericPackMapping {
            mapped_type: curr_mapping,
          }),
        ]));
        result
      }
      LookupResult::V1(_) => {
        let ok = env
          .current_mut()
          .mapped_generic_packs
          .bind_generic(super_tp, sub_tp);
        let mut result = SubtypingResult::uncacheable(ok);
        result.with_both_component(Component::PackField(PackField::Tail));
        result
      }
      LookupResult::V2(_) => {
        let mut result = SubtypingResult::uncacheable_fail();
        result.with_both_component(Component::PackField(PackField::Tail));
        result
      }
    }
  }

  /// # Safety 说明
  /// 形参全为受检类型/引用（`scope` 非空 `&Scope`、arena 包句柄、`env` 独占借用），
  /// 契约由类型承载。
  pub(crate) fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_type_pack_id_variadic_type_pack(
    &mut self,
    env: &mut SubtypingEnvironment,
    scope: &Scope,
    sub_tp: TypePackId,
    _sub: &GenericTypePack,
    super_tp: TypePackId,
    super_variadic: &VariadicTypePack,
  ) -> SubtypingResult {
    // Safety: `self.builtin_types.as_ptr()` 是 Subtyping 构造期按 NotNull<BuiltinTypes>
    // 持有的会话级内置类型表，非空且比本次调用长寿；仅拷贝 `any_type` 这一个
    // TypeId 句柄值，只读。
    let any_type = self.builtin_types.get().any_type;
    // Safety: 同上——内置类型表长寿非空，本行只读拷贝 `unknown_type` 句柄值。
    let unknown_type = self.builtin_types.get().unknown_type;
    let super_ty = super_variadic.ty;

    if super_ty == any_type || super_ty == unknown_type {
      return SubtypingResult::ok();
    }

    let lookup_result = env.lookup_generic_pack(sub_tp);
    match lookup_result {
      LookupResult::V0(curr_mapping) => {
        let mut result = self
          .is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
            env,
            curr_mapping,
            super_tp,
            scope,
          );
        result.with_sub_path(Path::from_components(alloc::vec![
          Component::PackField(PackField::Tail),
          Component::GenericPackMapping(GenericPackMapping {
            mapped_type: curr_mapping,
          }),
        ]));
        result.with_super_component(Component::PackField(PackField::Tail));
        result
      }
      LookupResult::V1(_) => {
        let ok = env
          .current_mut()
          .mapped_generic_packs
          .bind_generic(sub_tp, super_tp);
        let mut result = SubtypingResult::uncacheable(ok);
        result.with_both_component(Component::PackField(PackField::Tail));
        result
      }
      LookupResult::V2(_) => {
        let mut result = SubtypingResult::uncacheable_fail();
        result.with_both_component(Component::PackField(PackField::Tail));
        result
      }
    }
  }

  /// # Safety 说明
  /// 形参全为受检类型/引用（`scope` 非空 `&Scope`、arena 包句柄、`env` 独占借用），
  /// 契约由类型承载。
  pub(crate) fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_nothing(
    &mut self,
    env: &mut SubtypingEnvironment,
    scope: &Scope,
    sub_tp: TypePackId,
    _sub: &GenericTypePack,
    _nothing: Nothing,
  ) -> SubtypingResult {
    // Safety: `self.builtin_types.as_ptr()` 由 Subtyping 构造契约保证非空且会话级长寿，
    // 此处仅拷贝 `empty_type_pack` 句柄值，只读。
    let empty_type_pack = self.builtin_types.get().empty_type_pack;
    let lookup_result = env.lookup_generic_pack(sub_tp);

    match lookup_result {
      LookupResult::V0(curr_mapping) => {
        let mut result = self
          .is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
            env,
            curr_mapping,
            empty_type_pack,
            scope,
          );
        result.with_sub_path(Path::from_components(alloc::vec![
          Component::PackField(PackField::Tail),
          Component::GenericPackMapping(GenericPackMapping {
            mapped_type: curr_mapping,
          }),
        ]));
        result
      }
      LookupResult::V1(_) => {
        let ok = env
          .current_mut()
          .mapped_generic_packs
          .bind_generic(sub_tp, empty_type_pack);
        let mut result = SubtypingResult::uncacheable(ok);
        result.with_sub_component(Component::PackField(PackField::Tail));
        result
      }
      LookupResult::V2(_) => {
        let mut result = SubtypingResult::uncacheable_fail();
        result.with_sub_component(Component::PackField(PackField::Tail));
        result
      }
    }
  }

  /// # Safety 说明
  /// 形参全为受检类型/引用（`scope` 非空 `&Scope`、arena 包句柄、`env` 独占借用），
  /// 契约由类型承载。
  pub(crate) fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_nothing_type_pack_id_generic_type_pack(
    &mut self,
    env: &mut SubtypingEnvironment,
    scope: &Scope,
    _nothing: Nothing,
    super_tp: TypePackId,
    _super: &GenericTypePack,
  ) -> SubtypingResult {
    // Safety: `self.builtin_types.as_ptr()` 为 Subtyping 构造期持有的非空内置类型表，
    // 读取 `empty_type_pack` 这一个句柄值后立即使用，只读且指针长寿。
    let empty_type_pack = self.builtin_types.get().empty_type_pack;
    let lookup_result = env.lookup_generic_pack(super_tp);

    match lookup_result {
      LookupResult::V0(curr_mapping) => {
        let mut result = self
          .is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
            env,
            empty_type_pack,
            curr_mapping,
            scope,
          );
        result.with_super_path(Path::from_components(alloc::vec![
          Component::PackField(PackField::Tail),
          Component::GenericPackMapping(GenericPackMapping {
            mapped_type: curr_mapping,
          }),
        ]));
        result
      }
      LookupResult::V1(_) => {
        let ok = env
          .current_mut()
          .mapped_generic_packs
          .bind_generic(super_tp, empty_type_pack);
        let mut result = SubtypingResult::uncacheable(ok);
        result.with_super_component(Component::PackField(PackField::Tail));
        result
      }
      LookupResult::V2(_) => {
        let mut result = SubtypingResult::uncacheable_fail();
        result.with_super_component(Component::PackField(PackField::Tail));
        result
      }
    }
  }
}
