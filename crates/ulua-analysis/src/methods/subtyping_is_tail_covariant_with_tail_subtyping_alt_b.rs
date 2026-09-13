use crate::{
  enums::pack_field::PackField,
  records::{
    generic_pack_mapping::GenericPackMapping, generic_type_pack::GenericTypePack, path::Path,
    scope::Scope, subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult,
  },
  type_aliases::{component::Component, lookup_result::LookupResult, type_pack_id::TypePackId},
};

impl Subtyping {
  /// # Safety
  /// 调用方须保证 `scope` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
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
        let mut result = SubtypingResult {
          is_subtype: ok,
          normalization_too_complex: false,
          is_cacheable: false,
          ..Default::default()
        };
        result.with_both_component(Component::PackField(PackField::Tail));
        result
      }
      LookupResult::V2(_) => match super_lookup_result {
        LookupResult::V0(curr_mapping) => {
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
          let mut result = SubtypingResult {
            is_subtype: ok,
            normalization_too_complex: false,
            is_cacheable: false,
            ..Default::default()
          };
          result.with_both_component(Component::PackField(PackField::Tail));
          result
        }
        LookupResult::V2(_) => {
          let mut result = SubtypingResult {
            is_subtype: sub_tp == super_tp,
            normalization_too_complex: false,
            is_cacheable: false,
            ..Default::default()
          };
          result.with_both_component(Component::PackField(PackField::Tail));
          result
        }
      },
    }
  }
}
