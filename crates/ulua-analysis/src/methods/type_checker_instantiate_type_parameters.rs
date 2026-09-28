use alloc::vec::Vec;

use ulua_ast::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_type_or_pack::AstTypeOrPack, location::Location,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type, get_function_name_as_string::get_function_name_as_string, get_type},
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
    let base_type = follow_type::follow(base_type);
    let function_type = get_type::get::<FunctionType>(base_type);

    if let Some(ft) = function_type {
      let mut type_params = Vec::with_capacity(ft.generics.len());
      for _ in &ft.generics {
        type_params.push(self.fresh_type_scope_ptr(scope.clone()));
      }
      let mut type_pack_params = Vec::with_capacity(ft.generic_packs.len());
      for _ in &ft.generic_packs {
        type_pack_params.push(self.fresh_type_pack_scope_ptr(&scope));
      }

      let mut type_param_count = 0;
      let mut type_pack_param_count = 0;
      let mut type_params_iter = type_params.iter_mut();
      let mut type_pack_params_iter = type_pack_params.iter_mut();

      for &type_or_pack in explicit_types.iter() {
        // 变体载荷即 arena 存活注解节点，直接按引用交给 resolve，判序与 cpp
        // `if (typeOrPack.type) … else …` 一致。
        match type_or_pack {
          AstTypeOrPack::Type(ast_ty) => {
            type_param_count += 1;
            if let Some(param) = type_params_iter.next() {
              *param = self.resolve_type(scope.clone(), ast_ty);
            }
          }
          AstTypeOrPack::Pack(ast_tp) => {
            type_pack_param_count += 1;
            if let Some(param) = type_pack_params_iter.next() {
              *param = self.resolve_type_pack_scope_ptr_ast_type_pack(scope.clone(), ast_tp);
            }
          }
          // cpp 在 `type` 为 null 时按 pack 槽解引用 `typePack`（TypeChecker.cpp
          // instantiateTypeParameters），双空属 parser 不变式违例；此处以断言上报
          // 并保持 pack 计数推进，不向下游透传空槽。
          AstTypeOrPack::Error => {
            LUAU_ASSERT!(false);
            type_pack_param_count += 1;
          }
        }
      }

      if type_param_count > ft.generics.len() || type_pack_param_count > ft.generic_packs.len() {
        let name = if !function_expr.is_null() {
          // Safety: 上一条件已排除 null；function_expr 为调用方（call 表达式
          // 检查路径）传入的 AST 表达式节点，位于 arena，活过本函数。
          // get_function_name_as_string 仅只读遍历节点形状，不留存借用。
          let fe = unsafe { &*function_expr };
          get_function_name_as_string(fe)
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
    if get_type::get::<IntersectionType>(base_type).is_some() {
      edge_case = InstantiateGenericsOnNonFunction::INTERSECTION;
    } else if let Some(mttv) = get_type::get::<MetatableType>(base_type)
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
