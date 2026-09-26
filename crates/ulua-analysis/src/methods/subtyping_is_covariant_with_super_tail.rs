//! Faithful port of `Subtyping::isCovariantWithSuperTail`
//! (Analysis/src/Subtyping.cpp:1260-1338).
use alloc::vec::Vec;

use crate::{
  enums::{
    early_exit::EarlyExit, pack_field::PackField,
    subtyping_suppression_policy::SubtypingSuppressionPolicy, variant::Variant,
  },
  functions::{follow_type_pack, get_type_pack, slice_type_pack::slice_type_pack},
  methods::{
    path_builder_build::PathBuilderBuild, path_builder_tail::PathBuilderTail,
    path_builder_variadic::PathBuilderVariadic,
  },
  records::{
    free_type_pack::FreeTypePack, generic_pack_mapping::GenericPackMapping,
    generic_type_pack::GenericTypePack, index::Index, pack_slice::PackSlice,
    pack_subtype_constraint::PackSubtypeConstraint, path::Path, path_builder::PathBuilder,
    scope::Scope, subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult, type_error::TypeError, type_pack::TypePack,
    unexpected_type_pack_in_subtyping::UnexpectedTypePackInSubtyping,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    component::Component, constraint_v::ConstraintV, error_type_pack::ErrorTypePack,
    lookup_result::LookupResult, type_id::TypeId, type_pack_id::TypePackId,
  },
};
/// `is_covariant_with_super_tail` 的参数包（C++ Subtyping.cpp:1260 十参数签名，
/// `this` 之外九个）。按语义分组：环境与输出 / sub 侧 head+tail / super 侧 tail。
pub struct SuperTailCovariantArgs<'a> {
  // —— 环境与输出 ——
  pub env: &'a mut SubtypingEnvironment,
  pub output_result: &'a mut SubtypingResult,
  pub scope: *mut Scope,
  // —— sub 侧 head + tail ——
  pub sub_tp: TypePackId,
  pub sub_head_start_index: usize,
  pub sub_head: &'a [TypeId],
  pub sub_tail: Option<TypePackId>,
  // —— super 侧 tail ——
  pub super_tp: TypePackId,
  pub super_tail: TypePackId,
}
impl Subtyping {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn is_covariant_with_super_tail(
    &mut self,
    args: SuperTailCovariantArgs<'_>,
  ) -> EarlyExit {
    // 按原参数顺序解包，保持与 C++ 一一对应。
    let SuperTailCovariantArgs {
      env,
      output_result,
      sub_tp,
      sub_head_start_index,
      sub_head,
      sub_tail,
      super_tp,
      super_tail,
      scope,
    } = args;
    let _ = super_tp;

    if let Some(vt) = get_type_pack::get::<VariadicTypePack>(super_tail) {
      for (offset, &sub) in sub_head[sub_head_start_index..].iter().enumerate() {
        let i = sub_head_start_index + offset;
        let mut next = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
          env, sub, vt.ty, scope,
        );
        next.with_sub_component(Component::Index(Index {
          index: i,
          variant: Variant::Pack,
        }));
        next.with_super_path(
          PathBuilder {
            components: Vec::new(),
          }
          .tail()
          .variadic()
          .build(),
        );
        output_result.and_also(next, SubtypingSuppressionPolicy::Any);
      }
      EarlyExit::No
    } else if get_type_pack::get::<GenericTypePack>(super_tail).is_some() {
      let lookup_result = env.lookup_generic_pack(super_tail);
      let result: SubtypingResult;
      if let LookupResult::V2(_) = lookup_result {
        // get_if<MappedGenericEnvironment::NotBindable>
        let mut r = SubtypingResult::uncacheable_fail();
        r.with_sub_component(Component::PackSlice(PackSlice {
          start_index: sub_head_start_index,
        }));
        r.with_super_component(Component::PackField(PackField::Tail));
        result = r;
      } else {
        let sub_tail_pack = slice_type_pack(
          sub_head_start_index,
          sub_tp,
          sub_head,
          sub_tail,
          // Safety: self.builtin_types.as_ptr() 对应 C++ `NotNull<BuiltinTypes>`，构造期接线为非空、比本
          // Subtyping 长寿的进程级只读单例；此处只读共享借用，slice_type_pack 仅取 empty_type_pack。
          { self.builtin_types.get() },
          // Safety: self.arena.as_ptr() 对应 C++ `NotNull<TypeArena>`，构造接线非空且长寿；重建的 &mut 仅
          // 存活于本次 slice_type_pack 调用（内部只 add_type_pack），单线程、此刻 arena 无其他借用。
          { self.arena.get_mut() },
        );

        if let LookupResult::V0(mapped_gen) = lookup_result {
          // get_if<TypePackId> — subtype against the mapped generic pack.
          let mut super_tp_to_compare = mapped_gen;

          // If mappedGen has a hidden variadic tail, we clip it for better
          // arity mismatch reporting.
          if let Some(tp) = get_type_pack::get::<TypePack>(mapped_gen)
            && let Some(tail) = tp.tail
            && let Some(vtp) =
              get_type_pack::get::<VariadicTypePack>(follow_type_pack::follow(tail))
            && vtp.hidden
          {
            super_tp_to_compare =
              // Safety: self.arena.as_ptr() 为构造接线非空、长寿的 arena；此处独占短借用追加一个仅含
              // tp.head 的新 pack，bump arena 已分配块地址不移动，返回句柄在本次比较期内存活。
               self.arena.get_mut().add_type_pack_initializer_list_type_id(&tp.head);
          }

          // Safety: 被调 unsafe fn 的契约要求——env 独占借用、scope 非空且检查期存活、两侧
          // TypePackId 为 arena 存活句柄。此处 env 是本函数解构出的独占参数，scope 满足本方法入口
          // `# Safety`（C++ `NotNull<Scope>`），sub_tail_pack / super_tp_to_compare 均由 arena 分配
          // 且此刻存活；单线程递归下 self 无并存借用。
          let mut r = unsafe {
            self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
              env,
              sub_tail_pack,
              super_tp_to_compare,
              scope,
            )
          };
          r.with_sub_component(Component::PackSlice(PackSlice {
            start_index: sub_head_start_index,
          }));
          r.with_super_path(Path::from_components(alloc::vec![
            Component::PackField(PackField::Tail),
            Component::GenericPackMapping(GenericPackMapping {
              mapped_type: mapped_gen
            }),
          ]));
          result = r;
        } else {
          // get_if<MappedGenericEnvironment::Unmapped>
          let ok = env
            .mapped_generic_packs
            .bind_generic(super_tail, sub_tail_pack);
          let mut r = SubtypingResult::uncacheable(ok);
          r.with_sub_component(Component::PackSlice(PackSlice {
            start_index: sub_head_start_index,
          }));
          r.with_super_component(Component::PackField(PackField::Tail));
          result = r;
        }
      }

