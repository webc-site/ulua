use alloc::string::String;

use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString, ast_expr_global::AstExprGlobal,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal,
  },
  rtti::ast_node_try_as,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::{table_state::TableState, value_context::ValueContext},
  functions::{
    arc_as_mut::arc_as_mut,
    begin_type::{begin_intersection_type, begin_union_type},
    follow_type,
    get_mutable_table_type::get_mutable_table_type,
    get_type,
    is_table_intersection::is_table_intersection,
    lookup_extern_type_prop::lookup_extern_type_prop,
    reduce_union::reduce_union,
  },
  records::{
    any_type::AnyType,
    binding::Binding,
    cannot_extend_table::{self, CannotExtendTable},
    dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe,
    extern_type::ExternType,
    generic_error::GenericError,
    intersection_type::IntersectionType,
    never_type::NeverType,
    not_a_table::NotATable,
    property_type::Property,
    symbol::Symbol,
    table_indexer::TableIndexer,
    table_type::TableType,
    type_checker::TypeChecker,
    type_error::TypeError,
    union_type::UnionType,
    unknown_property::UnknownProperty,
    unknown_symbol::{self, UnknownSymbol},
  },
  type_aliases::{
    error_type::ErrorType, name_type::Name, scope_ptr_type::ScopePtr,
    type_error_data::TypeErrorData, type_id::TypeId,
  },
};

impl TypeChecker {
  pub fn check_l_value_binding_scope_ptr_ast_expr_local(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprLocal,
  ) -> TypeId {
    if let Some(ty) = scope.lookup_symbol(Symbol::from_local(expr.local.as_ptr())) {
      let ty = follow_type::follow(ty);
      if get_type::get::<NeverType>(ty).is_some() {
        return self.unknown_type;
      }
      return ty;
    }

    // expr.local 已句柄化恒非空：.get() 只读借用，name.value 为 NUL 结尾 C 字符串。
    let name_str = expr.local.get().name.as_str_or_empty();
    let error_data = TypeErrorData::UnknownSymbol(UnknownSymbol::new(
      name_str.to_string(),
      unknown_symbol::Context::Binding,
    ));
    let error = TypeError::type_error_location_type_error_data(expr.base.base.location, error_data);
    self.report_error_type_error(&error);

    self.error_recovery_type_scope_ptr(scope)
  }

  pub fn check_l_value_binding_scope_ptr_ast_expr_global(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprGlobal,
  ) -> TypeId {
    let name: Name = expr.name.as_str_or_empty().to_string();
    let module_scope = self.current_module.as_ref().expect("current_module 由 check_without_recursion_check 入口置入 Some、末尾才 take()，check 调用树内恒为 Some").get_module_scope();

    let sym = Symbol::from_global(expr.name);

    if let Some(binding) = module_scope.bindings.get(&sym) {
      let ty = follow_type::follow(binding.type_id);
      if get_type::get::<NeverType>(ty).is_some() {
        return self.unknown_type;
      }
      return ty;
    }

    let result = self.fresh_type_scope_ptr(scope.clone());

    {
      let binding = Binding {
        type_id: result,
        location: expr.base.base.location,
        deprecated: false,
        deprecated_suggestion: String::new(),
        documentation_symbol: None,
      };
      // SAFETY: module_scope 是 module->scopes 共享的 Scope（C++ 经 shared_ptr
      // 可变访问同义）；类型检查阶段单线程独占，无并发别名。
      let module_scope_ptr = unsafe { &mut *(arc_as_mut(&module_scope)) };
      module_scope_ptr.bindings.insert(sym, binding);
    }

    // If we're in strict mode, we want to report defining a global as an error,
    // but still add it to the bindings, so that autocomplete includes it in completions.
    if !self.is_nonstrict_mode() {
      let error_data =
        TypeErrorData::UnknownSymbol(UnknownSymbol::new(name, unknown_symbol::Context::Binding));
      let error =
        TypeError::type_error_location_type_error_data(expr.base.base.location, error_data);
      self.report_error_type_error(&error);
    }

    result
  }

