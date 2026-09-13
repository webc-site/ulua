use alloc::vec::Vec;

use ulua_ast::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_type_or_pack::AstTypeOrPack, location::Location,
};

use crate::{
  functions::{
    follow_type::follow_type_id, get_function_name_as_string::get_function_name_as_string,
    get_type_alt_j::get_type_id,
  },
  records::{
    function_type::FunctionType, generic_type_definition::GenericTypeDefinition,
    generic_type_pack_definition::GenericTypePackDefinition,
    instantiate_generics_on_non_function::InstantiateGenericsOnNonFunction,
    intersection_type::IntersectionType, metatable_type::MetatableType, type_checker::TypeChecker,
    type_fun::TypeFun, type_instantiation_count_mismatch::TypeInstantiationCountMismatch,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId},
};
impl TypeChecker {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn instantiate_type_parameters(
    &mut self,
    scope: ScopePtr,
    base_type: TypeId,
    explicit_types: AstArray<AstTypeOrPack>,
    function_expr: *const AstExpr,
    location: &Location,
  ) -> TypeId {
    let base_type = follow_type_id(base_type);
    let function_type = get_type_id::<FunctionType>(base_type);

    if let Some(ft) = function_type {
      let mut type_params = Vec::with_capacity(ft.generics.len());
      for _ in &ft.generics {
        type_params.push(self.fresh_type_scope_ptr(scope.clone()));
      }
      let mut type_pack_params = Vec::with_capacity(ft.generic_packs.len());
      for _ in &ft.generic_packs {
        type_pack_params.push(self.fresh_type_pack_scope_ptr(scope.clone()));
      }

      let mut type_param_count = 0;
      let mut type_pack_param_count = 0;
      let mut type_params_iter = type_params.iter_mut();
      let mut type_pack_params_iter = type_pack_params.iter_mut();

      for type_or_pack in explicit_types.iter() {
        if !type_or_pack.r#type.is_null() {
          type_param_count += 1;
          if let Some(param) = type_params_iter.next() {
            *param = self.resolve_type(scope.clone(), unsafe { &*type_or_pack.r#type });
          }
        } else {
          type_pack_param_count += 1;
          if let Some(param) = type_pack_params_iter.next() {
            *param = self.resolve_type_pack_scope_ptr_ast_type_pack(scope.clone(), unsafe {
              &*type_or_pack.type_pack
            });
          }
        }
      }

      if type_param_count > ft.generics.len() || type_pack_param_count > ft.generic_packs.len() {
        let name = if !function_expr.is_null() {
          get_function_name_as_string(unsafe { &*function_expr })
        } else {
          None
        };
        self.report_error_location_type_error_data(
          location,
          TypeErrorData::TypeInstantiationCountMismatch(TypeInstantiationCountMismatch {
            function_name: name,
            function_type: base_type,
            provided_types: type_param_count,
            maximum_types: ft.generics.len(),
            provided_type_packs: type_pack_param_count,
            maximum_type_packs: ft.generic_packs.len(),
          }),
        );
      }

      let mut base_fun = TypeFun::type_fun_type_id(base_type);
      base_fun.type_params.reserve(ft.generics.len());
      for &generic_id in &ft.generics {
        base_fun.type_params.push(GenericTypeDefinition {
          ty: generic_id,
          default_value: None,
        });
      }

      base_fun.type_pack_params.reserve(ft.generic_packs.len());
      for &generic_pack_id in &ft.generic_packs {
        base_fun.type_pack_params.push(GenericTypePackDefinition {
          tp: generic_pack_id,
          default_value: None,
        });
      }

      return self.instantiate_type_fun(
        &scope,
        &base_fun,
        &type_params,
        &type_pack_params,
        location,
      );
    }

    let mut edge_case = InstantiateGenericsOnNonFunction::NONE;
    if get_type_id::<IntersectionType>(base_type).is_some() {
      edge_case = InstantiateGenericsOnNonFunction::INTERSECTION;
    } else if let Some(mttv) = get_type_id::<MetatableType>(base_type)
      && self
        .get_index_type_from_type_impl(
          scope,
          mttv.metatable(),
          &"__call".to_string(),
          location,
          false,
        )
        .is_some()
    {
      edge_case = InstantiateGenericsOnNonFunction::METATABLE_CALL;
    }

    self.report_error_location_type_error_data(
      location,
      TypeErrorData::InstantiateGenericsOnNonFunction(InstantiateGenericsOnNonFunction {
        interesting_edge_case: edge_case,
      }),
    );
    base_type
  }
}
