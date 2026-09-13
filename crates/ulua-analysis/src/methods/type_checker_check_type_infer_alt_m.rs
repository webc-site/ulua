use alloc::string::String;
use core::ffi::CStr;

use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_node::AstNode, ast_stat_function::AstStatFunction,
  },
  rtti::ast_node_try_as,
};

use crate::{
  enums::{control_flow::ControlFlow, table_state::TableState},
  functions::{
    follow_type::follow_type_id, get_mutable_table_type::get_mutable_table_type,
    is_table_intersection::is_table_intersection,
  },
  methods::type_checker_check_function_signature::scope_mut,
  records::{
    binding::Binding,
    cannot_extend_table::{CannotExtendTable, Context},
    only_tables_can_have_methods::OnlyTablesCanHaveMethods,
    symbol::Symbol,
    type_checker::TypeChecker,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId},
};
impl TypeChecker {
  pub fn check_scope_ptr_type_id_scope_ptr_ast_stat_function(
    &mut self,
    scope: &ScopePtr,
    mut ty: TypeId,
    fun_scope: &ScopePtr,
    function: &AstStatFunction,
  ) -> ControlFlow {
    // SAFETY: function.name 指向 AST arena 节点；repr(C) base 偏移 0，cast 有效。
    let name_node = unsafe { &*function.name.cast::<AstNode>() };

    if let Some(expr_name) = ast_node_try_as::<AstExprGlobal>(name_node) {
      let module_scope = self.current_module.as_ref().unwrap().get_module_scope();
      let name = Symbol::from_global(expr_name.name);
      let previously_defined =
        self.is_nonstrict_mode() && module_scope.bindings.contains_key(&name);
      let old_binding = if previously_defined {
        module_scope.bindings.get(&name).cloned()
      } else {
        None
      };

      // SAFETY: 见 scope_mut 契约。
      unsafe {
        (*scope_mut(&module_scope)).bindings.insert(
          name.clone(),
          Binding {
            type_id: ty,
            location: expr_name.base.base.location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
      }

      // SAFETY: function.func 指向 AST arena 节点。
      self.check_function_body(fun_scope, ty, unsafe { &*function.func });

      let final_binding = if let Some(old) = old_binding {
        old
      } else {
        Binding {
          type_id: self.quantify(fun_scope, ty, expr_name.base.base.location),
          location: expr_name.base.base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        }
      };

      // SAFETY: 见 scope_mut 契约。
      unsafe {
        (*scope_mut(&module_scope))
          .bindings
          .insert(name, final_binding);
      }

      return ControlFlow::None;
    }

    if let Some(local_name) = ast_node_try_as::<AstExprLocal>(name_node) {
      let symbol = Symbol::from_local(local_name.local);
      // SAFETY: local 指向 AST arena 节点。
      let name_location = unsafe { (*local_name.local).location };

      // SAFETY: 见 scope_mut 契约。
      unsafe {
        (*scope_mut(scope)).bindings.insert(
          symbol.clone(),
          Binding {
            type_id: ty,
            location: name_location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
      }

      // SAFETY: function.func 指向 AST arena 节点。
      self.check_function_body(fun_scope, ty, unsafe { &*function.func });

      let quantified_ty = self.quantify(fun_scope, ty, name_location);
      let quantified = self.any_if_nonstrict(quantified_ty);
      // SAFETY: 见 scope_mut 契约。
      unsafe {
        (*scope_mut(scope)).bindings.insert(
          symbol,
          Binding {
            type_id: quantified,
            location: name_location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
      }

      return ControlFlow::None;
    }

    if let Some(index_name) = ast_node_try_as::<AstExprIndexName>(name_node) {
      let expr_ty = self
        .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
          scope,
          // SAFETY: index_name->expr 指向 AST arena 节点。
          unsafe { &*index_name.expr },
          None,
          false,
        )
        .r#type;
      let mut ttv = get_mutable_table_type(expr_ty);
      // SAFETY: index.value 为 NUL 结尾 C 字符串（AST arena 持有）。
      let prop_name = unsafe {
        CStr::from_ptr(index_name.index.value)
          .to_string_lossy()
          .into_owned()
      };

      if self
        .get_index_type_from_type(
          scope.clone(),
          expr_ty,
          &prop_name,
          &index_name.index_location,
          false,
        )
        .is_none()
      {
        let data = if ttv.is_some() || is_table_intersection(expr_ty) {
          TypeErrorData::CannotExtendTable(CannotExtendTable {
            table_type: expr_ty,
            context: Context::Property,
            prop: prop_name.clone(),
          })
        } else {
          TypeErrorData::OnlyTablesCanHaveMethods(OnlyTablesCanHaveMethods {
            table_type: expr_ty,
          })
        };

        self.report_error_location_type_error_data(&function.base.base.location, data);
      }

      ty = follow_type_id(ty);
      if let Some(ttv) = ttv.as_deref_mut()
        && ttv.state != TableState::Sealed
      {
        let property = ttv.props.entry(prop_name.clone()).or_default();
        property.set_type(ty);
        property.location = Some(index_name.index_location);
      }

      // SAFETY: function.func 指向 AST arena 节点。
      self.check_function_body(fun_scope, ty, unsafe { &*function.func });

      // SAFETY: 同上，check_function_body 可能改变 state，故再次读取。
      if let Some(ttv) = ttv
        && ttv.state != TableState::Sealed
      {
        let quantified = follow_type_id(self.quantify(fun_scope, ty, index_name.index_location));
        let property = ttv.props.entry(prop_name).or_default();
        property.set_type(quantified);
        property.location = Some(index_name.index_location);
      }

      return ControlFlow::None;
    }

    // SAFETY: function.func 指向 AST arena 节点。
    self.check_function_body(fun_scope, ty, unsafe { &*function.func });
    ControlFlow::None
  }
}