      output_result.and_also(result, SubtypingSuppressionPolicy::Any);
      EarlyExit::Yes
    } else if get_type_pack::get::<ErrorTypePack>(super_tail).is_some() {
      let mut r = SubtypingResult::ok();
      r.with_super_component(Component::PackField(PackField::Tail));
      *output_result = r;
      EarlyExit::Yes
    } else if get_type_pack::get::<FreeTypePack>(super_tail).is_some() {
      let sub_tail_pack = slice_type_pack(
        sub_head_start_index,
        sub_tp,
        sub_head,
        sub_tail,
        // Safety: self.builtin_types.as_ptr() 对应 C++ `NotNull<BuiltinTypes>`，构造接线非空、比 Subtyping
        // 长寿的只读单例；此处共享借用仅供 slice_type_pack 读 empty_type_pack。
        { self.builtin_types.get() },
        // Safety: self.arena.as_ptr() 对应 C++ `NotNull<TypeArena>`，构造接线非空且长寿；&mut 重建仅在
        // slice_type_pack 调用期内独占存活（单线程，无并存 arena 借用）。
        { self.arena.get_mut() },
      );
      let mut r = SubtypingResult::ok();
      r.with_super_component(Component::PackField(PackField::Tail));
      r.with_assumed_constraint(ConstraintV::PackSubtype(PackSubtypeConstraint {
        sub_pack: sub_tail_pack,
        super_pack: super_tail,
        returns: false,
      }));
      output_result.and_also(r, SubtypingSuppressionPolicy::Any);
      EarlyExit::Yes
    } else {
      let mut r = SubtypingResult::fail();
      r.with_super_component(Component::PackField(PackField::Tail));
      r.with_error(TypeError::type_error_location_type_error_data(
        // Safety: scope 是本方法入口 `# Safety` 契约要求的非空 NotNull<Scope>，在整个协变检查
        // 期间存活；此分支仅在尾包不匹配时只读其 location。
        unsafe { (*scope).location },
        UnexpectedTypePackInSubtyping { tp: super_tail }.into(),
      ));
      *output_result = r;
      EarlyExit::Yes
    }
  }
}
