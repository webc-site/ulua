use core::{ffi::CStr, ptr::null};

use ulua_ast::records::{
  ast_array::AstArray, ast_generic_type::AstGenericType, ast_generic_type_pack::AstGenericTypePack,
  ast_name::AstName,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{duplicate_generic_parameter::DuplicateGenericParameter, type_checker_2::TypeChecker2},
  type_aliases::type_error_data::TypeErrorData,
};
impl TypeChecker2 {
  pub fn visit_generics(
    &mut self,
    generics: AstArray<*mut AstGenericType>,
    generic_packs: AstArray<*mut AstGenericTypePack>,
  ) {
    let mut seen: DenseHashSet<AstName> = DenseHashSet::new(AstName { value: null() });

    for generic in generics.iter() {
      let generic = unsafe { &**generic };
      let name = generic.name;

      if seen.contains(&name) {
        let parameter_name = unsafe { CStr::from_ptr(name.value).to_string_lossy().into_owned() };
        self.report_error_type_error_data_location(
          TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(parameter_name)),
          &generic.base.location,
        );
      } else {
        seen.insert(name);
      }

      if !generic.default_value.is_null() {
        unsafe { self.visit_ast_type(generic.default_value) };
      }
    }

    for generic_pack in generic_packs.iter() {
      let generic_pack = unsafe { &**generic_pack };
      let name = generic_pack.name;

      if seen.contains(&name) {
        let parameter_name = unsafe { CStr::from_ptr(name.value).to_string_lossy().into_owned() };
        self.report_error_type_error_data_location(
          TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(parameter_name)),
          &generic_pack.base.location,
        );
      } else {
        seen.insert(name);
      }

      if !generic_pack.default_value.is_null() {
        self.visit_ast_type_pack(generic_pack.default_value);
      }
    }
  }
}
