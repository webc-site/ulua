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
    scope: *mut Scope,
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

  /// # Safety
  /// - `scope` 须为沿 `is_subtype` 入口按 NotNull<Scope> 契约逐级传递、在本次
  ///   递归全程存活的非空 `Scope` 指针（本函数原样转发给 unsafe 的
  ///   `is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope`）。
  /// - `sub_tp`/`super_tp` 须为类型 arena 中已初始化的 `TypePackVar` 句柄，
  ///   且两个 GenericTypePack 借用（`_sub`/`_super`）由该句柄 follow 而来。
  /// - `env` 为调用方独占的递归环境借用。
  pub(crate) fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_type_pack_id_generic_type_pack(
    &mut self,
    env: &mut SubtypingEnvironment,
    scope: *mut Scope,
    sub_tp: TypePackId,
    _sub: &GenericTypePack,
    super_tp: TypePackId,
    _super: &GenericTypePack,
  ) -> SubtypingResult {
    let sub_lookup_result = env.lookup_generic_pack(sub_tp);
    let super_lookup_result = env.lookup_generic_pack(super_tp);

    match sub_lookup_result {
      LookupResult::V0(curr_mapping) => {
        // Safety: `curr_mapping` 是 env 中 sub_tp 已绑定的包句柄（与本函数入参
        // 同源 arena）；`scope` 按本函数头部契约非空存活；`env` 此处独占传入。
        let mut result = unsafe {
          self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
            env,
            curr_mapping,
            super_tp,
            scope,
          )
        };
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
        let ok = env.mapped_generic_packs.bind_generic(sub_tp, super_tp);
        let mut result = SubtypingResult::uncacheable(ok);
        result.with_both_component(Component::PackField(PackField::Tail));
        result
      }
      LookupResult::V2(_) => match super_lookup_result {
        LookupResult::V0(curr_mapping) => {
          // Safety: `curr_mapping` 来自 super_tp 在 env 中的映射句柄，`sub_tp` 为
          // arena 已分配包；`scope`/`env` 满足本函数头部的 NotNull 与独占借用契约。
          let mut result = unsafe {
            self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
              env,
              sub_tp,
              curr_mapping,
              scope,
            )
          };
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
          let ok = env.mapped_generic_packs.bind_generic(super_tp, sub_tp);
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

  /// # Safety
  /// - `scope` 须为 NotNull<Scope> 语义逐级转发、递归全程存活的非空指针，
  ///   仅被转发给 unsafe 的 `is_covariant_with_*_not_null_scope` 使用。
  /// - `sub_tp` 为已 follow 出 `_sub`（VariadicTypePack）的 arena 包句柄；
  ///   `super_tp` 为 `_super`（GenericTypePack）的句柄，仅用于 env 查找/绑定。
  /// - `env` 为调用方在该递归层独占的可变借用。
  pub(crate) fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_variadic_type_pack_type_pack_id_generic_type_pack(
    &mut self,
    env: &mut SubtypingEnvironment,
    scope: *mut Scope,
    sub_tp: TypePackId,
    _sub: &VariadicTypePack,
    super_tp: TypePackId,
    _super: &GenericTypePack,
  ) -> SubtypingResult {
    let lookup_result = env.lookup_generic_pack(super_tp);
    match lookup_result {
      LookupResult::V0(curr_mapping) => {
        // Safety: `sub_tp` 是变参尾部所属的 arena 包句柄、`curr_mapping` 由
        // super_tp 在 env 解出；`scope` 沿 NotNull 契约转发，`env` 独占借用，
        // 满足被调 unsafe fn 对句柄存活与 scope 非空的要求。
        let mut result = unsafe {
          self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
            env,
            sub_tp,
            curr_mapping,
            scope,
          )
        };
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
        let ok = env.mapped_generic_packs.bind_generic(super_tp, sub_tp);
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

  /// # Safety
  /// - `scope` 须非空且在本次递归判定全程存活（NotNull<Scope> 转发契约），本函数
  ///   仅把它交给 unsafe 的 `is_covariant_with_*_not_null_scope`。
  /// - `sub_tp`/`super_tp` 为 arena 分配的包句柄，分别已 follow 出 `_sub`
  ///   （GenericTypePack）与 `super_variadic`（VariadicTypePack）。
  /// - `env` 为当前递归层独占借用；本函数还会解引用 `self.builtin_types.as_ptr()`，
  ///   其非空与长寿由 Subtyping 构造契约保证。
  pub(crate) fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_type_pack_id_variadic_type_pack(
    &mut self,
    env: &mut SubtypingEnvironment,
    scope: *mut Scope,
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
        // Safety: `curr_mapping` 为泛型 sub_tp 在 env 中已绑定的包句柄，
        // `super_tp` 即变参尾部句柄；`scope`/`env` 沿用本函数头部的存活与非空
        // 契约转发给 unsafe 被调方。
        let mut result = unsafe {
          self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
            env,
            curr_mapping,
            super_tp,
            scope,
          )
        };
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
        let ok = env.mapped_generic_packs.bind_generic(sub_tp, super_tp);
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

  /// # Safety
  /// - `scope` 须为 NotNull<Scope> 语义转发、判定全程存活的非空指针；本函数
  ///   把它连同 arena 句柄传给 unsafe 的 `is_covariant_with_*_not_null_scope`。
  /// - `sub_tp` 为已 follow 出 `_sub`（GenericTypePack）的包句柄；空尾包一侧
  ///   使用 `builtin_types.empty_type_pack`，要求 `self.builtin_types.as_ptr()` 满足
  ///   Subtyping 的构造契约（非空、长寿）。
  /// - `env` 为当前递归层独占借用。
  pub(crate) fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_nothing(
    &mut self,
    env: &mut SubtypingEnvironment,
    scope: *mut Scope,
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
        // Safety: `curr_mapping` 是 sub_tp 在 env 的绑定句柄，`empty_type_pack`
        // 为内置 arena 常量包；`scope`/`env` 依本函数头部契约传入 unsafe 被调。
        let mut result = unsafe {
          self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
            env,
            curr_mapping,
            empty_type_pack,
            scope,
          )
        };
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

  /// # Safety
  /// - `scope` 须非空并存活至本次判定结束，仅原样转发给 unsafe 的
  ///   `is_covariant_with_*_not_null_scope`（NotNull<Scope> 契约）。
  /// - `super_tp` 为已 follow 出 `_super`（GenericTypePack）的包句柄；空侧
  ///   传入 `builtin_types.empty_type_pack`，依赖 `self.builtin_types.as_ptr()` 的
  ///   构造期非空不变量。
  /// - `env` 为当前递归层独占借用，其映射表在 bind_generic 前后保持一致。
  pub(crate) fn is_tail_covariant_with_tail_subtyping_environment_not_null_scope_nothing_type_pack_id_generic_type_pack(
    &mut self,
    env: &mut SubtypingEnvironment,
    scope: *mut Scope,
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
        // Safety: `curr_mapping` 由 super_tp 在 env 解出、`empty_type_pack` 为
        // 内置常量包句柄，二者与 `scope`（非空存活）、`env`（独占借用）共同
        // 满足被调 unsafe fn 的入参契约。
        let mut result = unsafe {
          self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
            env,
            empty_type_pack,
            curr_mapping,
            scope,
          )
        };
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
