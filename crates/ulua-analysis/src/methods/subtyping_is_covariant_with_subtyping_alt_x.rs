use alloc::string::ToString;

use ulua_common::FFlag;

use crate::{
  enums::subtyping_suppression_policy::SubtypingSuppressionPolicy,
  records::{
    property_type::Property, property_type_path::Property as PathProperty, scope::Scope,
    subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult,
  },
  type_aliases::component::Component,
};
fn property_component(name: &str, read: bool) -> Component {
  Component::Property(PathProperty {
    name: name.to_string(),
    is_read: read,
  })
}

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_property_property_string_bool_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_prop: &Property,
    super_prop: &Property,
    name: &str,
    force_covariant_test: bool,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut res = SubtypingResult {
      is_subtype: true,
      ..Default::default()
    };

    if super_prop.is_shared() && sub_prop.is_shared() {
      let sub_ty = sub_prop.read_ty.unwrap();
      let super_ty = super_prop.read_ty.unwrap();
      let mut part = if force_covariant_test {
        self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
          env, sub_ty, super_ty, scope,
        )
      } else {
        self.is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
          env, sub_ty, super_ty, scope,
        )
      };
      part.with_both_component(property_component(name, true));
      res.and_also(part, SubtypingSuppressionPolicy::Any);
    } else {
      if let (Some(sub_read), Some(super_read)) = (sub_prop.read_ty, super_prop.read_ty) {
        let mut part = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
          env, sub_read, super_read, scope,
        );
        part.with_both_component(property_component(name, true));
        res.and_also(part, SubtypingSuppressionPolicy::Any);
      }

      if let (Some(sub_write), Some(super_write)) = (sub_prop.write_ty, super_prop.write_ty)
        && !force_covariant_test
      {
        let mut part = self
          .is_contravariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
            env,
            sub_write,
            super_write,
            scope,
          );
        part.with_both_component(property_component(name, false));
        res.and_also(part, SubtypingSuppressionPolicy::Any);
      }

      let super_is_read_write = super_prop.read_ty.is_some() && super_prop.write_ty.is_some();
      let sub_is_read_only = sub_prop.read_ty.is_some() && sub_prop.write_ty.is_none();
      let sub_is_write_only = sub_prop.read_ty.is_none() && sub_prop.write_ty.is_some();

      if super_is_read_write {
        if sub_is_read_only {
          let mut part = SubtypingResult {
            is_subtype: false,
            ..Default::default()
          };
          part.with_both_component(property_component(name, true));
          if FFlag::LuauPropertyModifierMismatchErrors.get() {
            part.with_property_modifier_violation();
          }
          res.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if sub_is_write_only {
          let mut part = SubtypingResult {
            is_subtype: false,
            ..Default::default()
          };
          part.with_both_component(property_component(name, false));
          if FFlag::LuauPropertyModifierMismatchErrors.get() {
            part.with_property_modifier_violation();
          }
          res.and_also(part, SubtypingSuppressionPolicy::Any);
        }
      }
    }

    res
  }
}
