use alloc::{
  string::{String, ToString},
  sync::Arc,
  vec::Vec,
};
use core::{mem::take, ptr::null_mut};

use ulua_ast::{
  enums::ast_table_access::AstTableAccess,
  records::{
    ast_attr::AstAttrType, ast_type::AstType, ast_type_error::AstTypeError,
    ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
    ast_type_intersection::AstTypeIntersection, ast_type_optional::AstTypeOptional,
    ast_type_reference::AstTypeReference, ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion,
  },
  rtti::ast_node_try_as,
};

use crate::{
  enums::table_state::TableState,
  functions::{finite::finite, first::first, reduce_union::reduce_union, size_type_pack::size},
  records::{
    function_argument::FunctionArgument,
    function_type::FunctionType,
    generic_error::GenericError,
    incorrect_generic_parameter_count::IncorrectGenericParameterCount,
    intersection_type::IntersectionType,
    property_type::Property,
    scope::Scope,
    swapped_generic_type_parameter::SwappedGenericTypeParameter,
    table_indexer::TableIndexer,
    table_type::TableType,
    type_checker::TypeChecker,
    type_pack,
    type_pack::TypePack,
    union_type::UnionType,
    unknown_symbol::{Context, UnknownSymbol},
  },
  type_aliases::{
    name_type::Name, props_type::Props, scope_ptr_type::ScopePtr, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
impl TypeChecker {
  pub fn resolve_type_worker(&mut self, scope: ScopePtr, annotation: &AstType) -> TypeId {
    // AstType 以 base 字段内嵌 AstNode（repr(C) 单继承），下转走安全 ast_node_try_as
    let node = &annotation.base;

    if let Some(group) = ast_node_try_as::<AstTypeGroup>(node) {
      return self.resolve_type(scope, unsafe { &*group.type_ });
    }

    if ast_node_try_as::<AstTypeError>(node).is_some() {
      return self.error_recovery_type_scope_ptr(&scope);
    }

    if let Some(reference) = ast_node_try_as::<AstTypeReference>(node) {
      let name: Name = reference.name.as_str_or_empty().to_string();

      let alias = if let Some(prefix) = reference.prefix {
        let prefix: Name = prefix.as_str_or_empty().to_string();
        scope.lookup_imported_type(&prefix, &name)
      } else {
        scope.lookup_type(&name)
      };

      if let Some(tf) = alias {
        if reference.parameters.is_empty()
          && tf.type_params().is_empty()
          && tf.type_pack_params().is_empty()
        {
          return tf.r#type();
        }

        let mut parameter_count_error_reported = false;
        let has_default_types = tf
          .type_params()
          .iter()
          .any(|param| param.default_value.is_some());
        let has_default_packs = tf
          .type_pack_params()
          .iter()
          .any(|param| param.default_value.is_some());

        if !reference.has_parameter_list
          && ((!tf.type_params().is_empty() && !has_default_types)
            || (!tf.type_pack_params().is_empty() && !has_default_packs))
        {
          self.report_error_location_type_error_data(
            &annotation.base.location,
            GenericError::new(String::from("Type parameter list is required")).into(),
          );
          parameter_count_error_reported = true;
        }

        let mut type_params: Vec<TypeId> = Vec::new();
        let mut extra_types: Vec<TypeId> = Vec::new();
        let mut type_pack_params: Vec<TypePackId> = Vec::new();

        for param in reference.parameters.iter() {
          if !param.r#type.is_null() {
            let ty = self.resolve_type(scope.clone(), unsafe { &*param.r#type });

            if type_params.len() < tf.type_params().len() || tf.type_pack_params().is_empty() {
              type_params.push(ty);
            } else if type_pack_params.is_empty() {
              extra_types.push(ty);
            } else {
              self.report_error_location_type_error_data(
                &annotation.base.location,
                GenericError::new(String::from(
                  "Type parameters must come before type pack parameters",
                ))
                .into(),
              );
            }
          } else if !param.type_pack.is_null() {
            let tp = self.resolve_type_pack_scope_ptr_ast_type_pack(scope.clone(), unsafe {
              &*param.type_pack
            });

            if type_pack_params.is_empty() && !extra_types.is_empty() {
              type_pack_params.push(self.add_type_pack_type_pack(TypePack {
                head: take(&mut extra_types),
                tail: None,
              }));
            }

            if type_params.len() < tf.type_params().len()
              && unsafe { size(tp, null_mut()) } == 1
              && unsafe { finite(tp, null_mut()) }
            {
              if let Some(first_ty) = first(tp, true) {
                type_params.push(first_ty);
              } else {
                type_pack_params.push(tp);
              }
            } else {
              type_pack_params.push(tp);
            }
          }
        }

        if type_pack_params.is_empty() && !extra_types.is_empty() {
          type_pack_params.push(self.add_type_pack_type_pack(TypePack {
            head: extra_types,
            tail: None,
          }));
        }

        let types_required = tf.type_params().len();
        let packs_required = tf.type_pack_params().len();
        let not_enough_parameters = (type_params.len() < types_required
          && type_pack_params.is_empty())
          || (type_params.len() == types_required && type_pack_params.len() < packs_required);

        if not_enough_parameters && (has_default_types || has_default_packs) {
          // 缺省参数补齐：遇 None 即停（map_while），与逐索引循环等价
          type_params.extend(
            tf.type_params()[type_params.len()..types_required]
              .iter()
              .map_while(|param| param.default_value),
          );

          type_pack_params.extend(
            tf.type_pack_params()[type_pack_params.len()..packs_required]
              .iter()
              .map_while(|param| param.default_value),
          );
        }

        if reference.parameters.is_empty() && type_pack_params.len() + 1 == packs_required {
          type_pack_params.push(self.add_type_pack_type_pack(TypePack {
            head: Vec::new(),
            tail: None,
          }));
        }

        if type_params.len() != types_required || type_pack_params.len() != packs_required {
          if !parameter_count_error_reported {
            self.report_error_location_type_error_data(
              &annotation.base.location,
              IncorrectGenericParameterCount {
                name: name.clone(),
                type_fun: tf.clone(),
                actual_parameters: type_params.len(),
                actual_pack_parameters: type_pack_params.len(),
              }
              .into(),
            );
          }

          while type_params.len() < types_required {
            type_params.push(self.error_recovery_type_scope_ptr(&scope));
          }

          while type_pack_params.len() < packs_required {
            type_pack_params.push(self.error_recovery_type_pack_scope_ptr(scope.clone()));
          }
        }

        let same_tys = type_params
          .iter()
          .zip(tf.type_params().iter())
          .all(|(arg, param)| *arg == param.ty);
        let same_tps = type_pack_params
          .iter()
          .zip(tf.type_pack_params().iter())
          .all(|(arg, param)| *arg == param.tp);

        if same_tys
          && same_tps
          && type_params.len() == tf.type_params().len()
          && type_pack_params.len() == tf.type_pack_params().len()
        {
          return tf.r#type();
        }

        return self.instantiate_type_fun(
          &scope,
          &tf,
          &type_params,
          &type_pack_params,
          &annotation.base.location,
        );
      }

      let mut type_name = String::new();
      if let Some(prefix) = reference.prefix {
        let prefix: Name = prefix.as_str_or_empty().to_string();
        type_name.push_str(&prefix);
        type_name.push('.');
      }
      type_name.push_str(&name);

      if scope.lookup_pack(&type_name).is_some() {
        self.report_error_location_type_error_data(
          &annotation.base.location,
          SwappedGenericTypeParameter {
            name: type_name,
            kind: SwappedGenericTypeParameter::TYPE,
          }
          .into(),
        );
      } else {
        self.report_error_location_type_error_data(
          &annotation.base.location,
          UnknownSymbol::new(type_name, Context::Type).into(),
        );
      }

      return self.error_recovery_type_scope_ptr(&scope);
    }

    if ast_node_try_as::<AstTypeOptional>(node).is_some() {
      return self.nil_type;
    }

    if let Some(table) = ast_node_try_as::<AstTypeTable>(node) {
      let mut props = Props::default();

      for prop in table.props.iter() {
        match prop.access {
          AstTableAccess::Read => {
            self.report_error_location_type_error_data(
              &prop.access_location.unwrap_or_default(),
              GenericError::new(String::from("read keyword is illegal here")).into(),
            );
          }
          AstTableAccess::Write => {
            self.report_error_location_type_error_data(
              &prop.access_location.unwrap_or_default(),
              GenericError::new(String::from("write keyword is illegal here")).into(),
            );
          }
          AstTableAccess::ReadWrite => {
            let name: Name = prop.name.as_str_or_empty().to_string();
            let ty = if prop.r#type.is_null() {
              self.error_recovery_type_scope_ptr(&scope)
            } else {
              self.resolve_type(scope.clone(), unsafe { &*prop.r#type })
            };
            let mut property = Property {
              type_location: Some(prop.location),
              ..Property::default()
            };
            property.read_ty = Some(ty);
            property.write_ty = Some(ty);
            props.insert(name, property);
          }
        }
      }

      let indexer = if table.indexer.is_null() {
        None
      } else {
        let indexer = unsafe { &*table.indexer };
        match indexer.access {
          AstTableAccess::Read => {
            self.report_error_location_type_error_data(
              &indexer.access_location.unwrap_or_default(),
              GenericError::new(String::from("read keyword is illegal here")).into(),
            );
            None
          }
          AstTableAccess::Write => {
            self.report_error_location_type_error_data(
              &indexer.access_location.unwrap_or_default(),
              GenericError::new(String::from("write keyword is illegal here")).into(),
            );
            None
          }
          AstTableAccess::ReadWrite => Some(TableIndexer {
            index_type: if indexer.index_type.is_null() {
              self.error_recovery_type_scope_ptr(&scope)
            } else {
              self.resolve_type(scope.clone(), unsafe { &*indexer.index_type })
            },
            index_result_type: if indexer.result_type.is_null() {
              self.error_recovery_type_scope_ptr(&scope)
            } else {
              self.resolve_type(scope.clone(), unsafe { &*indexer.result_type })
            },
            is_read_only: false,
          }),
        }
      };

      let table_ty =
        TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
          &props,
          indexer,
          scope.level,
          scope.as_ref() as *const _ as *mut _,
          TableState::Sealed,
        );
      return self.add_type(&table_ty);
    }

    if let Some(func) = ast_node_try_as::<AstTypeFunction>(node) {
      let func_scope = self.child_scope(&scope, &func.base.base.location);
      unsafe {
        let func_scope_raw = Arc::as_ptr(&func_scope) as *mut Scope;
        (*func_scope_raw).level = scope.level.incr();
      }

      let defs = self.create_generic_types(
        &func_scope,
        None,
        &annotation.base,
        &func.generics,
        &func.generic_packs,
        false,
      );

      let arg_types =
        self.resolve_type_pack_scope_ptr_ast_type_list(func_scope.clone(), &func.arg_types);
      let ret_types = if func.return_types.is_null() {
        self.add_type_pack_type_pack(type_pack::TypePack {
          head: Vec::new(),
          tail: None,
        })
      } else {
        self.resolve_type_pack_scope_ptr_ast_type_pack(func_scope.clone(), unsafe {
          &*func.return_types
        })
      };

      let mut ftv = FunctionType::function_type_new(arg_types, ret_types, None, false);
      ftv.level = func_scope.level;
      ftv.generics = defs.generic_types.iter().map(|def| def.ty).collect();
      ftv.generic_packs = defs.generic_packs.iter().map(|def| def.tp).collect();

      for arg_name in func.arg_names.iter() {
        ftv
          .arg_names
          .push(arg_name.map(|(name, location)| FunctionArgument {
            name: name.as_str_or_empty().to_string(),
            location,
          }));
      }

      ftv.is_checked_function = func.is_checked_function();
      let deprecated_attr = func.get_attribute(AstAttrType::Deprecated);
      ftv.is_deprecated_function = !deprecated_attr.is_null();
      if !deprecated_attr.is_null() {
        ftv.deprecated_info = Some(Arc::new(unsafe { (*deprecated_attr).deprecated_info() }));
      }

      return self.add_type(&ftv);
    }

    if let Some(type_of) = ast_node_try_as::<AstTypeTypeof>(node) {
      return self
        .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
          &scope,
          unsafe { &*type_of.expr },
          None,
          false,
        )
        .r#type;
    }

    if let Some(union) = ast_node_try_as::<AstTypeUnion>(node) {
      let mut parts = Vec::new();

      for part in union.types.iter() {
        if !part.is_null() {
          parts.push(self.resolve_type(scope.clone(), unsafe { &**part }));
        }
      }

      let reduced = reduce_union(&parts);
      return match reduced.len() {
        0 => self.never_type,
        1 => reduced[0],
        _ => self.add_type(&UnionType { options: reduced }),
      };
    }

    if let Some(intersection) = ast_node_try_as::<AstTypeIntersection>(node) {
      let mut parts = Vec::new();

      for part in intersection.types.iter() {
        if !part.is_null() {
          parts.push(self.resolve_type(scope.clone(), unsafe { &**part }));
        }
      }

      return match parts.len() {
        0 => self.never_type,
        1 => parts[0],
        _ => self.add_type(&IntersectionType { parts }),
      };
    }

    if let Some(singleton_bool) = ast_node_try_as::<AstTypeSingletonBool>(node) {
      return self.singleton_type_bool(singleton_bool.value);
    }

    if let Some(singleton_string) = ast_node_try_as::<AstTypeSingletonString>(node) {
      let bytes: Vec<u8> = singleton_string.value.iter().map(|c| *c as u8).collect();
      let value = String::from_utf8_lossy(&bytes).into_owned();
      return self.singleton_type_string(value);
    }

    self.error_recovery_type_scope_ptr(&scope)
  }
}
