//! Faithful port of `Subtyping::isSubTailCovariantWith`
//! (Analysis/src/Subtyping.cpp:1180-1258).
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
/// `is_sub_tail_covariant_with` 的参数包（C++ Subtyping.cpp:1180 十参数签名，
/// `this` 之外九个）。按语义分组：环境与输出 / sub 侧 tail / super 侧 head+tail。
pub struct SubTailCovariantArgs<'a> {
  // —— 环境与输出 ——
  pub env: &'a mut SubtypingEnvironment,
  pub output_result: &'a mut SubtypingResult,
  pub scope: *mut Scope,
  // —— sub 侧 tail ——
  pub sub_tp: TypePackId,
  pub sub_tail: TypePackId,
  // —— super 侧 head + tail ——
  pub super_tp: TypePackId,
  pub super_head_start_index: usize,
  pub super_head: &'a [TypeId],
  pub super_tail: Option<TypePackId>,
}
impl Subtyping {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn is_sub_tail_covariant_with(&mut self, args: SubTailCovariantArgs<'_>) -> EarlyExit {
    // 按原参数顺序解包，保持与 C++ 一一对应。
    let SubTailCovariantArgs {
      env,
      output_result,
      sub_tp,
      sub_tail,
      super_tp,
      super_head_start_index,
      super_head,
      super_tail,
      scope,
    } = args;
    let _ = sub_tp;

    if let Some(vt) = get_type_pack::get::<VariadicTypePack>(sub_tail) {
      for (offset, &super_head_ty) in super_head[super_head_start_index..].iter().enumerate() {
        let i = super_head_start_index + offset;
        let mut next = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
          env,
          vt.ty,
          super_head_ty,
          scope,
        );
        next.with_sub_path(
          PathBuilder {
            components: Vec::new(),
          }
          .tail()
          .variadic()
          .build(),
        );
        next.with_super_component(Component::Index(Index {
          index: i,
          variant: Variant::Pack,
        }));
        output_result.and_also(next, SubtypingSuppressionPolicy::Any);
      }
      EarlyExit::No
    } else if get_type_pack::get::<GenericTypePack>(sub_tail).is_some() {
      let lookup_result = env.lookup_generic_pack(sub_tail);
      let result: SubtypingResult;
      if let LookupResult::V2(_) = lookup_result {
        // get_if<MappedGenericEnvironment::NotBindable>
        let mut r = SubtypingResult::uncacheable_fail();
        r.with_sub_component(Component::PackField(PackField::Tail));
        r.with_super_component(Component::PackSlice(PackSlice {
          start_index: super_head_start_index,
        }));
        result = r;
      } else {
        let super_tail_pack = slice_type_pack(
          super_head_start_index,
          super_tp,
          super_head,
          super_tail,
          // Safety: `self.builtin_types.as_ptr()` 对应 C++ `NotNull<BuiltinTypes>` 只读内建
          // 单例，构造接线非空、比持有者长寿；取共享借用只读。
          { self.builtin_types.get() },
          // Safety: `self.arena.as_ptr()` 对应 `NotNull<TypeArena>`，非空且块地址在遍历期不
          // 移动；本次单线程串行独占可变借用喂给 `slice_type_pack`，无别名冲突。
          { self.arena.get_mut() },
        );

        if let LookupResult::V0(mapped_gen) = lookup_result {
          // get_if<TypePackId> — subtype against the mapped generic pack.
          let mut sub_tp_to_compare = mapped_gen;

          // If mappedGen has a hidden variadic tail, we clip it for better
          // arity mismatch reporting.
          if let Some(tp) = get_type_pack::get::<TypePack>(mapped_gen)
            && let Some(tail) = tp.tail
            && let Some(vtp) =
              get_type_pack::get::<VariadicTypePack>(follow_type_pack::follow(tail))
            && vtp.hidden
          {
            sub_tp_to_compare =
              // Safety: `self.arena.as_ptr()` 非空存活、add 不移动既有节点；`tp` 由
              // `get_type_pack::get::<TypePack>(mapped_gen)` 得到，是 arena 内存活的
              // `TypePack`，`&tp.head` 只读其切片喂给新建 pack。
               self.arena.get_mut().add_type_pack_initializer_list_type_id(&tp.head);
          }

          let mut r = unsafe {
            // Safety: 被调 `_not_null_scope` 变体要求 `scope` 非空有效——它与本函数
            // 终分支直接解引用的 `scope` 同源（入口 unsafe 契约保证存活非空 `Scope`）；
            // `env` 为入参活借用，`sub_tp_to_compare`/`super_tail_pack` 为 arena 存活
            // `TypePackId`；`self` 独占可变借用无别名冲突。
            self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
              env,
              sub_tp_to_compare,
              super_tail_pack,
              scope,
            )
          };
          r.with_sub_path(Path::from_components(alloc::vec![
            Component::PackField(PackField::Tail),
            Component::GenericPackMapping(GenericPackMapping {
              mapped_type: mapped_gen
            }),
          ]));
          r.with_super_component(Component::PackSlice(PackSlice {
            start_index: super_head_start_index,
          }));
          result = r;
        } else {
          // get_if<MappedGenericEnvironment::Unmapped>
          let ok = env
            .mapped_generic_packs
            .bind_generic(sub_tail, super_tail_pack);
          let mut r = SubtypingResult::uncacheable(ok);
          r.with_sub_component(Component::PackField(PackField::Tail));
          r.with_super_component(Component::PackSlice(PackSlice {
            start_index: super_head_start_index,
          }));
          result = r;
        }
      }

      output_result.and_also(result, SubtypingSuppressionPolicy::Any);
      EarlyExit::Yes
    } else if get_type_pack::get::<ErrorTypePack>(sub_tail).is_some() {
      let mut r = SubtypingResult::ok();
      r.with_sub_component(Component::PackField(PackField::Tail));
      *output_result = r;
      EarlyExit::Yes
    } else if get_type_pack::get::<FreeTypePack>(sub_tail).is_some() {
      let super_tail_pack = slice_type_pack(
        super_head_start_index,
        super_tp,
        super_head,
        super_tail,
        // Safety: `self.builtin_types.as_ptr()` 为构造接线的非空只读内建单例，取共享借用。
        self.builtin_types.get(),
        // Safety: `self.arena.as_ptr()` 非空、块地址稳定，单线程串行独占短借用喂给 slice_type_pack。
        self.arena.get_mut(),
      );
      let mut r = SubtypingResult::ok();
      r.with_sub_component(Component::PackField(PackField::Tail));
      r.with_assumed_constraint(ConstraintV::PackSubtype(PackSubtypeConstraint {
        sub_pack: sub_tail,
        super_pack: super_tail_pack,
        returns: false,
      }));
      output_result.and_also(r, SubtypingSuppressionPolicy::Any);
      EarlyExit::Yes
    } else {
      let mut r = SubtypingResult::fail();
      r.with_sub_component(Component::PackField(PackField::Tail));
      r.with_error(TypeError::type_error_location_type_error_data(
        // Safety: `scope` 为本 unsafe fn 入口契约要求的非空存活 `Scope` 指针，调用方
        // 沿子类型遍历一路透传同一 `scope`（递归分支亦原样转交），此处仅只读其 `location`。
        unsafe { (*scope).location },
        UnexpectedTypePackInSubtyping { tp: sub_tail }.into(),
      ));
      *output_result = r;
      EarlyExit::Yes
    }
  }
}
