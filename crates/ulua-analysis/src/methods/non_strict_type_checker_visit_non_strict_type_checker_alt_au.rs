use alloc::string::String;
use core::{ffi::CStr, ptr::null_mut};

use ulua_ast::records::ast_type_reference::AstTypeReference;
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{finite::finite, first::first, size_type_pack::size},
  records::{
    constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
    generic_error::GenericError,
    incorrect_generic_parameter_count::IncorrectGenericParameterCount,
    non_strict_type_checker::NonStrictTypeChecker,
    swapped_generic_type_parameter::SwappedGenericTypeParameter,
    type_fun::TypeFun,
    unknown_symbol::{Context, UnknownSymbol},
  },
  type_aliases::name_type::Name,
};
impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `ty` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_reference(&mut self, ty: *mut AstTypeReference) {
    let ty_ref = unsafe { &*ty };

    // C++ compares `ty->name` against `kLuauPrint` ("_luau_print") and
    // `kLuauForceConstraintSolvingIncomplete`
    // ("_luau_force_constraint_solving_incomplete") from TypeUtils.h.
    if FFlag::DebugLuauMagicTypes.get() {
      let magic_name = unsafe { CStr::from_ptr(ty_ref.name.value).to_string_lossy() };
      // No further validation is necessary in this case.
      if magic_name == "_luau_print" {
        return;
      }

      if magic_name == "_luau_force_constraint_solving_incomplete" {
        let error = ConstraintSolvingIncompleteError::default();
        self.report_error(error.into(), &ty_ref.base.base.location);
        return;
      }
    }

    let params = ty_ref.parameters;
    for &param in params.as_slice() {
      if !param.r#type.is_null() {
        self.visit_ast_type(param.r#type);
      } else {
        self.visit_ast_type_pack(param.type_pack);
      }
    }

    let scope_ptr = self.find_innermost_scope(ty_ref.base.base.location);
    LUAU_ASSERT!(!scope_ptr.is_null());
    let scope = unsafe { &*scope_ptr };

    let name_str: Name = unsafe {
      CStr::from_ptr(ty_ref.name.value)
        .to_string_lossy()
        .into_owned()
    };

    let alias: Option<TypeFun> = if let Some(prefix) = ty_ref.prefix {
      let prefix_str: Name = unsafe { CStr::from_ptr(prefix.value).to_string_lossy().into_owned() };
      scope.lookup_imported_type(&prefix_str, &name_str)
    } else {
      scope.lookup_type(&name_str)
    };

    if let Some(ref alias_ref) = alias {
      let types_required = alias_ref.type_params().len();
      let packs_required = alias_ref.type_pack_params().len();

      let has_default_types = alias_ref
        .type_params()
        .iter()
        .any(|el| el.default_value.is_some());

      let has_default_packs = alias_ref
        .type_pack_params()
        .iter()
        .any(|el| el.default_value.is_some());

      if !ty_ref.has_parameter_list
        && ((!alias_ref.type_params().is_empty() && !has_default_types)
          || (!alias_ref.type_pack_params().is_empty() && !has_default_packs))
      {
        let error = GenericError::new(String::from("Type parameter list is required"));
        self.report_error(error.into(), &ty_ref.base.base.location);
      }

      let mut types_provided = 0usize;
      let mut extra_types = 0usize;
      let mut packs_provided = 0usize;

      for &param in params.as_slice() {
        if !param.r#type.is_null() {
          if packs_provided != 0 {
            let error = GenericError::new(String::from(
              "Type parameters must come before type pack parameters",
            ));
            self.report_error(error.into(), &ty_ref.base.base.location);
            continue;
          }

          if types_provided < types_required {
            types_provided += 1;
          } else {
            extra_types += 1;
          }
        } else if !param.type_pack.is_null() {
          let tp = self.lookup_pack_annotation(param.type_pack);
          if !tp.is_some() {
            continue;
          }

          let tp_id = tp.unwrap();
          // C++ `size(*tp) == 1 && finite(*tp) && first(*tp)` — `size`/`finite`
          // default their `TxnLog*` to nullptr; `first` defaults
          // `ignoreHiddenVariadics` to true.
          if types_provided < types_required
            && unsafe { size(tp_id, null_mut()) } == 1
            && unsafe { finite(tp_id, null_mut()) }
            && first(tp_id, true).is_some()
          {
            types_provided += 1;
          } else {
            packs_provided += 1;
          }
        }
      }

      if extra_types != 0 && packs_provided == 0 {
        // Extra types are only collected into a pack if a pack is expected
        if packs_required != 0 {
          packs_provided += 1;
        } else {
          types_provided += extra_types;
        }
      }

      let mut idx = types_provided;
      while idx < types_required {
        if let Some(param) = alias_ref.type_params().get(idx)
          && param.default_value.is_some()
        {
          types_provided += 1;
        }
        idx += 1;
      }

      let mut idx = packs_provided;
      while idx < packs_required {
        if let Some(param) = alias_ref.type_pack_params().get(idx)
          && param.default_value.is_some()
        {
          packs_provided += 1;
        }
        idx += 1;
      }

      if extra_types == 0 && packs_provided + 1 == packs_required {
        packs_provided += 1;
      }

      if types_provided != types_required || packs_provided != packs_required {
        let error = IncorrectGenericParameterCount {
          name: name_str.clone(),
          type_fun: alias_ref.clone(),
          actual_parameters: types_provided,
          actual_pack_parameters: packs_provided,
        };
        self.report_error(error.into(), &ty_ref.base.base.location);
      }
    } else {
      if scope.lookup_pack(&name_str).is_some() {
        let error = SwappedGenericTypeParameter {
          name: String::from(unsafe { CStr::from_ptr(ty_ref.name.value).to_string_lossy() }),
          kind: SwappedGenericTypeParameter::TYPE,
        };
        self.report_error(error.into(), &ty_ref.base.base.location);
      } else {
        let mut symbol = String::new();
        if let Some(prefix) = ty_ref.prefix {
          let prefix_str = unsafe { CStr::from_ptr(prefix.value).to_string_lossy().into_owned() };
          symbol.push_str(&prefix_str);
          symbol.push('.');
        }
        let name_lossy = unsafe { CStr::from_ptr(ty_ref.name.value).to_string_lossy() };
        symbol.push_str(&name_lossy);

        let error = UnknownSymbol::new(symbol, Context::Type);
        self.report_error(error.into(), &ty_ref.base.base.location);
      }
    }
  }
}