  pub fn check_l_value_binding_scope_ptr_ast_expr_index_name_value_context(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprIndexName,
    ctx: ValueContext,
  ) -> TypeId {
    let mut lhs = self
      .check_expr(
        scope,
        // expr.expr 已句柄化恒非空：.get() 安全借用。
        expr.expr.get(),
        None,
        false,
      )
      .r#type;

    if get_type::get::<ErrorType>(lhs).is_some() || get_type::get::<AnyType>(lhs).is_some() {
      return lhs;
    }

    if get_type::get::<NeverType>(lhs).is_some() {
      return self.unknown_type;
    }

    self.tablify(lhs);

    // SAFETY: index.value 为 NUL 结尾 C 字符串（AST arena 持有）。
    let name: Name = expr.index.as_str_or_empty().to_string();

    // expr.expr 已句柄化恒非空；仅读其 base.location。
    lhs = self.strip_from_nil_and_report(lhs, &expr.expr.get().base.location);

    if let Some(lhs_table_ref) = get_mutable_table_type(lhs) {
      if let Some(prop) = lhs_table_ref.props.get(&name) {
        return prop.type_deprecated();
      } else if (ctx == ValueContext::LValue && lhs_table_ref.state == TableState::Unsealed)
        || lhs_table_ref.state == TableState::Free
      {
        let the_type = self.fresh_type_scope_ptr(scope.clone());
        let property = lhs_table_ref
          .props
          .entry(name)
          .or_insert_with(Property::default);
        property.set_type(the_type);
        property.location = Some(expr.index_location);
        return the_type;
      } else if let Some(indexer) = lhs_table_ref.indexer {
        let ok = self.unify_type_id_type_id_scope_ptr_location(
          self.string_type,
          indexer.index_type,
          scope,
          &expr.base.base.location,
        );
        let mut ret_type = indexer.index_result_type;
        if !ok {
          self.report_error_location_type_error_data(
            &expr.base.base.location,
            TypeErrorData::UnknownProperty(UnknownProperty {
              table: lhs,
              key: name,
            }),
          );
          ret_type = self.error_recovery_type_type_id(ret_type);
        }
        return ret_type;
      } else if lhs_table_ref.state == TableState::Sealed {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          expr.base.base.location,
          TypeErrorData::CannotExtendTable(CannotExtendTable {
            table_type: lhs,
            context: cannot_extend_table::Context::Property,
            prop: name,
          }),
        ));
        return self.error_recovery_type_scope_ptr(scope);
      } else {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          expr.base.base.location,
          TypeErrorData::GenericError(GenericError::new(String::from(
            "Internal error: generic tables are not lvalues",
          ))),
        ));
        return self.error_recovery_type_scope_ptr(scope);
      }
    } else if let Some(lhs_extern_type) = get_type::get::<ExternType>(lhs) {
      if let Some(prop) = lookup_extern_type_prop(lhs_extern_type, &name) {
        if ctx == ValueContext::LValue
          && let Some(write_ty) = prop.write_ty
        {
          return write_ty;
        }

        return prop.type_deprecated();
      }

      if let Some(indexer) = lhs_extern_type.indexer {
        let ok = self.unify_type_id_type_id_scope_ptr_location(
          self.string_type,
          indexer.index_type,
          scope,
          &expr.base.base.location,
        );
        if ok {
          return indexer.index_result_type;
        }
      }

      self.report_error_type_error(&TypeError::type_error_location_type_error_data(
        expr.base.base.location,
        TypeErrorData::UnknownProperty(UnknownProperty {
          table: lhs,
          key: name,
        }),
      ));
      return self.error_recovery_type_scope_ptr(scope);
    } else if get_type::get::<IntersectionType>(lhs).is_some() {
      if let Some(ty) =
        self.get_index_type_from_type(scope.clone(), lhs, &name, &expr.base.base.location, false)
      {
        return ty;
      }

      // If intersection has a table part, report that it cannot be extended just as a sealed table
      if is_table_intersection(lhs) {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          expr.base.base.location,
          TypeErrorData::CannotExtendTable(CannotExtendTable {
            table_type: lhs,
            context: cannot_extend_table::Context::Property,
            prop: name,
          }),
        ));
        return self.error_recovery_type_scope_ptr(scope);
      }
    }

    self.report_error_type_error(&TypeError::type_error_location_type_error_data(
      expr.base.base.location,
      TypeErrorData::NotATable(NotATable { ty: lhs }),
    ));
    self.error_recovery_type_scope_ptr(scope)
  }

  pub fn check_l_value_binding_scope_ptr_ast_expr_index_expr_value_context(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprIndexExpr,
    ctx: ValueContext,
  ) -> TypeId {
    // expr.expr / expr.index 已句柄化恒非空：.get() 安全借用。
    let (expr_ref, index_ref) = (expr.expr.get(), expr.index.get());
    let index_loc = index_ref.base.location;

    let expr_type = self.check_expr(scope, expr_ref, None, false).r#type;
    self.tablify(expr_type);
    let expr_type = self.strip_from_nil_and_report(expr_type, &expr_ref.base.location);
    let index_type = self.check_expr(scope, index_ref, None, false).r#type;
    let expr_type = follow_type::follow(expr_type);

    if get_type::get::<AnyType>(expr_type).is_some()
      || get_type::get::<ErrorType>(expr_type).is_some()
    {
      return expr_type;
    }

    if get_type::get::<NeverType>(expr_type).is_some() {
      return self.unknown_type;
    }

    let value = ast_node_try_as::<AstExprConstantString>(&index_ref.base);
    let value_name: Option<Name> =
      value.map(|v| String::from_utf8_lossy(v.value.as_bytes()).into_owned());

    if let Some(ref value_name) = value_name {
      if let Some(expr_extern_type) = get_type::get::<ExternType>(expr_type) {
        if let Some(prop) = lookup_extern_type_prop(expr_extern_type, value_name) {
          if ctx == ValueContext::LValue
            && let Some(write_ty) = prop.write_ty
          {
            return write_ty;
          }

          return prop.type_deprecated();
        }

        if let Some(ref indexer) = expr_extern_type.indexer {
          self.unify_type_id_type_id_scope_ptr_location(
            self.string_type,
            indexer.index_type,
            scope,
            &index_loc,
          );
          return indexer.index_result_type;
        }

        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          expr.base.base.location,
          TypeErrorData::UnknownProperty(UnknownProperty {
            table: expr_type,
            key: value_name.clone(),
          }),
        ));
        return self.error_recovery_type_scope_ptr(scope);
      } else if get_type::get::<IntersectionType>(expr_type).is_some() {
        let name = value_name;

        if let Some(ty) = self.get_index_type_from_type(
          scope.clone(),
          expr_type,
          name,
          &expr.base.base.location,
          false,
        ) {
          return ty;
        }

        if is_table_intersection(expr_type) {
          self.report_error_type_error(&TypeError::type_error_location_type_error_data(
            expr.base.base.location,
            TypeErrorData::CannotExtendTable(CannotExtendTable {
              table_type: expr_type,
              context: cannot_extend_table::Context::Property,
              prop: name.clone(),
            }),
          ));
          return self.error_recovery_type_scope_ptr(scope);
        }
      }
    } else {
      if let Some(expr_extern_type) = get_type::get::<ExternType>(expr_type)
        && let Some(ref indexer) = expr_extern_type.indexer
      {
        self.unify_type_id_type_id_scope_ptr_location(
          index_type,
          indexer.index_type,
          scope,
          &index_loc,
        );
        return indexer.index_result_type;
      }

      if get_type::get::<ExternType>(expr_type).is_some() {
        if self.is_nonstrict_mode() {
          return self.unknown_type;
        }
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          expr.base.base.location,
          TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(
            DynamicPropertyLookupOnExternTypesUnsafe { ty: expr_type },
          ),
        ));
        return self.error_recovery_type_scope_ptr(scope);
      }
    }

    let mut table_types: Vec<&'static mut TableType> = Vec::new();
    let mut is_union = true;

    if let Some(expr_union) = get_type::get::<UnionType>(expr_type) {
      table_types.reserve(expr_union.options.len());

      // C++ `for (auto option : exprUnion)`——UnionTypeIterator 展平嵌套
      // union 并 follow，裸遍历 options 会漏掉嵌套成员。
      for option in begin_union_type(expr_union) {
        let Some(option_table) = get_mutable_table_type(option) else {
          self.report_error_type_error(&TypeError::type_error_location_type_error_data(
            expr_ref.base.location,
            TypeErrorData::NotATable(NotATable { ty: expr_type }),
          ));
          return self.error_recovery_type_scope_ptr(scope);
        };
        table_types.push(option_table);
      }
    } else if let Some(expr_intersection) = get_type::get::<IntersectionType>(expr_type) {
      table_types.reserve(expr_intersection.parts.len());
      is_union = false;

      // C++ `for (auto part : exprIntersection)`——IntersectionTypeIterator
      // 同理展平。
      for part in begin_intersection_type(expr_intersection) {
        let Some(part_table) = get_mutable_table_type(part) else {
          self.report_error_type_error(&TypeError::type_error_location_type_error_data(
            expr_ref.base.location,
            TypeErrorData::NotATable(NotATable { ty: expr_type }),
          ));
          return self.error_recovery_type_scope_ptr(scope);
        };
        table_types.push(part_table);
      }
    } else {
      let Some(expr_table) = get_mutable_table_type(expr_type) else {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          expr_ref.base.location,
          TypeErrorData::NotATable(NotATable { ty: expr_type }),
        ));
        return self.error_recovery_type_scope_ptr(scope);
      };
      table_types.push(expr_table);
    }

    if let Some(ref key_name) = value_name {
      let mut property_types: DenseHashSet<TypeId> = DenseHashSet::default();

      for table_ref in &mut table_types {
        let table_ref: &mut TableType = table_ref;
        if let Some(prop) = table_ref.props.get(key_name) {
          property_types.insert(prop.type_deprecated());
        } else if (ctx == ValueContext::LValue && table_ref.state == TableState::Unsealed)
          || table_ref.state == TableState::Free
        {
          let result_type = self.fresh_type_scope_ptr(scope.clone());
          let property = table_ref.props.entry(key_name.clone()).or_default();
          property.set_type(result_type);
          property.location = Some(index_loc);
          property_types.insert(result_type);
        }
      }

      if property_types.size() == 1
        && let Some(&ty) = property_types.iter().next()
      {
        return ty;
      }

      if !property_types.empty() {
        if is_union {
          let options_vec: Vec<TypeId> = property_types.iter().copied().collect();
          let options = reduce_union(&options_vec);
          if options.is_empty() {
            return self.never_type;
          }
          if options.len() == 1 {
            return options[0];
          }
          return self.add_type(&UnionType { options });
        }

        let parts_vec: Vec<TypeId> = property_types.iter().copied().collect();
        return self.add_type(&IntersectionType { parts: parts_vec });
      }
    }

    let mut result_types: DenseHashSet<TypeId> = DenseHashSet::default();

    for table_ref in &mut table_types {
      let table_ref: &mut TableType = table_ref;
      if let Some(ref indexer) = table_ref.indexer {
        self.unify_type_id_type_id_scope_ptr_location(
          index_type,
          indexer.index_type,
          scope,
          &index_loc,
        );
        result_types.insert(indexer.index_result_type);
      } else if (ctx == ValueContext::LValue && table_ref.state == TableState::Unsealed)
        || table_ref.state == TableState::Free
      {
        let level = table_ref.level;
        let indexer_type = self.fresh_type_type_level(level);
        self.unify_type_id_type_id_scope_ptr_location(
          index_type,
          indexer_type,
          scope,
          &expr.base.base.location,
        );
        let index_result_type = self.fresh_type_type_level(level);

        let index_type_any = self.any_if_nonstrict(indexer_type);
        let index_result_type_any = self.any_if_nonstrict(index_result_type);
        table_ref.indexer = Some(TableIndexer {
          index_type: index_type_any,
          index_result_type: index_result_type_any,
          is_read_only: false,
        });
        result_types.insert(index_result_type);
      } else {
        if is_union {
          return self.any_type;
        }
        result_types.insert(self.any_type);
      }
    }

    if result_types.size() == 1
      && let Some(&ty) = result_types.iter().next()
    {
      return ty;
    }

    if is_union {
      let options_vec: Vec<TypeId> = result_types.iter().copied().collect();
      let options = reduce_union(&options_vec);
      if options.is_empty() {
        return self.never_type;
      }
      if options.len() == 1 {
        return options[0];
      }
      return self.add_type(&UnionType { options });
    }

    let parts_vec: Vec<TypeId> = result_types.iter().copied().collect();
    self.add_type(&IntersectionType { parts: parts_vec })
  }
}
